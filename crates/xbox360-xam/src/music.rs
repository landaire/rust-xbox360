//! The system music player and music library requests of XAM application
//! 0xFA as the 2.0.6717 dashboard sends them: request wrappers 0x9219F000
//! to 0x921A2000, called through 0x921E8830 to 0x921E8CB0. The first word
//! of every request is 0 at every dashboard call site; requests whose first
//! word is 0 carry the extended block 0x80002, 0 when they go through
//! `XMsgStartIORequestEx`.

use core::borrow::Borrow;
use core::borrow::BorrowMut;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ptr;

use xbox360_status::Hresult;
use xbox360_status::Win32Error;
use xbox360_sys::xam;

use crate::WideCStr;
use crate::media::DEVICE_DESCRIPTOR_SIZE;
use crate::media_library;
use crate::media_library::EnumerationKind;
use crate::media_library::EnumerationRequest;
use crate::media_library::RECORD_HEADER_SIZE;
use crate::title::Enumerator;

/// XAM application of the music player.
const MEDIA_APP: u32 = 0xFA;

/// First word of every request.
const CLIENT: u32 = 0;

/// The extended block of requests whose first word is 0.
const EXTENDED: [u32; 2] = [0x0008_0002, 0];

/// Size of a song record.
pub const SONG_RECORD_SIZE: usize = 0x3D8;

/// Size of a container record.
pub const CONTAINER_RECORD_SIZE: usize = 0x334;

/// Size of a queue record: the entry id and the address of its song.
pub const QUEUE_RECORD_SIZE: usize = 8;

/// Size of the current song block of 0x7000E (0x921DD188): the entry id,
/// the song record and its flags word.
pub const CURRENT_SONG_SIZE: usize = 0x3E0;

/// Identity of a queue entry: the first word of a queue record and of the
/// current song block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntryId(pub u32);

/// Direction of 0x7000A (0x921E1B80).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum MoveDirection {
    /// One place toward the end.
    Down = 0,
    /// One place toward the start.
    Up = 1,
}

/// Outputs of 0x70052 (0x921A1620).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transport {
    pub state: u32,
    pub can_play: u32,
    pub elapsed_ms: u32,
    pub status: u32,
}

/// Why a request was not sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MusicError {
    /// XAM reported a failure.
    Xam(Hresult),
    /// A record buffer is not a whole number of records.
    RecordLength { length: usize, record_size: usize },
}

impl core::fmt::Display for MusicError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Xam(result) => write!(f, "XAM failed with {:#010x}", result.0),
            Self::RecordLength { length, record_size } => {
                write!(f, "{length} bytes are not records of {record_size:#x} bytes")
            }
        }
    }
}

impl core::error::Error for MusicError {}

impl From<Hresult> for MusicError {
    fn from(result: Hresult) -> Self {
        Self::Xam(result)
    }
}

/// Memory handed to XAM: an owned buffer whose bytes stay at one address
/// while the value lives, also when it moves.
///
/// # Safety
///
/// `bytes_mut` returns the same address and length on every call for the
/// life of the value, and nothing else accesses the bytes.
pub unsafe trait StableBuffer {
    fn bytes_mut(&mut self) -> &mut [u8];
}

/// 0x7002E (0x9219F080): the handle memory for a queue of `capacity`
/// songs.
pub fn player_memory_size(capacity: u32) -> Result<u32, MusicError> {
    const MESSAGE: u32 = 0x7_002E;

    let mut size = 0u32;
    // Pointers are 32-bit on the console.
    let mut words = [CLIENT, capacity, ptr::addr_of_mut!(size) as u32];
    // SAFETY: `words` and `size` live until the synchronous request
    // returns.
    unsafe { io_request_ex(MESSAGE, &mut words) }?;

    Ok(size)
}

/// The player handle of 0x7000D (0x921E8CB0 keeps it at 0x92873690). It
/// owns its memory and the song record buffer, which XAM uses until the
/// handle closes on drop or [`Player::close`].
pub struct Player<M>
where
    M: StableBuffer,
{
    handle: u32,
    /// Address ranges of the handle memory and the record buffer, where XAM
    /// keeps the queued songs.
    memory_range: core::ops::Range<u32>,
    records_range: core::ops::Range<u32>,
    /// Kept until the handle is closed; dropped after [`Drop::drop`].
    _memory: M,
    _records: M,
}

