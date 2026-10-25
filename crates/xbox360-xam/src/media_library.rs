//! Media library requests of XAM application 0xFA as the 2.0.6717 dashboard
//! sends them: enumerations created with `XamCreateEnumeratorHandle` and
//! started with `XMsgInProcessCall` (0x9219F9A8, 0x9219FC48, 0x9219FD98),
//! and the content streams of media items (0x921A0F80, 0x9219F7C8).

use core::marker::PhantomData;
use core::ptr;
use core::ptr::NonNull;

use xbox360_status::Hresult;
use xbox360_status::Win32Error;
use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::Handle;

use crate::media::DEVICE_DESCRIPTOR_SIZE;
use crate::title::Enumerator;

/// XAM application of the media library.
const MEDIA_APP: u32 = 0xFA;

/// Size of the header that starts every media record; the item stream
/// requests take a copy of it.
pub const RECORD_HEADER_SIZE: usize = 0x23C;

/// One kind of media enumeration: the request that starts it, the messages
/// of its handle, the size of its private block and of its records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumerationKind {
    start: u32,
    enumerate: u32,
    close: u32,
    private_size: u32,
    record_size: u32,
}

impl EnumerationKind {
    /// 0x7001C (0x9219FC48): containers, 0x334-byte records. Request words
    /// 0, descriptor, container, 0, 1, flags, listing kind (0x800).
    pub const CONTAINERS: Self =
        Self { start: 0x7_001C, enumerate: 0x7_001D, close: 0x7_001E, private_size: 0x14E0, record_size: 0x334 };

    /// 0x7001F (0x9219FD98): media items, 0x330-byte records. Request words
    /// 0, descriptor, container, 0, 1, flags.
    pub const ITEMS: Self =
        Self { start: 0x7_001F, enumerate: 0x7_0020, close: 0x7_0021, private_size: 0x14D8, record_size: 0x330 };

    /// 0x7003E (0x9219F9A8): items and folders of one listing, 0x298-byte
    /// records. Request words 0, descriptor, container, 0, 1, flags,
    /// listing kind.
    pub const LISTING: Self =
        Self { start: 0x7_003E, enumerate: 0x7_003F, close: 0x7_0040, private_size: 0x13A8, record_size: 0x298 };

    /// 0x70014 (0x921A1080): songs, 0x3D8-byte records. Request words 0,
    /// descriptor, container, 0, sort order, 0 or 4.
    pub const SONGS: Self =
        Self { start: 0x7_0014, enumerate: 0x7_0015, close: 0x7_0016, private_size: 0x1628, record_size: 0x3D8 };

    /// 0x70041 (0x921A0028): the music player queue, 8-byte entries.
    /// Request words 0, player handle.
    pub const QUEUE: Self =
        Self { start: 0x7_0041, enumerate: 0x7_0042, close: 0x7_0043, private_size: 0x14, record_size: 0x8 };

    pub const fn record_size(self) -> usize {
        self.record_size as usize
    }
}

/// What a library enumeration lists: a whole device, or one record of it
/// such as a folder or an album. The dashboard passes one or the other,
/// never both.
#[derive(Clone, Copy, Debug)]
pub enum ListingScope<'a> {
    /// The device descriptor XAM reported (`media::devices`).
    Device(&'a [u8; DEVICE_DESCRIPTOR_SIZE]),
    /// A record of the device; the dashboard passes the record itself.
    Record(&'a [u8; RECORD_HEADER_SIZE]),
}

impl ListingScope<'_> {
    /// The descriptor and record words of a request.
    fn words(self) -> [u32; 2] {
        // Pointers are 32-bit on the console.
        match self {
            Self::Device(descriptor) => [descriptor.as_ptr() as u32, 0],
            Self::Record(record) => [0, record.as_ptr() as u32],
        }
    }
}

/// A library enumeration request.
#[derive(Clone, Copy, Debug)]
pub enum EnumerationRequest<'a> {
    /// [`EnumerationKind::CONTAINERS`]. `kind` selects the containers: the
    /// music library passes 8 albums, 0x10 artists, 0x20 genres, 0x40
    /// playlists; folder views pass 0x800.
    Containers { scope: ListingScope<'a>, flags: u32, kind: u32 },
    /// [`EnumerationKind::ITEMS`].
    Items { scope: ListingScope<'a>, flags: u32 },
    /// [`EnumerationKind::LISTING`].
    Listing { scope: ListingScope<'a>, flags: u32, kind: u32 },
    /// [`EnumerationKind::SONGS`].
    Songs { scope: ListingScope<'a>, sort: u32, flags: u32 },
}

impl EnumerationRequest<'_> {
    pub const fn kind(self) -> EnumerationKind {
        match self {
            Self::Containers { .. } => EnumerationKind::CONTAINERS,
            Self::Items { .. } => EnumerationKind::ITEMS,
            Self::Listing { .. } => EnumerationKind::LISTING,
            Self::Songs { .. } => EnumerationKind::SONGS,
        }
    }
}

/// An open library enumeration. XAM may keep the addresses of the request
/// in the handle, so the enumeration borrows what the request named.
pub struct MediaEnumeration<'a> {
    enumerator: Enumerator,
    borrowed: PhantomData<&'a ()>,
}

impl MediaEnumeration<'_> {
    pub(crate) fn into_enumerator(self) -> Enumerator {
        self.enumerator
    }
}

