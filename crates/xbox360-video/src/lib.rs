//! Scanout of CPU-rendered frames for a title. [`Display`] owns the GPU
//! command processor the way Direct3D does (`VdInitializeEngines` with the
//! kernel's microcode, a ring of its own, `VdSwap`) and flips between two
//! tiled front buffers of the current display size at vertical blank.
//! Dropping it gives the engines back so the console returns to the shell.

#![no_std]
// Other targets build the tiling and its tests only.
#![cfg_attr(not(target_os = "xbox360"), allow(dead_code, unused_imports))]

#[cfg(target_os = "xbox360")]
mod fetch;
#[cfg(target_os = "xbox360")]
pub mod memory;
#[cfg(target_os = "xbox360")]
pub mod ring;
mod error;
mod tiling;

#[cfg(target_os = "xbox360")]
pub use self::console::BackBuffer;
#[cfg(target_os = "xbox360")]
pub use self::console::Display;
#[cfg(target_os = "xbox360")]
pub use self::console::ShaderEpoch;
#[cfg(target_os = "xbox360")]
pub use self::console::ShaderOwner;
pub use self::error::BufferKind;
pub use self::error::DisplayError;

/// Size of a frame in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameSize {
    pub width: u32,
    pub height: u32,
}

/// The part of the display surface that the scaler shows, in surface
/// pixels: columns `left..right`, rows `top..bottom`. With a 4:3 video mode
/// on a 1280x720 surface the kernel reports columns 160..1120.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayWindow {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl DisplayWindow {
    /// `None` unless the window starts on a tile and lies inside `surface`.
    fn tiled(self, surface: FrameSize) -> Option<TiledWindow> {
        let inside = self.left < self.right && self.top < self.bottom && self.right <= surface.width && self.bottom <= surface.height;
        let aligned = self.left % tiling::TILE_EDGE == 0 && self.top % tiling::TILE_EDGE == 0;
        (inside && aligned).then(|| TiledWindow {
            tile_x: self.left / tiling::TILE_EDGE,
            tile_y: self.top / tiling::TILE_EDGE,
            size: FrameSize { width: self.right - self.left, height: self.bottom - self.top },
        })
    }
}

/// A [`DisplayWindow`] that starts on a tile.
#[derive(Clone, Copy, Debug)]
struct TiledWindow {
    tile_x: u32,
    tile_y: u32,
    size: FrameSize,
}

/// Borrowed pixels: `size.height` rows of `stride` pixels, of which the
/// first `size.width` are visible. Each pixel is a `u32` with red in bits
/// 0-7, green in 8-15 and blue in 16-23; bits 24-31 (alpha) are ignored.
#[derive(Clone, Copy)]
pub struct FrameView<'a> {
    pixels: &'a [u32],
    size: FrameSize,
    stride: u32,
}

impl<'a> FrameView<'a> {
    /// `None` when `stride` is less than the width or `pixels` is too short.
    pub fn new(pixels: &'a [u32], size: FrameSize, stride: u32) -> Option<Self> {
        let needed = match size.height {
            0 => 0,
            rows => (rows as usize - 1).checked_mul(stride as usize)?.checked_add(size.width as usize)?,
        };
        (stride >= size.width && pixels.len() >= needed).then_some(Self { pixels, size, stride })
    }

    pub fn size(&self) -> FrameSize {
        self.size
    }
}

#[cfg(target_os = "xbox360")]
mod console {
    use core::sync::atomic::AtomicBool;
    use core::sync::atomic::AtomicU32;
    use core::sync::atomic::Ordering;

    use xbox360_sys::mmio::GpuRegisters;
    use xbox360_sys::processor;
    use xbox360_sys::xboxkrnl::DisplayInformation;
    use xbox360_sys::xboxkrnl::KeEnterCriticalRegion;
    use xbox360_sys::xboxkrnl::KeLeaveCriticalRegion;
    use xbox360_sys::xboxkrnl::SystemCommandBuffer;
    use xbox360_sys::xboxkrnl::VdGetCurrentDisplayInformation;
    use xbox360_sys::xboxkrnl::VdGetSystemCommandBuffer;
    use xbox360_sys::xboxkrnl::VdInitializeEngines;
    use xbox360_sys::xboxkrnl::VdRetrainEDRAM;
    use xbox360_sys::xboxkrnl::VdRetrainEDRAMWorker;
    use xbox360_sys::xboxkrnl::VdShutdownEngines;
    use xbox360_sys::xboxkrnl::VdSwap;

