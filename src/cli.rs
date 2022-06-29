use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Resume generator")]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    /// Resume to generate, (Json or Toml)
    pub filename: PathBuf,
    #[structopt(parse(from_os_str), default_value = "resume.pdf")]
    /// Generated resume
    pub output: PathBuf,
    #[structopt(parse(from_os_str), long = "gs", name = "path")]
    /// Ghostscript executable
    pub ghostscript: Option<PathBuf>,
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl Cli {
    pub fn log_level(&self) -> Option<log::LevelFilter> {
        self.verbose.log_level().map(|l| l.to_level_filter())
    }
}