impl<M> Player<M>
where
    M: StableBuffer,
{
    /// 0x7000D (0x9219F080), then 0x7002F (0x921A04E0): the handle for a
    /// queue of `capacity` songs in `memory`, sized by
    /// [`player_memory_size`], with the song record buffer `records`. The
    /// dashboard leaves the fourth and fifth words of 0x7002F unset; they
    /// are 0 here.
    pub fn create(mut memory: M, mut records: M, capacity: u32) -> Result<Self, MusicError> {
        const CREATE: u32 = 0x7_000D;
        const ATTACH: u32 = 0x7_002F;

        let memory_bytes = memory.bytes_mut();
        let memory_size = u32::try_from(memory_bytes.len()).map_err(|_| Hresult::INVALID_ARGUMENT)?;
        // Pointers are 32-bit on the console.
        let memory_start = memory_bytes.as_mut_ptr() as u32;
        let records_bytes = records.bytes_mut();
        let records_size = u32::try_from(records_bytes.len()).map_err(|_| Hresult::INVALID_ARGUMENT)?;
        let records_start = records_bytes.as_mut_ptr() as u32;

        let mut handle = 0u32;
        let mut words = [CLIENT, memory_start, memory_size, 0, capacity, 0, 0, 0, ptr::addr_of_mut!(handle) as u32];
        // SAFETY: `words` and `handle` live until the synchronous request
        // returns; the player owns `memory` until the handle closes.
        unsafe { io_request_ex(CREATE, &mut words) }?;
        let player = Self {
            handle,
            memory_range: memory_start..memory_start + memory_size,
            records_range: records_start..records_start + records_size,
            _memory: memory,
            _records: records,
        };

        let mut words = [CLIENT, records_start, records_size, 0, 0, handle];
        // SAFETY: as above for `records`. On failure the player closes on
        // drop, as 0x921A04E0 closes the handle.
        unsafe { io_request_ex(ATTACH, &mut words) }?;

        Ok(player)
    }

    /// Copies the bytes at `address` to `out` when they lie inside the
    /// handle memory or the record buffer, where the addresses of queue
    /// records point. Returns whether it copied. Exclusive, as a pending
    /// [`RecordListing`] writes the record buffer.
    pub fn read_block(&mut self, address: u32, out: &mut [u8]) -> bool {
        let records = self.records_range.clone();
        self.read_within(&[self.memory_range.clone(), records], address, out)
    }

    /// [`Player::read_block`] limited to the handle memory, also while a
    /// [`RecordListing`] writes the record buffer.
    pub fn read_handle_block(&self, address: u32, out: &mut [u8]) -> bool {
        self.read_within(&[self.memory_range.clone()], address, out)
    }

    fn read_within(&self, ranges: &[core::ops::Range<u32>], address: u32, out: &mut [u8]) -> bool {
        let Ok(length) = u32::try_from(out.len()) else {
            return false;
        };
        let Some(end) = address.checked_add(length) else {
            return false;
        };
        if !ranges.iter().any(|range| range.start <= address && end <= range.end) {
            return false;
        }

        for (offset, byte) in out.iter_mut().enumerate() {
            // SAFETY: the range lies inside memory the player owns, so it is
            // allocated, and `StableBuffer` keeps other accesses away; the
            // record buffer is read only through `&mut self`, which no
            // pending listing allows. The read is volatile because XAM
            // writes the memory.
            *byte = unsafe { ptr::read_volatile((address as usize + offset) as *const u8) };
        }

        true
    }

    /// 0x70013 (0x9219F230): closes the handle. The memory is released
    /// afterwards.
    pub fn close(self) -> Result<(), MusicError> {
        let this = core::mem::ManuallyDrop::new(self);
        let result = close_handle(this.handle);

        // SAFETY: `this` is not used again and its drop does not run, so
        // the buffers are read out once and dropped here.
        drop(unsafe { ptr::read(&this._memory) });
        // SAFETY: as above.
        drop(unsafe { ptr::read(&this._records) });

        result
    }

    /// 0x70031 (0x921A0740): nonzero while the player has media loaded.
    pub fn state(&self) -> Result<u32, MusicError> {
        const MESSAGE: u32 = 0x7_0031;

        let mut state = 0u32;
        let mut words = [CLIENT, self.handle, ptr::addr_of_mut!(state) as u32];
        // SAFETY: `words` and `state` live until the synchronous request
        // returns.
        unsafe { io_request(MESSAGE, &mut words) }?;

        Ok(state)
    }

    /// 0x70028 (0x921A03A8): empties the queue.
    pub fn clear_queue(&self) -> Result<(), MusicError> {
        const MESSAGE: u32 = 0x7_0028;

        let mut words = [CLIENT, self.handle];
        // SAFETY: `words` lives until the synchronous request returns.
        unsafe { io_request_ex(MESSAGE, &mut words) }
    }

    /// 0x70011 (0x921A0158): appends the song records of `records`.
    pub fn enqueue(&self, records: &[u8]) -> Result<(), MusicError> {
        const MESSAGE: u32 = 0x7_0011;

        let count = record_count(records, SONG_RECORD_SIZE)?;
        let mut words = [CLIENT, self.handle, records.as_ptr() as u32, 0, count, 0];
        // SAFETY: `words` and `records` live until the synchronous request
        // returns.
        unsafe { io_request(MESSAGE, &mut words) }
    }

    /// 0x70012 (0x921A0208) with list 1: removes the songs of the song
    /// records `records` from the queue (0x921E0550, 0x921DE670).
    pub fn remove_songs(&self, records: &[u8]) -> Result<(), MusicError> {
        const SONG_RECORDS: u32 = 1;

        let count = record_count(records, SONG_RECORD_SIZE)?;
        // SAFETY: `records` holds `count` song records.
        unsafe { self.remove(records.as_ptr(), count, SONG_RECORDS) }
    }

    /// 0x70012 (0x921A0208) with list 0: removes the entries of the queue
    /// records `entries` (0x921E2648).
    pub fn remove_entries(&self, entries: &[u8]) -> Result<(), MusicError> {
        const QUEUE_RECORDS: u32 = 0;

        let count = record_count(entries, QUEUE_RECORD_SIZE)?;
        // SAFETY: `entries` holds `count` queue records.
        unsafe { self.remove(entries.as_ptr(), count, QUEUE_RECORDS) }
    }

    unsafe fn remove(&self, items: *const u8, count: u32, list: u32) -> Result<(), MusicError> {
        const MESSAGE: u32 = 0x7_0012;

        let mut words = [CLIENT, self.handle, items as u32, count, list];
        // SAFETY: `words` lives until the synchronous request returns; the
        // caller provides the records.
        unsafe { io_request_ex(MESSAGE, &mut words) }
    }

    /// 0x7000A (0x921A0450): moves a queue entry one place.
    pub fn move_entry(&self, entry: EntryId, direction: MoveDirection) -> Result<(), MusicError> {
        const MESSAGE: u32 = 0x7_000A;

        let mut words = [CLIENT, self.handle, entry.0, direction as u32];
        // SAFETY: `words` lives until the synchronous request returns.
        unsafe { io_request(MESSAGE, &mut words) }
    }

    /// 0x70056 (0x9219F3C0). The dashboard passes entry 0 with `start` 1
    /// after it fills the queue (0x921DA580) and a queue entry with
    /// `start` 0 for a row of the queue list (0x921DCC60).
    pub fn play_entry(&self, entry: EntryId, start: u32) -> Result<(), MusicError> {
        const MESSAGE: u32 = 0x7_0056;

        let mut words = [CLIENT, self.handle, entry.0, start];
        // SAFETY: `words` lives until the call returns.
        unsafe { system_call(MESSAGE, &mut words) }
    }

    /// 0x70041 (0x921A0028) and 0x921DBDA8: the queue as queue records of
    /// [`QUEUE_RECORD_SIZE`] bytes in `buffer`, one `XamEnumerate`. Returns
    /// the record count XAM stores. As there, the result of
    /// `XamEnumerate` is not read.
    pub fn read_queue(&self, buffer: &mut [u8]) -> Result<u32, MusicError> {
        const NO_FLAGS: u32 = 0;

        let count = record_count(buffer, QUEUE_RECORD_SIZE)?;
        let words = [CLIENT, self.handle];
        // SAFETY: the words name only the handle, which outlives the
        // enumeration closed in this call.
        let enumerator = unsafe { media_library::start_raw(EnumerationKind::QUEUE, &words, count) }?;

        let mut items = 0u32;
        let size = u32::try_from(buffer.len()).map_err(|_| Hresult::INVALID_ARGUMENT)?;
        // SAFETY: XAM writes at most `size` bytes to `buffer` and the count
        // to `items` before the synchronous call returns.
        let _ = unsafe { enumerator.enumerate_counted(NO_FLAGS, buffer.as_mut_ptr(), size, &mut items) };

        Ok(items.min(count))
    }

    /// 0x70011 (0x921DA580) of the first `count` records of the record
    /// buffer, which a completed record listing filled.
    fn enqueue_listed(&mut self, count: u32) -> Result<(), MusicError> {
        const MESSAGE: u32 = 0x7_0011;

        let records = self.records_range.end - self.records_range.start;
        if u64::from(count) * SONG_RECORD_SIZE as u64 > u64::from(records) {
            return Err(Hresult::INVALID_ARGUMENT.into());
        }

        let mut words = [CLIENT, self.handle, self.records_range.start, 0, count, 0];
        // SAFETY: `words` lives until the synchronous request returns; the
        // records lie in memory the player owns.
        unsafe { io_request(MESSAGE, &mut words) }
    }
}