/// Opens the enumeration of `request` for at most `count` records.
pub fn start_enumeration<'a>(request: EnumerationRequest<'a>, count: u32) -> Result<MediaEnumeration<'a>, Hresult> {
    const CLIENT: u32 = 0;
    const LEVEL: u32 = 1;

    let kind = request.kind();
    let (words, length) = match request {
        EnumerationRequest::Containers { scope, flags, kind } => {
            let [descriptor, record] = scope.words();
            ([CLIENT, descriptor, record, 0, LEVEL, flags, kind], 7)
        }
        EnumerationRequest::Items { scope, flags } => {
            let [descriptor, record] = scope.words();
            ([CLIENT, descriptor, record, 0, LEVEL, flags, 0], 6)
        }
        EnumerationRequest::Listing { scope, flags, kind } => {
            let [descriptor, record] = scope.words();
            ([CLIENT, descriptor, record, 0, LEVEL, flags, kind], 7)
        }
        EnumerationRequest::Songs { scope, sort, flags } => {
            let [descriptor, record] = scope.words();
            ([CLIENT, descriptor, record, 0, sort, flags, 0], 6)
        }
    };

    // SAFETY: the words name only the borrowed descriptor or record, which
    // the returned enumeration keeps borrowed.
    let enumerator = unsafe { start_raw(kind, &words[..length], count) }?;

    Ok(MediaEnumeration { enumerator, borrowed: PhantomData })
}

/// Opens an enumeration of `kind` for at most `count` records. `words`
/// holds the words of the start request before the private block address,
/// which this call appends. A first word of 0 gets the handle flags
/// 0x20000001, any other 0, as at every dashboard call site.
///
/// # Safety
///
/// Every address in `words` names memory of the layout the request
/// expects, valid until the returned enumerator closes.
pub(crate) unsafe fn start_raw(kind: EnumerationKind, words: &[u32], count: u32) -> Result<Enumerator, Hresult> {
    const MAX_WORDS: usize = 8;
    const ANY_USER: u32 = u32::MAX;
    const FLAGS_FIRST_WORD_ZERO: u32 = 0x2000_0001;
    const PRIVATE_OFFSET: usize = 0x18;

    if words.len() >= MAX_WORDS {
        return Err(Hresult::INVALID_ARGUMENT);
    }
    let flags = if words.first() == Some(&0) { FLAGS_FIRST_WORD_ZERO } else { 0 };

    let mut handle = Handle::NULL;
    // SAFETY: `handle` is a live local.
    let created = unsafe {
        xam::XamCreateEnumeratorHandle(
            ANY_USER,
            MEDIA_APP,
            kind.enumerate,
            kind.close,
            kind.private_size,
            count,
            flags,
            &mut handle,
        )
    };
    created.result().map_err(Hresult::from_win32)?;
    let enumerator = Enumerator::from_handle(handle);

    let mut object: *mut u8 = ptr::null_mut();
    // SAFETY: `handle` is open and `object` is a live local.
    unsafe { xam::XamGetPrivateEnumStructureFromHandle(handle, &mut object) };
    let Some(object) = NonNull::new(object) else {
        return Err(Hresult::FAIL);
    };

    // SAFETY: the private block of the handle is `private_size` bytes from
    // the object's +0x18, and the reference keeps it alive.
    let private = unsafe { object.as_ptr().add(PRIVATE_OFFSET) };
    // SAFETY: as above; the dashboard zeroes the block before the request.
    unsafe { ptr::write_bytes(private, 0, kind.private_size as usize) };

    let mut request = [0u32; MAX_WORDS];
    request[..words.len()].copy_from_slice(words);
    // Pointers are 32-bit on the console.
    request[words.len()] = private as u32;
    // SAFETY: `request` lives until the in-process call returns, the
    // private block until the handle closes, and the caller keeps the
    // memory the words name.
    let started = unsafe { xam::XMsgInProcessCall(MEDIA_APP, kind.start, request.as_mut_ptr(), ptr::null_mut()) };

    // SAFETY: drops the reference XamGetPrivateEnumStructureFromHandle took.
    unsafe { xboxkrnl::ObDereferenceObject(object.as_ptr() as u32) };

    if started.is_failure() {
        return Err(started);
    }

    Ok(enumerator)
}

/// Reads the next records of `enumeration` into `buffer` and returns their
/// count, waiting for XAM. `Ok(0)` after the last record.
pub fn next_records(enumeration: &MediaEnumeration<'_>, buffer: &mut [u8]) -> Result<u32, Win32Error> {
    next_raw(&enumeration.enumerator, buffer)
}

