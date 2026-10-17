//! XEX metadata read from a TOML manifest.

mod error;

use std::path::Path;

use bitflags::Flags;
use serde::Deserialize;
use toml::Spanned;
use xenon_types::MediaId;
use xenon_types::TitleId;
use xenon_types::Version;
use xenon_types::VirtualAddress;
use xex2::builder::BoundingPath;
use xex2::builder::EncryptionTarget;
use xex2::opt::ImageFlags;
use xex2::opt::ModuleFlags;

use crate::imports::ExportingModule;
use crate::imports::LIBRARIES;
use crate::layout::page_size;
use crate::manifest::error::Location;
use crate::manifest::error::ManifestErrorKind;
use crate::manifest::error::ManifestKey;
use crate::manifest::error::VersionError;

pub use self::error::ManifestError;

pub const DEFAULT_LOAD_ADDRESS: VirtualAddress = VirtualAddress(0x8200_0000);
pub const DEFAULT_IMPORT_VERSION: Version = Version {
    major: 2,
    minor: 0,
    build: 21173,
    revision: 0,
};
pub const DEFAULT_IMPORT_MIN_VERSION: Version = Version {
    major: 2,
    minor: 0,
    build: 1861,
    revision: 0,
};
/// TLS slots of a title without `tls_slots`, the XDK default. Without a
/// TLS_INFO header the kernel gives a title no slots, and std aborts.
pub const DEFAULT_TITLE_TLS_SLOTS: u32 = 64;

/// XEX metadata. `Default` is the metadata used without a manifest.
#[derive(Debug, PartialEq)]
pub struct Manifest {
    pub title_id: TitleId,
    pub media_id: MediaId,
    pub load_address: VirtualAddress,
    /// Title version, also recorded in the export table.
    pub version: Version,
    pub base_version: Version,
    pub image_flags: ImageFlags,
    pub module_flags: ModuleFlags,
    pub bounding_path: Option<BoundingPath>,
    pub compress: bool,
    pub encryption_target: Option<EncryptionTarget>,
    pub imports: ImportVersions,
    pub exports: Option<ExportSettings>,
    /// Thread-local storage slots recorded in the TLS_INFO header, if any.
    pub tls_slots: Option<u32>,
}

/// Versions recorded in every import library descriptor.
#[derive(Debug, PartialEq)]
pub struct ImportVersions {
    pub version: Version,
    pub min_version: Version,
}

/// Identity of the emitted export table.
#[derive(Debug, PartialEq)]
pub struct ExportSettings {
    pub module: &'static ExportingModule,
    pub min_version: Version,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            title_id: TitleId(0),
            media_id: MediaId(0),
            load_address: DEFAULT_LOAD_ADDRESS,
            version: Version::from(0u32),
            base_version: Version::from(0u32),
            image_flags: ImageFlags::empty(),
            module_flags: ModuleFlags::TITLE,
            bounding_path: None,
            compress: false,
            encryption_target: None,
            imports: ImportVersions {
                version: DEFAULT_IMPORT_VERSION,
                min_version: DEFAULT_IMPORT_MIN_VERSION,
            },
            exports: None,
            tls_slots: Some(DEFAULT_TITLE_TLS_SLOTS),
        }
    }
}