/// Size of the scratch memory of a [`RecordListing`]: its overlapped block
/// and a copy of the descriptor or record the request names.
pub const LISTING_SCRATCH_SIZE: usize = 0x260;

/// The song enumeration of a record listing (0x921DB1D8 passes flags 4).
#[derive(Clone, Copy, Debug)]
pub struct SongListing<'a> {
    pub scope: media_library::ListingScope<'a>,
    pub sort: u32,
    pub flags: u32,
}

/// State of a [`RecordListing`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListingState {
    Pending,
    /// The call is over: the record count when the extended error is
    /// nonnegative or `ERROR_NO_MORE_ITEMS` as an HRESULT (0x80070103),
    /// as 0x921DC958 tests; `None` for any other.
    Complete {
        records: Option<u32>,
    },
}

/// Why [`RecordListing::start`] did not start; the player and the scratch
/// memory come back.
pub struct ListingStartError<P, S> {
    pub player: P,
    pub scratch: S,
    pub error: MusicError,
}

/// 0x921DB1D8: one asynchronous `XamEnumerate` of songs into the record
/// buffer of the player. The listing holds the player exclusively (`P` is
/// the player or a `&mut` of it) until [`RecordListing::finish`] or drop,
/// which cancel a pending call and wait for it, so nothing else reads or
/// writes the record buffer meanwhile. Requests that do not touch the
/// record buffer go through [`RecordListing::player`].
pub struct RecordListing<P, M, S>
where
    P: BorrowMut<Player<M>>,
    M: StableBuffer,
    S: StableBuffer,
{
    /// Open until the call is over (0x921DC958 closes it then).
    enumerator: Option<Enumerator>,
    overlapped: *mut xam::Overlapped,
    /// Holds the overlapped block and the request copy.
    scratch: ManuallyDrop<S>,
    player: ManuallyDrop<P>,
    _buffer: PhantomData<M>,
}