pub(crate) fn next_raw(enumerator: &Enumerator, buffer: &mut [u8]) -> Result<u32, Win32Error> {
    const NO_FLAGS: u32 = 0;

    let size = u32::try_from(buffer.len()).map_err(|_| Win32Error::INVALID_PARAMETER)?;
    let mut items = 0u32;
    // SAFETY: XAM writes at most `size` bytes to `buffer` and the count to
    // `items` before the synchronous call returns.
    let result = unsafe { enumerator.enumerate_counted(NO_FLAGS, buffer.as_mut_ptr(), size, &mut items) };

    match result.error() {
        None => Ok(items),
        Some(Win32Error::NO_MORE_ITEMS) => Ok(0),
        Some(error) => Err(error),
    }
}

/// Which stream of a media item [`ItemStream::open`] opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemStreamSource {
    /// Message 0x7002C (0x921A0F80): the item itself.
    Content,
    /// Message 0x7002D (0x9219F7C8): the device's thumbnail of the item.
    Thumbnail,
}

/// The data form a stream request asks for: the record mask bit of the
/// item that permits it (record +0x28C or +0x32C). Only forms whose
/// request this module fills completely are offered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemStreamForm(u32);

impl ItemStreamForm {
    /// The encoded file, read as bytes.
    pub const FILE: Self = Self(0x40);

    /// The mask bit of the form.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// A byte stream of a media item, released on drop.
pub struct ItemStream {
    object: NonNull<u32>,
}

impl ItemStream {
    /// Opens a stream of the item whose record starts with `header`.
    pub fn open(
        source: ItemStreamSource,
        header: &[u8; RECORD_HEADER_SIZE],
        form: ItemStreamForm,
    ) -> Result<Self, Hresult> {
        let message = match source {
            ItemStreamSource::Content => 0x7_002C,
            ItemStreamSource::Thumbnail => 0x7_002D,
        };

        let mut header = *header;
        let mut object: u32 = 0;
        // Pointers are 32-bit on the console. The fourth word is an image
        // description object for form 0x80; the file form passes none.
        let mut words = [0, header.as_mut_ptr() as u32, form.0, 0, ptr::addr_of_mut!(object) as u32];
        // SAFETY: `words`, `header` and `object` live until the in-process
        // call returns.
        let result = unsafe { xam::XMsgInProcessCall(MEDIA_APP, message, words.as_mut_ptr(), ptr::null_mut()) };
        if result.is_failure() {
            return Err(result);
        }
        let object = NonNull::new(object as usize as *mut u32).ok_or(Hresult::FAIL)?;

        Ok(Self { object })
    }

    /// Reads up to `buffer.len()` bytes and returns how many were read; 0
    /// at the end of the stream.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Hresult> {
        const READ_SLOT: usize = 0x3C / 4;

        let size = u32::try_from(buffer.len()).map_err(|_| Hresult::INVALID_ARGUMENT)?;
        let mut read = 0u32;
        // SAFETY: the object is live until drop, and the callee writes at
        // most `size` bytes to `buffer` and the count to `read`.
        let result =
            Hresult(unsafe { self.call(READ_SLOT, buffer.as_mut_ptr() as u32, size, ptr::addr_of_mut!(read) as u32) });
        if result.is_failure() {
            return Err(result);
        }

        Ok((read as usize).min(buffer.len()))
    }

    /// Calls the method in vtable slot `slot` with the object and three
    /// arguments.
    ///
    /// # Safety
    ///
    /// The method must take those arguments.
    unsafe fn call(&self, slot: usize, first: u32, second: u32, third: u32) -> u32 {
        let this = self.object.as_ptr();
        // SAFETY: the object starts with its vtable address, and the vtable
        // holds the code address of each method.
        let method = unsafe { *((*this) as usize as *const u32).add(slot) };
        // SAFETY: the caller guarantees the argument list.
        unsafe { call_console(this as u32, first, second, third, method) }
    }
}

impl Drop for ItemStream {
    fn drop(&mut self) {
        const RELEASE_SLOT: usize = 0x8 / 4;

        // SAFETY: the release method takes only the object, which is not
        // used after this.
        unsafe { self.call(RELEASE_SLOT, 0, 0, 0) };
    }
}

/// Calls the console function at `function` with r3 to r6. Console callees
/// store their arguments in the caller's frame from sp+0x10 to sp+0x50, so
/// the call runs in a 0x50-byte frame of its own.
///
/// # Safety
///
/// `function` must be the code address of a function that takes at most
/// four 32-bit arguments.
#[unsafe(naked)]
unsafe extern "C" fn call_console(first: u32, second: u32, third: u32, fourth: u32, function: u32) -> u32 {
    core::arch::naked_asm!(
        "stwu 1, -0x50(1)",
        "mflr 0",
        "stw 0, 0x54(1)",
        "mtctr 7",
        "bctrl",
        "lwz 0, 0x54(1)",
        "mtlr 0",
        "addi 1, 1, 0x50",
        "blr",
    )
}
