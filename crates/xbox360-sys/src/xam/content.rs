use super::CreationDisposition;

mod error;

pub use self::error::ContentDataError;

/// A storage device id, as the device selector reports it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentDevice(u32);

impl ContentDevice {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub const fn id(self) -> u32 {
        self.0
    }
}

value_enum! {
    /// Kind of a content package (`XCONTENTTYPE_*`), with Xenia's values.
    #[non_exhaustive]
    pub enum ContentType: u32 {
        SavedGame = 0x0000_0001,
        MarketplaceContent = 0x0000_0002,
        Publisher = 0x0000_0003,
        Xbox360Title = 0x0000_1000,
        IptvPauseBuffer = 0x0000_2000,
        XnaCommunity = 0x0000_3000,
        InstalledGame = 0x0000_4000,
        XboxTitle = 0x0000_5000,
        SocialTitle = 0x0000_6000,
        GamesOnDemand = 0x0000_7000,
        StoragePack = 0x0000_8000,
        AvatarItem = 0x0000_9000,
        Profile = 0x0001_0000,
        GamerPicture = 0x0002_0000,
        Theme = 0x0003_0000,
        CacheFile = 0x0004_0000,
        StorageDownload = 0x0005_0000,
        XboxSavedGame = 0x0006_0000,
        XboxDownload = 0x0007_0000,
        GameDemo = 0x0008_0000,
        Video = 0x0009_0000,
        GameTitle = 0x000a_0000,
        Installer = 0x000b_0000,
        GameTrailer = 0x000c_0000,
        ArcadeTitle = 0x000d_0000,
        Xna = 0x000e_0000,
        LicenseStore = 0x000f_0000,
        Movie = 0x0010_0000,
        Tv = 0x0020_0000,
        MusicVideo = 0x0030_0000,
        GameVideo = 0x0040_0000,
        PodcastVideo = 0x0050_0000,
        ViralVideo = 0x0060_0000,
        CommunityGame = 0x0200_0000,
    }
}

/// Length of the display name, in UTF-16 units including the NUL.
pub const DISPLAY_NAME_UNITS: usize = 128;
/// Length of the file name in bytes. A name of this length has no NUL.
pub const FILE_NAME_BYTES: usize = 42;

/// `XCONTENT_DATA`: the device, kind and names of a package.
#[repr(C)]
#[derive(Clone)]
pub struct ContentData {
    device: ContentDevice,
    content_type: ContentType,
    display_name: [u16; DISPLAY_NAME_UNITS],
    file_name: [u8; FILE_NAME_BYTES],
    padding: [u8; 2],
}

const _: () = assert!(core::mem::size_of::<ContentData>() == 0x134);

impl ContentData {
    pub fn new(
        device: ContentDevice,
        content_type: ContentType,
        display_name: &str,
        file_name: &[u8],
    ) -> Result<Self, ContentDataError> {
        if file_name.is_empty() || file_name.len() > FILE_NAME_BYTES {
            return Err(ContentDataError::FileNameLength {
                bytes: file_name.len(),
            });
        }

        if file_name.contains(&0) {
            return Err(ContentDataError::FileNameNul);
        }

        let units = display_name.encode_utf16().count();
        if units >= DISPLAY_NAME_UNITS {
            return Err(ContentDataError::DisplayNameTooLong { units });
        }

        let mut data = Self {
            device,
            content_type,
            display_name: [0; DISPLAY_NAME_UNITS],
            file_name: [0; FILE_NAME_BYTES],
            padding: [0; 2],
        };

        for (slot, unit) in data
            .display_name
            .iter_mut()
            .zip(display_name.encode_utf16())
        {
            *slot = unit;
        }

        data.file_name[..file_name.len()].copy_from_slice(file_name);

        Ok(data)
    }

    pub const fn device(&self) -> ContentDevice {
        self.device
    }

    pub const fn content_type(&self) -> ContentType {
        self.content_type
    }
}

/// `XamContentCreate` flags. The low four bits are the
/// [`CreationDisposition`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentCreateFlags(u32);

impl ContentCreateFlags {
    pub const fn new(disposition: CreationDisposition) -> Self {
        Self(disposition as u32)
    }

    /// `bits` including bits without a name, for values passed on unchanged.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// What `XamContentCreate` did, as XAM reports it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Disposition(u32);

/// Dispositions this crate names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DispositionKind {
    Created,
    Opened,
}

impl Disposition {
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// `None` for a value this crate does not name.
    pub const fn kind(self) -> Option<DispositionKind> {
        match self.0 {
            1 => Some(DispositionKind::Created),
            2 => Some(DispositionKind::Opened),
            _ => None,
        }
    }
}