impl<P, M, S> RecordListing<P, M, S>
where
    P: BorrowMut<Player<M>>,
    M: StableBuffer,
    S: StableBuffer,
{
    /// Starts the enumeration `request` for at most `count` records and
    /// one `XamEnumerate` of the whole record buffer. A call that does not
    /// pend is a failure, as in 0x921DB1D8. `scratch` holds at least
    /// [`LISTING_SCRATCH_SIZE`] bytes, 4-byte aligned.
    pub fn start(
        mut player: P,
        mut scratch: S,
        request: SongListing<'_>,
        count: u32,
    ) -> Result<Self, ListingStartError<P, S>> {
        const NO_FLAGS: u32 = 0;
        const REQUEST_OFFSET: usize = 0x20;

        let result = (|| {
            let records_range = Borrow::<Player<M>>::borrow(&player).records_range.clone();
            let records = records_range.end - records_range.start;
            if u64::from(count) * SONG_RECORD_SIZE as u64 > u64::from(records) {
                return Err(MusicError::from(Hresult::INVALID_ARGUMENT));
            }
            let bytes = scratch.bytes_mut();
            if bytes.len() < LISTING_SCRATCH_SIZE || bytes.as_ptr() as usize % 4 != 0 {
                return Err(Hresult::INVALID_ARGUMENT.into());
            }

            let (head, copy) = bytes.split_at_mut(REQUEST_OFFSET);
            let scope = match request.scope {
                media_library::ListingScope::Device(descriptor) => {
                    let copy: &mut [u8; DEVICE_DESCRIPTOR_SIZE] =
                        (&mut copy[..DEVICE_DESCRIPTOR_SIZE]).try_into().map_err(|_| Hresult::INVALID_ARGUMENT)?;
                    copy.copy_from_slice(descriptor);
                    media_library::ListingScope::Device(copy)
                }
                media_library::ListingScope::Record(record) => {
                    let copy: &mut [u8; RECORD_HEADER_SIZE] =
                        (&mut copy[..RECORD_HEADER_SIZE]).try_into().map_err(|_| Hresult::INVALID_ARGUMENT)?;
                    copy.copy_from_slice(record);
                    media_library::ListingScope::Record(copy)
                }
            };
            let request = EnumerationRequest::Songs { scope, sort: request.sort, flags: request.flags };
            // The scratch memory holds the copy until the listing ends.
            let enumerator = media_library::start_enumeration(request, count)?.into_enumerator();

            let overlapped = head.as_mut_ptr().cast::<xam::Overlapped>();
            // SAFETY: the scratch memory is 4-byte aligned, holds the block,
            // and the listing owns it.
            unsafe { ptr::write(overlapped, xam::Overlapped::default()) };
            // SAFETY: the record buffer lies in memory the player owns, and
            // the listing holds the player, the block and the enumeration
            // until the call is over or cancelled.
            let code =
                unsafe { enumerator.enumerate(NO_FLAGS, records_range.start as usize as *mut u8, records, overlapped) };
            if code.error() != Some(Win32Error::IO_PENDING) {
                return Err(Hresult::FAIL.into());
            }

            Ok((enumerator, overlapped))
        })();

        match result {
            Ok((enumerator, overlapped)) => Ok(Self {
                enumerator: Some(enumerator),
                overlapped,
                scratch: ManuallyDrop::new(scratch),
                player: ManuallyDrop::new(player),
                _buffer: PhantomData,
            }),
            Err(error) => Err(ListingStartError { player, scratch, error }),
        }
    }

    /// The player for requests that do not touch the record buffer.
    pub fn player(&self) -> &Player<M> {
        Borrow::<Player<M>>::borrow(&*self.player)
    }

    /// The state of the call; once it is over the enumeration closes, as
    /// in 0x921DC958.
    pub fn state(&mut self) -> ListingState {
        const NO_MORE_ITEMS: u32 = 0x8007_0103;

        if self.is_pending() {
            return ListingState::Pending;
        }
        self.enumerator = None;

        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        // SAFETY: the block is live and the call no longer writes it.
        let block = unsafe { ptr::read_volatile(self.overlapped) };
        let succeeded = (block.extended_error as i32) >= 0 || block.extended_error == NO_MORE_ITEMS;

        ListingState::Complete { records: succeeded.then_some(block.length) }
    }

    /// 0x70011 (0x921DA580) of the first `count` records the completed
    /// listing wrote.
    pub fn enqueue(&mut self, count: u32) -> Result<(), MusicError> {
        match self.state() {
            ListingState::Complete { records: Some(records) } if count <= records => {
                BorrowMut::<Player<M>>::borrow_mut(&mut *self.player).enqueue_listed(count)
            }
            _ => Err(Hresult::INVALID_ARGUMENT.into()),
        }
    }

    /// Cancels a pending call, waiting for it (0x921DA1C8), and gives the
    /// player and the scratch memory back.
    pub fn finish(self) -> (P, S) {
        let mut this = ManuallyDrop::new(self);
        this.cancel();
        this.enumerator = None;

        // SAFETY: `this` is not dropped and not used after the fields are
        // read out once.
        unsafe { (ManuallyDrop::take(&mut this.player), ManuallyDrop::take(&mut this.scratch)) }
    }

    fn is_pending(&self) -> bool {
        // SAFETY: the block is live; XAM writes the result field last and
        // the volatile read observes its stores.
        let result = unsafe { ptr::read_volatile(&raw const (*self.overlapped).result) };
        result.error() == Some(Win32Error::IO_PENDING)
    }

    fn cancel(&mut self) {
        if !self.is_pending() {
            return;
        }

        let wait = true;
        // SAFETY: the block names the pending call and lives through it.
        unsafe { xam::XMsgCancelIORequest(self.overlapped, wait) };
    }
}