    use super::BufferKind;
    use super::DisplayError;
    use super::DisplayWindow;
    use super::TiledWindow;
    use super::FrameSize;
    use super::FrameView;
    use super::fetch;
    use super::memory::Caching;
    use super::memory::GpuAddress;
    use super::memory::PhysicalBuffer;
    use super::ring::OPCODE_REGISTER_READ_MODIFY_WRITE;
    use super::ring::Ring;
    use super::ring::SwapIdentifier;
    use super::ring::type0;
    use super::ring::type3;
    use super::tiling;
    use super::tiling::TILE_EDGE;

    /// Display controller registers that D3D's present writes.
    const SCANOUT_CONTROL: u16 = 0x1841;
    const SCANOUT_ADDRESS: u16 = 0x1844;
    const SCANOUT_UPDATE: u16 = 0x1851;
    /// Set from a write of `SCANOUT_ADDRESS` outside vertical blank until the
    /// controller latches it at the next blank.
    const UPDATE_PENDING: u32 = 1 << 0;
    /// Set from the latch for about 3 ms. A write while it is set takes
    /// effect at once.
    const UPDATE_TAKEN: u32 = 1 << 1;
    /// D3D writes 0 for 8:8:8:8 front buffers and 3 for 2:10:10:10.
    const SCANOUT_FORMAT_1930: u16 = 0x1930;
    /// `SCANOUT_CONTROL` bits 8-10: 0 selects 8:8:8:8, 1 2:10:10:10.
    const SCANOUT_CONTROL_KEEP: u32 = 0xffff_f8ff;

    /// Dwords that `VdSwap` writes.
    const SWAP_DWORDS: u32 = 64;
    /// Scanout register writes ahead of the swap.
    const FLIP_DWORDS: u32 = 2 + 4 + 2;

    /// Command capacity D3D gives each `VdRetrainEDRAM` step, in dwords.
    const RETRAIN_COMMAND_DWORDS: u32 = 0x1000;
    /// Work area D3D gives `VdRetrainEDRAM` (on its stack).
    const RETRAIN_WORK_BYTES: usize = 0x800;

    #[repr(C, align(16))]
    struct RetrainWork([u8; RETRAIN_WORK_BYTES]);

    /// When D3D retrains, as the `VdRetrainEDRAM` flags it passes then.
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    enum Retraining {
        AfterSwap = 0,
        AtInit = 1,
    }

    /// Set while a `Display` owns the GPU engines. `VdInitializeEngines`
    /// traps when they are already owned.
    static ENGINES_OWNED: AtomicBool = AtomicBool::new(false);

    /// Source of [`ShaderEpoch`] values.
    static NEXT_EPOCH: AtomicU32 = AtomicU32::new(0);

    /// Source of [`ShaderOwner`] values.
    static NEXT_OWNER: AtomicU32 = AtomicU32::new(0);

    /// Identifies a period in which the shader programs a renderer loaded
    /// stay loaded. A new one starts when the engines are taken, when a flip
    /// runs the system command buffer, whose commands are the system's, and
    /// when another renderer claims the shaders.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ShaderEpoch(u32);

    /// A renderer that loads shader programs through a [`Display`].
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ShaderOwner(u32);

    impl ShaderOwner {
        pub fn new() -> Self {
            Self(NEXT_OWNER.fetch_add(1, Ordering::Relaxed))
        }
    }

    impl Default for ShaderOwner {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ShaderEpoch {
        fn next() -> Self {
            Self(NEXT_EPOCH.fetch_add(1, Ordering::Relaxed))
        }
    }

    struct FrontBuffer {
        memory: PhysicalBuffer,
        fetch_constant: [u32; 6],
    }

    struct Surface {
        buffers: [FrontBuffer; 2],
        pitch_tiles: u32,
        window: TiledWindow,
    }

