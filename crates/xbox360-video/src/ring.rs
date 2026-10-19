//! The command processor ring of a title that owns the GPU engines.

use xbox360_sys::mmio::GpuRegisters;
use xbox360_sys::processor;
use xbox360_sys::xboxkrnl::VdEnableRingBufferRPtrWriteBack;
use xbox360_sys::xboxkrnl::VdInitializeRingBuffer;
use xbox360_sys::xboxkrnl::VdSetSystemCommandBufferGpuIdentifierAddress;

use super::memory::Caching;
use super::memory::PhysicalBuffer;

/// Ring write pointer register, in dwords from the ring base.
const RING_WRITE_POINTER: u16 = 0x1c5;

/// Ring size of D3D's default.
const RING_BYTES: u32 = 0x8000;
const RING_DWORDS: u32 = RING_BYTES / 4;

/// The submission granule: D3D pads every submission to 32 bytes.
const GRANULE_DWORDS: u32 = 8;

/// The writeback page.
const WRITEBACK_BYTES: u32 = 0x1000;
/// Engine initialization packet values, as D3D writes them.
const ENGINE_INIT_MASK: u32 = 0x3ff;
const ENGINE_INIT_STEP: u32 = 0x80;
const ENGINE_INIT_WORD12: u32 = 0x0001_0800;
const ENGINE_INIT_WORD13: u32 = 7;
/// Wait packet function: compare memory for equality.
const WAIT_MEMORY_EQUAL: u32 = 0x13;
/// Wait packet poll interval as D3D writes it.
const WAIT_INTERVAL: u32 = 0x100;
/// Event write type: shader work done.
const EVENT_SHADER_DONE: u32 = 3;
/// Address low bits of event writes and memory waits: 8-in-32 swap.
const ENDIAN_8_IN_32: u32 = 2;

/// Offsets in the writeback page, as D3D lays out its block.
const SWAP_IDENTIFIER_OFFSET: u32 = 0x08;
const READ_POINTER_OFFSET: u32 = 0x3c;
/// Dword that [`Ring::wait_for_pipeline`] has the GPU write.
const PIPELINE_FENCE_OFFSET: u32 = 0x40;

/// Type-0 packet header: write `count` consecutive registers from `register`.
pub const fn type0(register: u16, count: u32) -> u32 {
    ((count - 1) << 16) | register as u32
}

/// Packet type 3 in bits 30-31.
const PACKET_TYPE_3: u32 = 0xc000_0000;

/// Type-3 packet header: `opcode` with `count` payload dwords.
pub const fn type3(opcode: u8, count: u32) -> u32 {
    PACKET_TYPE_3 | ((count - 1) << 16) | (opcode as u32) << 8
}

pub const OPCODE_NOP: u8 = 0x10;
const OPCODE_WAIT: u8 = 0x3c;
const OPCODE_EVENT_WRITE: u8 = 0x58;
pub const OPCODE_REGISTER_READ_MODIFY_WRITE: u8 = 0x21;
const OPCODE_ENGINE_INIT: u8 = 0x48;

/// Value a swap's event write stores; it increases by 2 per swap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwapIdentifier(pub u32);

impl SwapIdentifier {
    /// True once `completed` is this identifier or a later one.
    pub fn reached_by(self, completed: SwapIdentifier) -> bool {
        completed.0.wrapping_sub(self.0) as i32 >= 0
    }
}

pub struct Ring {
    commands: PhysicalBuffer,
    writeback: PhysicalBuffer,
    /// Next dword to write, in dwords from the ring base.
    write: u32,
    /// Last value [`Ring::wait_for_pipeline`] had the GPU write.
    fence: u32,
}

impl Ring {
    /// Allocate the ring, attach it to the command processor and run the
    /// engine initialization packet. The engines must be owned
    /// (`VdInitializeEngines`).
    pub fn new() -> Option<Self> {
        let commands = PhysicalBuffer::new(RING_BYTES, Caching::WriteCombined)?;
        let mut writeback = PhysicalBuffer::new(WRITEBACK_BYTES, Caching::Uncached)?;
        writeback.words().fill(0);
        // VdInitializeRingBuffer takes log2(dwords) - 1 and the writeback
        // block log2 of (bytes / 512), as D3D derives them.
        // SAFETY: both buffers stay allocated until `Drop` detaches them.
        unsafe {
            VdInitializeRingBuffer(commands.physical().0, RING_DWORDS.ilog2() - 1);
            VdEnableRingBufferRPtrWriteBack(writeback.physical().0 + READ_POINTER_OFFSET, (RING_BYTES >> 9).ilog2());
            VdSetSystemCommandBufferGpuIdentifierAddress(writeback.virtual_address(SWAP_IDENTIFIER_OFFSET));
        }
        let mut ring = Self { commands, writeback, write: 0, fence: 0 };
        let init = ring.reserve(19);
        init[0] = type3(OPCODE_ENGINE_INIT, 18);
        init[1] = ENGINE_INIT_MASK;
        init[2] = 0;
        init[3] = 0;
        for (i, word) in init[4..12].iter_mut().enumerate() {
            *word = i as u32 * ENGINE_INIT_STEP;
        }
        init[12] = ENGINE_INIT_WORD12;
        init[13] = ENGINE_INIT_WORD13;
        init[14..19].fill(0);
        ring.commit(19);
        ring.kick();
        Some(ring)
    }