impl<P, M, S> Drop for RecordListing<P, M, S>
where
    P: BorrowMut<Player<M>>,
    M: StableBuffer,
    S: StableBuffer,
{
    fn drop(&mut self) {
        self.cancel();
        self.enumerator = None;

        // SAFETY: the call is over, so the scratch memory and the player
        // are released once, here.
        unsafe {
            ManuallyDrop::drop(&mut self.scratch);
            ManuallyDrop::drop(&mut self.player);
        }
    }
}

impl<M> Drop for Player<M>
where
    M: StableBuffer,
{
    fn drop(&mut self) {
        let _ = close_handle(self.handle);
    }
}

/// Memory the player uses for playback, given by 0x70044 (0x921A06A0) with
/// mode 0 and taken back with mode 2 on drop (0x921E8988).
pub struct ServiceMemory<M>
where
    M: StableBuffer,
{
    /// Kept until XAM gives it back; dropped after [`Drop::drop`].
    _memory: M,
}

impl<M> ServiceMemory<M>
where
    M: StableBuffer,
{
    pub fn give(mut memory: M) -> Result<Self, MusicError> {
        const GIVE: u32 = 0;

        let bytes = memory.bytes_mut();
        let size = u32::try_from(bytes.len()).map_err(|_| Hresult::INVALID_ARGUMENT)?;
        // SAFETY: the value owns `memory` until drop takes it back.
        unsafe { service_memory(GIVE, bytes.as_mut_ptr() as u32, size) }?;

        Ok(Self { _memory: memory })
    }
}