    impl Surface {
        fn current() -> Result<Self, DisplayError> {
            let mut information = DisplayInformation::new();
            // SAFETY: the block is 0x58 writable bytes.
            unsafe { VdGetCurrentDisplayInformation(information.as_mut_ptr()) };
            let size = FrameSize {
                width: u32::from(information.surface_width),
                height: u32::from(information.surface_height),
            };
            if size.width == 0 || size.height == 0 || size.width % TILE_EDGE != 0 {
                return Err(DisplayError::UnsupportedMode { size });
            }
            let window = DisplayWindow {
                left: information.left,
                top: information.top,
                right: information.right,
                bottom: information.bottom,
            };
            let window = window
                .tiled(size)
                .ok_or(DisplayError::UnsupportedWindow {
                    surface: size,
                    window,
                })?;
            let pitch_tiles = size.width / TILE_EDGE;
            let tiles_high = size.height.div_ceil(TILE_EDGE);
            let bytes = pitch_tiles * tiles_high * TILE_EDGE * TILE_EDGE * 4;
            let front_buffer = || {
                let mut memory = PhysicalBuffer::new(bytes, Caching::WriteCombined).ok_or(
                    DisplayError::OutOfMemory {
                        buffer: BufferKind::FrontBuffer,
                    },
                )?;
                memory.words().fill(0);
                let fetch_constant = fetch::front_buffer(
                    memory.virtual_address(0),
                    pitch_tiles,
                    size.width,
                    size.height,
                );
                Ok(FrontBuffer {
                    memory,
                    fetch_constant,
                })
            };

            Ok(Self {
                buffers: [front_buffer()?, front_buffer()?],
                pitch_tiles,
                window,
            })
        }
    }

    /// A submitted flip.
    #[derive(Clone, Copy)]
    struct Flip {
        identifier: SwapIdentifier,
        address: GpuAddress,
    }

    /// The buffer the next frame goes to, for a GPU renderer that resolves
    /// into it: tiled 32 bpp, B, G, R, X, `pitch` pixels per row.
    #[derive(Clone, Copy, Debug)]
    pub struct BackBuffer {
        pub address: GpuAddress,
        pub size: FrameSize,
        pub pitch: u32,
    }

    pub struct Display {
        ring: Ring,
        buffers: [FrontBuffer; 2],
        /// Index of the buffer the next frame is drawn into.
        back: usize,
        /// Tiles per surface row.
        pitch_tiles: u32,
        window: TiledWindow,
        pending: Option<Flip>,
        shader_epoch: ShaderEpoch,
        shader_owner: Option<ShaderOwner>,
    }

    impl Display {
        /// Take the GPU and set up two front buffers of the current display
        /// surface size. Frames have the size of the part of the surface
        /// that is displayed.
        pub fn new() -> Result<Self, DisplayError> {
            let Surface {
                buffers,
                pitch_tiles,
                window,
            } = Surface::current()?;

            if ENGINES_OWNED.swap(true, Ordering::AcqRel) {
                return Err(DisplayError::AlreadyOpen);
            }
            // Null microcode selects the kernel's own. No shutdown callback.
            // A title that ends without dropping the `Display` needs no
            // termination notification: the kernel's title cleanup shuts the
            // engines down when the title still owns them.
            // SAFETY: the engines are not owned (checked above).
            unsafe { VdInitializeEngines(0, 0, 0, core::ptr::null(), core::ptr::null()) };
            let Some(ring) = Ring::new() else {
                release_engines();
                return Err(DisplayError::OutOfMemory { buffer: BufferKind::Ring });
            };
            let mut display = Self { ring, buffers, back: 0, pitch_tiles, window, pending: None, shader_epoch: ShaderEpoch::next(), shader_owner: None };
            display.retrain_edram(Retraining::AtInit);
            Ok(display)
        }

        /// Rebuild the front buffers for the kernel's current display
        /// surface without releasing the GPU engines.
        pub fn reset(&mut self) -> Result<(), DisplayError> {
            let Surface {
                buffers,
                pitch_tiles,
                window,
            } = Surface::current()?;

            self.wait_for_flip();
            let shown = self.buffers[self.back ^ 1].fetch_constant;
            self.submit_flip(GpuAddress(0), &shown);
            self.wait_for_flip();

            self.buffers = buffers;
            self.back = 0;
            self.pitch_tiles = pitch_tiles;
            self.window = window;
            self.pending = None;
            self.shader_epoch = ShaderEpoch::next();
            self.shader_owner = None;

            Ok(())
        }

