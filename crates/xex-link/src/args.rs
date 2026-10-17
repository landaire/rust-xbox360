use std::path::Path;

use rootcause::IntoReport;

use crate::error::Result;
use crate::error::XexLinkError;
use crate::error::io;
use crate::manifest::Manifest;

pub const USAGE: &str = "\
usage: xex-link <input.elf> <output.xex> [--manifest PATH]

options:
  --manifest PATH         TOML file of XEX metadata; every key is optional

manifest keys:
  title_id = INT          title id (default 0x00000000)
  media_id = INT          media id (default 0x00000000)
  load_address = INT      image base the ELF was linked at (default 0x82000000)
  version = \"VER\"         title version and export table version, dotted
                          a.b.c.d or \"0xHEX\" (default \"0.0.0.0\")
  base_version = \"VER\"    minimum loader version, same format (default \"0.0.0.0\")
  image_flags = [\"FLAG\"]  ImageFlags names or \"0xHEX\" (e.g. SMALL_PAGES; default [])
  module_flags = [\"FLAG\"] ModuleFlags names or \"0xHEX\" (default [\"TITLE\"])
  bounding_path = \"PATH\"  device path the loaded file must resolve to
  compress = BOOL         LZX-compress the image (Normal, 64 KB window; default false)
  encrypt = \"TARGET\"      encrypt payload with the \"devkit\" or \"retail\" master key
  tls_slots = INT         TLS slots in the TLS_INFO header; 0 omits it
                          (default 64 for titles, none for other modules)
  [imports]
  version = \"VER\"         imported module version (default \"2.0.21173.0\")
  min_version = \"VER\"     minimum imported module version (default \"2.0.1861.0\")
  [exports]               emit an export table
  module = \"NAME\"         module id to record: xboxkrnl.exe, xam.xex, xbdm.xex
  min_version = \"VER\"     oldest compatible version recorded in the export
                          table (default \"2.0.1861.0\")";

pub struct Config {
    pub input: String,
    pub output: String,
    pub manifest: Manifest,
}

impl Config {
    /// Parse the process arguments. `--help` prints usage and exits.
    pub fn from_args() -> Result<Self> {
        let mut input = None;
        let mut output = None;
        let mut manifest_path = None;

        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--manifest" => {
                    let path = args
                        .next()
                        .ok_or_else(|| XexLinkError::MissingValue(arg.clone()).into_report())?;
                    manifest_path = Some(path);
                }
                "-h" | "--help" => {
                    println!("{USAGE}");
                    std::process::exit(0);
                }
                _ if arg.starts_with('-') => {
                    return Err(XexLinkError::UnexpectedArg(arg).into_report());
                }
                _ if input.is_none() => input = Some(arg),
                _ if output.is_none() => output = Some(arg),
                _ => return Err(XexLinkError::UnexpectedArg(arg).into_report()),
            }
        }

        let (Some(input), Some(output)) = (input, output) else {
            return Err(XexLinkError::MissingPaths.into_report());
        };

        let manifest = match manifest_path {
            Some(path) => {
                let text = io(&path, std::fs::read_to_string(&path))?;
                Manifest::parse(Path::new(&path), &text)
                    .map_err(|e| XexLinkError::Manifest(e).into_report())?
            }
            None => Manifest::default(),
        };

        Ok(Self {
            input,
            output,
            manifest,
        })
    }
}