impl<M> Drop for ServiceMemory<M>
where
    M: StableBuffer,
{
    fn drop(&mut self) {
        const RECLAIM: u32 = 2;

        // SAFETY: the request names no memory.
        let _ = unsafe { service_memory(RECLAIM, 0, 0) };
    }
}

/// 0x70052 (0x921A1620).
pub fn transport() -> Result<Transport, MusicError> {
    const MESSAGE: u32 = 0x7_0052;

    let mut transport = Transport { state: 0, can_play: 0, elapsed_ms: 0, status: 0 };
    let mut words = [
        CLIENT,
        ptr::addr_of_mut!(transport.state) as u32,
        ptr::addr_of_mut!(transport.can_play) as u32,
        ptr::addr_of_mut!(transport.elapsed_ms) as u32,
        ptr::addr_of_mut!(transport.status) as u32,
    ];
    // SAFETY: `words` and the outputs live until the call returns.
    unsafe { in_process(MESSAGE, &mut words) }?;

    Ok(transport)
}

/// 0x70053 (0x921A16A0): nonzero while a title controls playback.
pub fn title_controls_playback() -> Result<u32, MusicError> {
    const MESSAGE: u32 = 0x7_0053;

    let mut value = 0u32;
    let mut words = [CLIENT, ptr::addr_of_mut!(value) as u32];
    // SAFETY: `words` and `value` live until the call returns.
    unsafe { in_process(MESSAGE, &mut words) }?;

    Ok(value)
}

/// 0x7000E (0x9219F320) with the second word 0: the current song block.
pub fn current_song(block: &mut [u8; CURRENT_SONG_SIZE]) -> Result<(), MusicError> {
    const MESSAGE: u32 = 0x7_000E;

    let mut words = [CLIENT, 0, block.as_mut_ptr() as u32];
    // SAFETY: `words` and `block` live until the synchronous request
    // returns.
    unsafe { io_request(MESSAGE, &mut words) }
}

/// 0x70049 (0x921A1290): the play/pause button of Now Playing sends it
/// while the 0x70052 state is not 1, and 0x70005 while it is (0x921DCEE0).
pub fn play_pause() -> Result<(), MusicError> {
    simple_call(0x7_0049)
}

/// 0x70003 (0x9219F430).
pub fn resume() -> Result<(), MusicError> {
    simple_call(0x7_0003)
}

/// 0x70005 (0x9219F4B8).
pub fn pause() -> Result<(), MusicError> {
    simple_call(0x7_0005)
}

/// 0x70006 (0x9219F4F0).
pub fn next() -> Result<(), MusicError> {
    simple_call(0x7_0006)
}

/// 0x70007 (0x9219F528).
pub fn previous() -> Result<(), MusicError> {
    simple_call(0x7_0007)
}

/// 0x70004 (0x9219F468) with the second word 1, as at every call site.
pub fn stop() -> Result<(), MusicError> {
    const MESSAGE: u32 = 0x7_0004;

    let mut words = [CLIENT, 1];
    // SAFETY: `words` lives until the call returns.
    unsafe { system_call(MESSAGE, &mut words) }
}

/// 0x70051 (0x921A1490) with the second word 1, as at every call site.
pub fn playback_mode() -> Result<u32, MusicError> {
    const MESSAGE: u32 = 0x7_0051;

    let mut mode = 0u32;
    let mut words = [CLIENT, 1, ptr::addr_of_mut!(mode) as u32, 0, 0];
    // SAFETY: `words` and `mode` live until the call returns.
    unsafe { in_process(MESSAGE, &mut words) }?;

    Ok(mode)
}