        /// D3D's EDRAM retraining step (`D3D_RetrainEDRAM` 0x92132310), run
        /// when the device is created and after every swap. The kernel
        /// reports retraining due only while its training count is 0 (it
        /// was 1 when this title started on a development kit), so the
        /// usual cost is one call that returns at once.
        fn retrain_edram(&mut self, when: Retraining) {
            let mut work = RetrainWork([0; RETRAIN_WORK_BYTES]);
            let flags = when as u32;
            // SAFETY: no preconditions; paired with the leave below.
            unsafe { KeEnterCriticalRegion() };
            if when == Retraining::AtInit {
                // SAFETY: the argument is unused by the kernel.
                unsafe { VdRetrainEDRAMWorker(0) };
            }
            let mut state = 0u32;
            let mut commands: *mut u32 = core::ptr::null_mut();
            // SAFETY: the first step writes only `state`.
            let due = unsafe { VdRetrainEDRAM(flags, &mut commands, RETRAIN_COMMAND_DWORDS, &mut state, work.0.as_mut_ptr(), RETRAIN_WORK_BYTES as u32) };
            if due != 0 {
                if when == Retraining::AfterSwap {
                    self.ring.block_until_idle();
                }
                loop {
                    let words = self.ring.reserve(RETRAIN_COMMAND_DWORDS);
                    let start = words.as_mut_ptr();
                    let mut cursor = start;
                    // SAFETY: `words` holds the capacity passed; the kernel
                    // advances `cursor` past what it wrote.
                    let more = unsafe { VdRetrainEDRAM(flags, &mut cursor, RETRAIN_COMMAND_DWORDS, &mut state, work.0.as_mut_ptr(), RETRAIN_WORK_BYTES as u32) };
                    let written = (cursor as usize - start as usize) / 4;
                    self.ring.commit(written as u32);
                    // The next step reads the results on the CPU.
                    self.ring.block_until_idle();
                    if more == 0 {
                        break;
                    }
                }
                // What the kernel's commands leave loaded is not known.
                self.shader_epoch = ShaderEpoch::next();
            }
            // SAFETY: entered above.
            unsafe { KeLeaveCriticalRegion() };
        }

        /// The size of the frames: the displayed part of the surface.
        pub fn size(&self) -> FrameSize {
            self.window.size
        }

        /// Byte offset of the window's first pixel in a front buffer.
        fn window_offset(&self) -> u32 {
            (self.window.tile_y * self.pitch_tiles + self.window.tile_x) * TILE_EDGE * TILE_EDGE * 4
        }

        /// The shader epoch for `owner`, which is about to load or use
        /// shader programs: programs it loaded under another one must be
        /// loaded again.
        pub fn claim_shaders(&mut self, owner: ShaderOwner) -> ShaderEpoch {
            if self.shader_owner != Some(owner) {
                self.shader_owner = Some(owner);
                self.shader_epoch = ShaderEpoch::next();
            }
            self.shader_epoch
        }

        /// Draw `frame` into the back buffer and flip to it at the next
        /// vertical blank. Waits for the previous flip to take effect first,
        /// so the buffer drawn into is never on screen. Parts of the window
        /// outside `frame` are black.
        pub fn present(&mut self, frame: FrameView<'_>) {
            self.wait_for_flip();
            let offset = self.window_offset() as usize / 4;
            let window = self.window;
            let back = &mut self.buffers[self.back];
            let source = tiling::SourceImage {
                pixels: frame.pixels,
                width: frame.size.width,
                height: frame.size.height,
                stride: frame.stride,
            };
            let target = tiling::TiledTarget {
                pitch_tiles: self.pitch_tiles,
                tiles_wide: window.size.width.div_ceil(TILE_EDGE),
                tiles_high: window.size.height.div_ceil(TILE_EDGE),
            };
            if let Some(words) = back.memory.words().get_mut(offset..) {
                tiling::tile_image(words, target, source);
            }
            let address = back.memory.physical();
            let fetch_constant = back.fetch_constant;
            self.submit_flip(address, &fetch_constant);
            self.back ^= 1;
        }