impl Manifest {
    /// Parse and validate manifest `text`; `path` is recorded in errors.
    pub fn parse(path: &Path, text: &str) -> Result<Self, ManifestError> {
        let fail = |kind| ManifestError {
            path: path.to_path_buf(),
            kind,
        };

        let raw: RawManifest = toml::from_str(text).map_err(|error| {
            fail(ManifestErrorKind::Parse {
                location: error.span().map(|span| Location::at(text, span.start)),
                message: error.message().to_string(),
            })
        })?;

        raw.resolve(text).map_err(fail)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    title_id: Option<TitleId>,
    media_id: Option<MediaId>,
    load_address: Option<Spanned<VirtualAddress>>,
    version: Option<Spanned<String>>,
    base_version: Option<Spanned<String>>,
    image_flags: Option<Vec<Spanned<String>>>,
    module_flags: Option<Vec<Spanned<String>>>,
    bounding_path: Option<Spanned<String>>,
    compress: Option<bool>,
    encrypt: Option<RawEncryptionTarget>,
    imports: Option<RawImports>,
    exports: Option<RawExports>,
    tls_slots: Option<u32>,
}

impl RawManifest {
    // Absent keys take the values used without a manifest.
    fn resolve(self, text: &str) -> Result<Manifest, ManifestErrorKind> {
        let defaults = Manifest::default();

        let image_flags = match self.image_flags {
            Some(names) => flags(text, ManifestKey::ImageFlags, names)?,
            None => defaults.image_flags,
        };
        let module_flags = match self.module_flags {
            Some(names) => flags(text, ManifestKey::ModuleFlags, names)?,
            None => defaults.module_flags,
        };

        let load_address = match self.load_address {
            Some(address) => {
                let align = page_size(image_flags);
                if address.get_ref().0 % align != 0 {
                    return Err(ManifestErrorKind::UnalignedLoadAddress {
                        address: address.get_ref().0,
                        align,
                        location: Location::at(text, address.span().start),
                    });
                }

                address.into_inner()
            }
            None => defaults.load_address,
        };

        let bounding_path = self
            .bounding_path
            .map(|value| {
                BoundingPath::new(value.get_ref().as_str()).map_err(|_| {
                    ManifestErrorKind::InvalidBoundingPath {
                        location: Location::at(text, value.span().start),
                        value: value.into_inner(),
                    }
                })
            })
            .transpose()?;

        let imports = match self.imports {
            Some(imports) => ImportVersions {
                version: version(
                    text,
                    ManifestKey::ImportsVersion,
                    imports.version,
                    defaults.imports.version,
                )?,
                min_version: version(
                    text,
                    ManifestKey::ImportsMinVersion,
                    imports.min_version,
                    defaults.imports.min_version,
                )?,
            },
            None => defaults.imports,
        };
        let exports = self
            .exports
            .map(|exports| exports.resolve(text))
            .transpose()?;

        Ok(Manifest {
            title_id: self.title_id.unwrap_or(defaults.title_id),
            media_id: self.media_id.unwrap_or(defaults.media_id),
            load_address,
            version: version(text, ManifestKey::Version, self.version, defaults.version)?,
            base_version: version(
                text,
                ManifestKey::BaseVersion,
                self.base_version,
                defaults.base_version,
            )?,
            image_flags,
            module_flags,
            bounding_path,
            compress: self.compress.unwrap_or(defaults.compress),
            encryption_target: self.encrypt.map(|target| match target {
                RawEncryptionTarget::Devkit => EncryptionTarget::Devkit,
                RawEncryptionTarget::Retail => EncryptionTarget::Retail,
            }),
            imports,
            exports,
            // Absent: titles get the default and other modules no header.
            // Zero: no header, which gives the same zero slots.
            tls_slots: match self.tls_slots {
                Some(0) => None,
                Some(slots) => Some(slots),
                None => module_flags
                    .contains(ModuleFlags::TITLE)
                    .then_some(DEFAULT_TITLE_TLS_SLOTS),
            },
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum RawEncryptionTarget {
    Devkit,
    Retail,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawImports {
    version: Option<Spanned<String>>,
    min_version: Option<Spanned<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExports {
    module: Spanned<String>,
    min_version: Option<Spanned<String>>,
}

impl RawExports {
    fn resolve(self, text: &str) -> Result<ExportSettings, ManifestErrorKind> {
        let module = LIBRARIES
            .iter()
            .find(|module| module.name == self.module.get_ref())
            .ok_or_else(|| ManifestErrorKind::UnknownExportModule {
                location: Location::at(text, self.module.span().start),
                name: self.module.get_ref().clone(),
            })?;
        let min_version = version(
            text,
            ManifestKey::ExportsMinVersion,
            self.min_version,
            DEFAULT_IMPORT_MIN_VERSION,
        )?;

        Ok(ExportSettings {
            module,
            min_version,
        })
    }
}

fn version(
    text: &str,
    key: ManifestKey,
    value: Option<Spanned<String>>,
    default: Version,
) -> Result<Version, ManifestErrorKind> {
    let Some(value) = value else {
        return Ok(default);
    };

    parse_version(value.get_ref()).map_err(|reason| ManifestErrorKind::InvalidVersion {
        key,
        location: Location::at(text, value.span().start),
        value: value.into_inner(),
        reason,
    })
}

/// Parse a version as dotted `major.minor.build.revision`, or a packed `0xHEX`.
fn parse_version(value: &str) -> Result<Version, VersionError> {
    if !value.contains('.') {
        let digits = value
            .strip_prefix("0x")
            .or_else(|| value.strip_prefix("0X"))
            .unwrap_or(value);
        return u32::from_str_radix(digits, 16)
            .map(Version::from)
            .map_err(VersionError::Packed);
    }

    let mut parts = [0u16; 4];
    for (index, field) in value.split('.').enumerate() {
        let part = parts.get_mut(index).ok_or(VersionError::TooManyFields)?;
        *part = field
            .parse::<u16>()
            .map_err(|error| VersionError::Field { index, error })?;
    }

    // Version packs as major:4, minor:4, build:16, revision:8. Reject fields
    // that would overflow their width and silently corrupt neighbors.
    let [major, minor, build, revision] = parts;
    if major > 0xF {
        return Err(VersionError::MajorRange);
    }
    if minor > 0xF {
        return Err(VersionError::MinorRange);
    }
    if revision > 0xFF {
        return Err(VersionError::RevisionRange);
    }

    Ok(Version {
        major,
        minor,
        build,
        revision,
    })
}

/// Union of flag names or packed `0xHEX` values.
fn flags<F>(
    text: &str,
    key: ManifestKey,
    names: Vec<Spanned<String>>,
) -> Result<F, ManifestErrorKind>
where
    F: Flags<Bits = u32>,
{
    names.into_iter().try_fold(F::empty(), |flags, name| {
        let flag = match name.get_ref().strip_prefix("0x") {
            Some(hex) => u32::from_str_radix(hex, 16).ok().map(F::from_bits_retain),
            None => F::from_name(name.get_ref()),
        };
        flag.map(|flag| flags.union(flag))
            .ok_or_else(|| ManifestErrorKind::InvalidFlag {
                key,
                location: Location::at(text, name.span().start),
                name: name.into_inner(),
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> Result<Manifest, ManifestErrorKind> {
        Manifest::parse(Path::new("xex.toml"), text).map_err(|error| error.kind)
    }

    #[test]
    fn empty_manifest_is_the_default_metadata() {
        assert_eq!(parse("").unwrap(), Manifest::default());
    }

    #[test]
    fn every_key_maps_to_its_header_field() {
        let manifest = parse(
            r#"
title_id = 0x48454C4F
media_id = 0x12345678
load_address = 0x91F00000
version = "2.0.21256.0"
base_version = "0x20000000"
image_flags = ["SMALL_PAGES", "0x40000000"]
module_flags = ["TITLE", "DLL"]
bounding_path = '\Device\Harddisk0\Partition1\DEVKIT\FlashEx\xbdm.xex'
compress = true
encrypt = "retail"
tls_slots = 32
[imports]
version = "2.0.17559.0"
min_version = "2.0.1888.0"
[exports]
module = "xbdm.xex"
min_version = "2.0.1861.0"
"#,
        )
        .unwrap();

        assert_eq!(
            manifest,
            Manifest {
                title_id: TitleId(0x4845_4C4F),
                media_id: MediaId(0x1234_5678),
                load_address: VirtualAddress(0x91F0_0000),
                version: Version {
                    major: 2,
                    minor: 0,
                    build: 21256,
                    revision: 0
                },
                base_version: Version {
                    major: 2,
                    minor: 0,
                    build: 0,
                    revision: 0
                },
                image_flags: ImageFlags::SMALL_PAGES | ImageFlags::from_bits_retain(0x4000_0000),
                module_flags: ModuleFlags::TITLE | ModuleFlags::DLL,
                bounding_path: Some(
                    BoundingPath::new(r"\Device\Harddisk0\Partition1\DEVKIT\FlashEx\xbdm.xex")
                        .unwrap()
                ),
                compress: true,
                encryption_target: Some(EncryptionTarget::Retail),
                imports: ImportVersions {
                    version: Version {
                        major: 2,
                        minor: 0,
                        build: 17559,
                        revision: 0
                    },
                    min_version: Version {
                        major: 2,
                        minor: 0,
                        build: 1888,
                        revision: 0
                    },
                },
                exports: Some(ExportSettings {
                    module: &LIBRARIES[2],
                    min_version: DEFAULT_IMPORT_MIN_VERSION
                }),
                tls_slots: Some(32),
            }
        );
    }

    #[test]
    fn tls_slots_default_to_64_for_titles_only() {
        assert_eq!(parse("").unwrap().tls_slots, Some(DEFAULT_TITLE_TLS_SLOTS));
        assert_eq!(parse("module_flags = [\"DLL\"]").unwrap().tls_slots, None);
        assert_eq!(
            parse("module_flags = [\"DLL\"]\ntls_slots = 8")
                .unwrap()
                .tls_slots,
            Some(8)
        );
        assert_eq!(parse("tls_slots = 0").unwrap().tls_slots, None);
    }

    #[test]
    fn small_pages_permit_a_4k_aligned_load_address() {
        let manifest = parse("load_address = 0x82001000\nimage_flags = [\"SMALL_PAGES\"]").unwrap();
        assert_eq!(manifest.load_address, VirtualAddress(0x8200_1000));
    }

    #[test]
    fn invalid_manifests_report_the_offending_value() {
        let at = |line, column| Location { line, column };
        let cases = [
            (
                "load_address = 0x82001000",
                ManifestErrorKind::UnalignedLoadAddress {
                    address: 0x8200_1000,
                    align: 0x1_0000,
                    location: at(1, 16),
                },
            ),
            (
                "module_flags = [\"TITLE\", \"GAME\"]",
                ManifestErrorKind::InvalidFlag {
                    key: ManifestKey::ModuleFlags,
                    name: "GAME".to_string(),
                    location: at(1, 26),
                },
            ),
            (
                "[imports]\nmin_version = \"16.0.0.0\"",
                ManifestErrorKind::InvalidVersion {
                    key: ManifestKey::ImportsMinVersion,
                    value: "16.0.0.0".to_string(),
                    location: at(2, 15),
                    reason: VersionError::MajorRange,
                },
            ),
            (
                "version = \"1.2.3.4.5\"",
                ManifestErrorKind::InvalidVersion {
                    key: ManifestKey::Version,
                    value: "1.2.3.4.5".to_string(),
                    location: at(1, 11),
                    reason: VersionError::TooManyFields,
                },
            ),
            (
                "[exports]\nmodule = \"xboxkrnl.xex\"",
                ManifestErrorKind::UnknownExportModule {
                    name: "xboxkrnl.xex".to_string(),
                    location: at(2, 10),
                },
            ),
            (
                "bounding_path = \"\"",
                ManifestErrorKind::InvalidBoundingPath {
                    value: String::new(),
                    location: at(1, 17),
                },
            ),
        ];
        for (text, expected) in cases {
            assert_eq!(parse(text).unwrap_err(), expected, "{text}");
        }
    }

    #[test]
    fn schema_errors_report_their_location() {
        let cases = [
            ("title_id = 1\ntitle = 2", at(2, 1)),
            (
                "[imports]\nversion = \"2.0.0.0\"\nmax_version = \"2.0.0.0\"",
                at(3, 1),
            ),
            ("encrypt = \"xbox\"", at(1, 11)),
            ("title_id = 0x100000000", at(1, 12)),
            ("[exports]\nmin_version = \"2.0.0.0\"", at(1, 1)),
        ];
        for (text, expected) in cases {
            match parse(text).unwrap_err() {
                ManifestErrorKind::Parse { location, .. } => {
                    assert_eq!(location, Some(expected), "{text}")
                }
                other => panic!("{text}: expected a parse error, got {other:?}"),
            }
        }

        fn at(line: usize, column: usize) -> Location {
            Location { line, column }
        }
    }
}