/// 0x70050 (0x921A13F0) with the last word 1, as at its call site
/// (0x922913F8).
pub fn set_playback_mode(mode: u32) -> Result<(), MusicError> {
    const MESSAGE: u32 = 0x7_0050;

    let mut words = [CLIENT, mode, 0, 0, 1];
    // SAFETY: `words` lives until the synchronous request returns.
    unsafe { io_request(MESSAGE, &mut words) }
}

/// 0x7004E (0x921A1510) with the second word 1: the volume as a fraction.
pub fn volume() -> Result<f32, MusicError> {
    const MESSAGE: u32 = 0x7_004E;

    let mut volume = 0f32;
    let mut words = [CLIENT, 1, ptr::addr_of_mut!(volume) as u32];
    // SAFETY: `words` and `volume` live until the call returns.
    unsafe { in_process(MESSAGE, &mut words) }?;

    Ok(volume)
}

/// 0x7004F (0x921A1588) with the third word 1.
pub fn set_volume(volume: f32) -> Result<(), MusicError> {
    const MESSAGE: u32 = 0x7_004F;

    let mut words = [CLIENT, volume.to_bits(), 1];
    // SAFETY: `words` lives until the synchronous request returns.
    unsafe { io_request(MESSAGE, &mut words) }
}

/// 0x70036 (0x921A09A8): deletes the song of `record` from the library.
pub fn delete_song(record: &[u8; SONG_RECORD_SIZE]) -> Result<(), MusicError> {
    // SAFETY: `record` is a song record that lives through the request.
    unsafe { record_request(0x7_0036, record.as_ptr()) }
}

/// 0x70037 (0x921A0A50): deletes the container of `record`.
pub fn delete_container(record: &[u8; CONTAINER_RECORD_SIZE]) -> Result<(), MusicError> {
    // SAFETY: `record` is a container record that lives through the
    // request.
    unsafe { record_request(0x7_0037, record.as_ptr()) }
}

/// 0x7003C (0x921A0CD8): renames the playlist of `record` to `name`.
pub fn rename_playlist(record: &[u8; CONTAINER_RECORD_SIZE], name: &WideCStr) -> Result<(), MusicError> {
    const MESSAGE: u32 = 0x7_003C;

    let mut words = [CLIENT, record.as_ptr() as u32, name.as_ptr() as u32];
    // SAFETY: `words`, the record and the name live until the synchronous
    // request returns.
    unsafe { io_request_ex(MESSAGE, &mut words) }
}

/// Tags that changed (0x921E6310).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChangedTags(pub u32);

impl ChangedTags {
    pub const TITLE: Self = Self(0x1);
    pub const ARTIST: Self = Self(0x2);
    pub const GENRE: Self = Self(0x4);
}

/// The new tags of 0x70034 and 0x70035 (0x921E6458).
pub struct TagUpdate<'a> {
    pub changed: ChangedTags,
    pub title: &'a WideCStr,
    pub artist: &'a WideCStr,
    pub genre: &'a WideCStr,
}

/// 0x70034 (0x921A0858): the tags of the album of `record`.
pub fn update_album_tags(record: &[u8; CONTAINER_RECORD_SIZE], update: &TagUpdate<'_>) -> Result<(), MusicError> {
    // SAFETY: the record lives through the request.
    unsafe { tag_request(0x7_0034, record.as_ptr(), update) }
}

/// 0x70035 (0x921A0900): the tags of the song of `record`.
pub fn update_song_tags(record: &[u8; SONG_RECORD_SIZE], update: &TagUpdate<'_>) -> Result<(), MusicError> {
    // SAFETY: the record lives through the request.
    unsafe { tag_request(0x7_0035, record.as_ptr(), update) }
}

/// The parameter block of 0x921E6458: record, mask, title, artist, genre,
/// and zeroed words (one for albums, three for songs; three here).
unsafe fn tag_request(message: u32, record: *const u8, update: &TagUpdate<'_>) -> Result<(), MusicError> {
    let block = [
        record as u32,
        update.changed.0,
        update.title.as_ptr() as u32,
        update.artist.as_ptr() as u32,
        update.genre.as_ptr() as u32,
        0,
        0,
        0,
    ];
    // SAFETY: `block` and the strings live through the request; the caller
    // provides the record.
    unsafe { record_request(message, block.as_ptr().cast()) }
}

fn close_handle(handle: u32) -> Result<(), MusicError> {
    const MESSAGE: u32 = 0x7_0013;

    let mut words = [CLIENT, handle];
    // SAFETY: `words` lives until the synchronous request returns.
    unsafe { io_request_ex(MESSAGE, &mut words) }
}