        /// Start a frame that the GPU renders: waits until the previous flip
        /// has taken effect, so the returned buffer is off screen. Commands
        /// that fill it go to `ring`; [`Self::present_gpu_frame`] flips to it.
        pub fn begin_gpu_frame(&mut self) -> (BackBuffer, &mut Ring) {
            self.wait_for_flip();
            let back = BackBuffer {
                address: GpuAddress(self.buffers[self.back].memory.physical().0 + self.window_offset()),
                size: self.window.size,
                pitch: self.pitch_tiles * TILE_EDGE,
            };
            (back, &mut self.ring)
        }

        /// The command ring, to observe GPU progress.
        pub fn ring(&self) -> &Ring {
            &self.ring
        }

        /// Flip to the buffer of [`Self::begin_gpu_frame`] once the commands
        /// already in the ring have finished writing it.
        pub fn present_gpu_frame(&mut self) {
            self.ring.wait_for_pipeline();
            let back = &self.buffers[self.back];
            let address = back.memory.physical();
            let fetch_constant = back.fetch_constant;
            self.submit_flip(address, &fetch_constant);
            self.back ^= 1;
        }

        /// Wait until the last submitted flip is latched by the display.
        /// The swap identifier can land before the display controller has
        /// taken the address write, so the address is also read back before
        /// the pending bit is trusted.
        fn wait_for_flip(&self) {
            let Some(flip) = self.pending else {
                return;
            };
            while !flip.identifier.reached_by(self.ring.completed_identifier()) {
                processor::low_priority_hint();
            }
            let registers = GpuRegisters::get();
            while registers.register(SCANOUT_ADDRESS).read() != flip.address.0 {
                processor::low_priority_hint();
            }
            // A write during the blank that latched the flip takes effect at
            // once, so waiting out that blank keeps it to one flip per refresh.
            while registers.register(SCANOUT_UPDATE).read() & (UPDATE_PENDING | UPDATE_TAKEN) != 0 {
                processor::low_priority_hint();
            }
        }

        /// Point the scanout at `address` and issue `VdSwap`.
        fn submit_flip(&mut self, address: GpuAddress, fetch_constant: &[u32; 6]) {
            let mut system_buffer = SystemCommandBuffer::new();
            let mut identifier = 0;
            // SAFETY: both outputs are writable.
            unsafe { VdGetSystemCommandBuffer(&mut system_buffer, &mut identifier) };

            let identifier_address = self.ring.identifier_address();
            let words = self.ring.reserve(FLIP_DWORDS + SWAP_DWORDS);
            words[0] = type0(SCANOUT_ADDRESS, 1);
            words[1] = address.0;
            words[2] = type3(OPCODE_REGISTER_READ_MODIFY_WRITE, 3);
            words[3] = u32::from(SCANOUT_CONTROL);
            words[4] = SCANOUT_CONTROL_KEEP;
            words[5] = 0;
            words[6] = type0(SCANOUT_FORMAT_1930, 1);
            words[7] = 0;
            let swap = &mut words[FLIP_DWORDS as usize..];
            let mut outputs = [0u32; 5];
            let [out0, out1, out2, out3, out4] = &mut outputs;
            // SAFETY: `swap` has the 64 dwords VdSwap writes; the identifier
            // address is the writeback dword the ring registered.
            unsafe {
                VdSwap(
                    swap.as_mut_ptr(),
                    fetch_constant.as_ptr(),
                    identifier_address,
                    &system_buffer,
                    identifier,
                    out0,
                    out1,
                    out2,
                    out3,
                    out4,
                )
            };
            if system_buffer.address != 0 {
                self.shader_epoch = ShaderEpoch::next();
            }
            self.ring.commit(FLIP_DWORDS + SWAP_DWORDS);
            self.ring.kick();
            self.retrain_edram(Retraining::AfterSwap);
            self.pending = Some(Flip { identifier: SwapIdentifier(identifier), address });
        }
    }

    impl Drop for Display {
        fn drop(&mut self) {
            self.wait_for_flip();
            // Leave the scanout as the kernel's own ring does with no title:
            // address 0, so it never reads freed memory.
            let shown = self.buffers[self.back ^ 1].fetch_constant;
            self.submit_flip(GpuAddress(0), &shown);
            self.wait_for_flip();
            self.ring.detach();
            release_engines();
        }
    }

    fn release_engines() {
        if ENGINES_OWNED.swap(false, Ordering::AcqRel) {
            // SAFETY: the engines are owned by this title and idle.
            unsafe { VdShutdownEngines() };
        }
    }
}
