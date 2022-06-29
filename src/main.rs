#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate clap_verbosity_flag;
extern crate printpdf;
extern crate qrcode_generator;
extern crate simple_logger;

mod cli;
mod pdf;
mod resume;
use cli::CLI;
use pdf::Pdf;
use resume::Resume;
use simple_logger::SimpleLogger;
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::fs::{canonicalize, remove_file, rename};
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;

fn main() {
    let opt = CLI::from_args();
    if let Some(level) = opt.log_level() {
        SimpleLogger::new().with_level(level).init().unwrap();
    }
    let filename = opt.filename;
    let output = opt.output;
    let ghostscript = opt.ghostscript;
    std::process::exit(
        match Resume::from_path(filename)
            .and_then(|resume| {
                let pdf = Pdf::new(resume)?;
                pdf.save(&output)
            })
            .and_then(|_| optimize_pdf(&output, ghostscript))
        {
            Ok(()) => {
                info!("Resume generated");
                0
            }

            Err(err) => {
                error!("{}", err);
                1
            }
        },
    )
}

fn find_it<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let exe_name = enhance_exe_name(exe_name.as_ref());
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

#[cfg(not(target_os = "windows"))]
fn enhance_exe_name(exe_name: &Path) -> Cow<Path> {
    exe_name.into()
}

#[cfg(target_os = "windows")]
fn enhance_exe_name(exe_name: &Path) -> Cow<Path> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let raw_input: Vec<_> = exe_name.as_os_str().encode_wide().collect();
    let raw_extension: Vec<_> = OsStr::new(".exe").encode_wide().collect();

    if raw_input.ends_with(&raw_extension) {
        exe_name.into()
    } else {
        let mut with_exe = exe_name.as_os_str().to_owned();
        with_exe.push(".exe");
        PathBuf::from(with_exe).into()
    }
}

fn optimize_pdf(filename: &PathBuf, ghostscript: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    debug!("Looking for ghostscript");
    match ghostscript
        .filter(|gs| gs.is_file())
        .or_else(|| find_it("gs"))
    {
        None => {
            debug!("Ghostscript not found, skipping optimization");
            Ok(())
        }
        Some(path) => {
            let canonicalize_filename = canonicalize(filename).unwrap();
            debug!("Ghostscript found, {:?}", path);
            let mut temp_file = env::temp_dir();
            temp_file.push(canonicalize_filename.file_name().unwrap());
            let temp_file_as_string = temp_file.to_str().unwrap();
            rename(filename, &temp_file_as_string)?;
            debug!(
                "Running Ghostscript using temp file: {}",
                &temp_file_as_string
            );
            let output = Command::new(path)
                .current_dir(env::current_dir().unwrap())
                .arg("-dBATCH")
                .arg("-dNOPAUSE")
                .arg("-sDEVICE=pdfwrite")
                .arg("-dCompatibilityLevel=1.3")
                .arg("-dEncodeColorImages=true")
                .arg(format!(
                    "-sOutputFile={}",
                    canonicalize_filename.to_str().unwrap()
                ))
                .arg(&temp_file_as_string)
                .output()?;
            debug!("Ghostscript {}", output.status);
            remove_file(&temp_file_as_string)?;
            Ok(())
        }
    }
}