fn record_count(records: &[u8], record_size: usize) -> Result<u32, MusicError> {
    if records.len() % record_size != 0 {
        return Err(MusicError::RecordLength { length: records.len(), record_size });
    }

    u32::try_from(records.len() / record_size).map_err(|_| Hresult::INVALID_ARGUMENT.into())
}

/// 0x70044 returns the thread's last error after the request starts
/// (0x921A06A0), and its caller fails on any nonzero value.
unsafe fn service_memory(mode: u32, buffer: u32, size: u32) -> Result<(), MusicError> {
    const MESSAGE: u32 = 0x7_0044;

    let mut words = [CLIENT, mode, buffer, size];
    let mut extended = EXTENDED;
    // SAFETY: `words` and `extended` live until the synchronous request
    // returns; the caller keeps the named buffer.
    let result = unsafe {
        xam::XMsgStartIORequestEx(
            MEDIA_APP,
            MESSAGE,
            ptr::null_mut(),
            words.as_mut_ptr().cast(),
            size_of_val(&words) as u32,
            extended.as_mut_ptr(),
        )
    };
    if result.is_failure() {
        return Err(result.into());
    }

    match xbox360_status::last_error() {
        None => Ok(()),
        Some(error) if (error.code() as i32) < 0 => Err(Hresult(error.code()).into()),
        Some(error) => Err(Hresult::from_win32(error).into()),
    }
}

unsafe fn record_request(message: u32, record: *const u8) -> Result<(), MusicError> {
    let mut words = [CLIENT, record as u32];
    // SAFETY: `words` lives until the synchronous request returns; the
    // caller provides the record.
    unsafe { io_request_ex(message, &mut words) }
}

fn simple_call(message: u32) -> Result<(), MusicError> {
    let mut words = [CLIENT];
    // SAFETY: `words` lives until the call returns.
    unsafe { system_call(message, &mut words) }
}

/// `XMsgStartIORequestEx` with the extended block, synchronous.
unsafe fn io_request_ex(message: u32, words: &mut [u32]) -> Result<(), MusicError> {
    let mut extended = EXTENDED;
    // SAFETY: the caller keeps `words` and the memory it names valid until
    // the request completes.
    let result = unsafe {
        xam::XMsgStartIORequestEx(
            MEDIA_APP,
            message,
            ptr::null_mut(),
            words.as_mut_ptr().cast(),
            size_of_val(words) as u32,
            extended.as_mut_ptr(),
        )
    };

    synchronous_result(result)
}

/// `XMsgStartIORequest`, synchronous.
unsafe fn io_request(message: u32, words: &mut [u32]) -> Result<(), MusicError> {
    // SAFETY: the caller keeps `words` and the memory it names valid until
    // the synchronous request returns.
    let result = unsafe {
        xam::XMsgStartIORequest(
            MEDIA_APP,
            message,
            ptr::null_mut(),
            words.as_mut_ptr().cast(),
            size_of_val(words) as u32,
        )
    };

    synchronous_result(result)
}

/// The wrappers take the thread's last error after a synchronous request
/// as an HRESULT and fail on a negative one.
fn synchronous_result(result: Hresult) -> Result<(), MusicError> {
    if result.is_failure() {
        return Err(result.into());
    }

    match xbox360_status::last_error() {
        Some(error) if (error.code() as i32) < 0 => Err(Hresult(error.code()).into()),
        _ => Ok(()),
    }
}

/// `XMsgSystemProcessCall`; the dashboard does not read its result.
unsafe fn system_call(message: u32, words: &mut [u32]) -> Result<(), MusicError> {
    // SAFETY: the caller keeps `words` valid until the call returns.
    let result =
        unsafe { xam::XMsgSystemProcessCall(MEDIA_APP, message, words.as_mut_ptr().cast(), size_of_val(words) as u32) };
    if result.is_failure() { Err(result.into()) } else { Ok(()) }
}

/// `XMsgInProcessCall` with the request block as the first parameter.
unsafe fn in_process(message: u32, words: &mut [u32]) -> Result<(), MusicError> {
    // SAFETY: the caller keeps `words` and the outputs it names valid until
    // the call returns.
    let result = unsafe { xam::XMsgInProcessCall(MEDIA_APP, message, words.as_mut_ptr(), ptr::null_mut()) };
    if result.is_failure() { Err(result.into()) } else { Ok(()) }
}