    /// Virtual address of the dword where the GPU writes swap identifiers.
    pub fn identifier_address(&self) -> u32 {
        self.writeback.virtual_address(SWAP_IDENTIFIER_OFFSET)
    }

    /// The last swap identifier the GPU wrote.
    pub fn completed_identifier(&self) -> SwapIdentifier {
        SwapIdentifier(self.writeback.read_word(SWAP_IDENTIFIER_OFFSET))
    }

    fn read_pointer(&self) -> u32 {
        self.writeback.read_word(READ_POINTER_OFFSET)
    }

    /// `dwords` contiguous free dwords at the write position, after
    /// wrapping and waiting for the GPU as needed. Follow with [`Self::commit`].
    pub fn reserve(&mut self, dwords: u32) -> &mut [u32] {
        // The kick after this submission may pad up to GRANULE_DWORDS + 1.
        debug_assert!(dwords + GRANULE_DWORDS + 1 < RING_DWORDS);
        if self.write + dwords > RING_DWORDS {
            let tail = RING_DWORDS - self.write;
            self.wait_free(tail);
            let words = self.commands.words();
            words[self.write as usize] = type3(OPCODE_NOP, tail - 1);
            self.write = 0;
            self.kick();
        }
        self.wait_free(dwords + GRANULE_DWORDS + 1);
        let start = self.write as usize;
        &mut self.commands.words()[start..start + dwords as usize]
    }

    pub fn commit(&mut self, dwords: u32) {
        self.write += dwords;
    }

    /// Pad to the granule and move the write pointer.
    pub fn kick(&mut self) {
        let mut pad = self.write.next_multiple_of(GRANULE_DWORDS) - self.write;
        if pad == 1 {
            // A NOP is at least two dwords.
            pad += GRANULE_DWORDS;
        }
        if pad != 0 {
            let start = self.write as usize;
            self.commands.words()[start] = type3(OPCODE_NOP, pad - 1);
        }
        self.write = (self.write + pad) % RING_DWORDS;
        // The CPU stores to the write-combined ring must reach memory before
        // the GPU sees the new write pointer.
        processor::sync();
        // SAFETY: the dwords up to the write pointer are complete packets.
        unsafe { GpuRegisters::get().register(RING_WRITE_POINTER).write(self.write) };
    }

    /// Wait until the GPU has read every submitted dword.
    pub fn wait_idle(&self) {
        while self.read_pointer() != self.write {
            processor::low_priority_hint();
        }
    }

    fn wait_free(&self, dwords: u32) {
        // One dword stays unused so that equal pointers mean empty.
        while (self.read_pointer().wrapping_sub(self.write).wrapping_sub(1)) % RING_DWORDS < dwords {
            processor::low_priority_hint();
        }
    }

    /// Make the command processor wait until every earlier command has
    /// left the pipeline: an event write of a new value to the writeback
    /// page once the pipeline drains, and a wait for that value in memory.
    pub fn wait_for_pipeline(&mut self) {
        self.fence = self.fence.wrapping_add(1);
        let fence = self.fence;
        let address = self.writeback.physical().0 + PIPELINE_FENCE_OFFSET;
        let words = self.reserve(10);
        words[..4].copy_from_slice(&[type3(OPCODE_EVENT_WRITE, 3), EVENT_SHADER_DONE, address | ENDIAN_8_IN_32, fence]);
        words[4..].copy_from_slice(&[type3(OPCODE_WAIT, 5), WAIT_MEMORY_EQUAL, address | ENDIAN_8_IN_32, fence, u32::MAX, WAIT_INTERVAL]);
        self.commit(10);
    }

    /// True once the GPU has passed the last [`Self::wait_for_pipeline`].
    pub fn pipeline_drained(&self) -> bool {
        self.writeback.read_word(PIPELINE_FENCE_OFFSET) == self.fence
    }

    /// Wait for the GPU and stop it writing swap identifiers. Run before
    /// `VdShutdownEngines`, as D3D does.
    pub fn detach(&mut self) {
        self.wait_idle();
        // SAFETY: detaching the identifier address has no precondition.
        unsafe { VdSetSystemCommandBufferGpuIdentifierAddress(0) };
    }
}

