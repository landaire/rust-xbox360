use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use clap::Subcommand;
use xbox360_asm_rewrite::audit;
use xbox360_asm_rewrite::reservation;
use xbox360_asm_rewrite::rewrite;

use crate::error::Error;

mod error;

/// Rewrites callee-saved register saves and restores in the assembly of
/// an Xbox 360 executable into calls to shared helper routines.
#[derive(Parser)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Rewrite an assembly file.
    Rewrite {
        input: PathBuf,
        output: PathBuf,
        /// Receives the compiled function list for `audit`.
        #[arg(long)]
        scope: PathBuf,
        /// Receives one line per refused site.
        #[arg(long)]
        refusals: PathBuf,
    },
    /// Check `llvm-objdump -d --no-show-raw-insn` output of the linked
    /// image for sequences the rules still rewrite.
    Audit {
        disassembly: PathBuf,
        #[arg(long)]
        scope: PathBuf,
    },
    /// Check a linked ELF image for reservations the kernel loader reports
    /// as unprotected.
    Reservations { image: PathBuf },
}

fn open(path: &Path) -> Result<BufReader<File>, Error> {
    File::open(path)
        .map(BufReader::new)
        .map_err(|source| Error::Open {
            path: path.to_owned(),
            source,
        })
}

fn create(path: &Path) -> Result<BufWriter<File>, Error> {
    File::create(path)
        .map(BufWriter::new)
        .map_err(|source| Error::Open {
            path: path.to_owned(),
            source,
        })
}

fn run(arguments: Arguments) -> Result<(), Error> {
    match arguments.command {
        Command::Rewrite {
            input,
            output,
            scope,
            refusals,
        } => {
            let listings = rewrite::Listings {
                scope: create(&scope)?,
                refusals: create(&refusals)?,
            };
            let report = rewrite::rewrite(open(&input)?, create(&output)?, listings)?;
            println!("{report}");
        }
        Command::Audit { disassembly, scope } => {
            let scope = audit::read_scope(open(&scope)?)?;
            let report = audit::audit(open(&disassembly)?, &scope)?;
            print!("{report}");

            if !report.passed() {
                return Err(Error::Rewritable {
                    count: report.rewritable.len(),
                });
            }
        }
        Command::Reservations { image } => {
            let found = reservation::scan_elf(&image)?;

            for site in &found {
                println!("unprotected {site}");
            }

            if !found.is_empty() {
                return Err(Error::Unprotected { count: found.len() });
            }
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match run(Arguments::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let mut message = format!("error: {error}");
            let mut source = std::error::Error::source(&error);

            while let Some(cause) = source {
                message.push_str(&format!(": {cause}"));
                source = cause.source();
            }

            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
