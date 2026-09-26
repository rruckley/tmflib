use clap::Parser;
use log::info;
use std::path::PathBuf;

use tmf_gen::{generate_spec_file, write_output, ModuleConfig};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, help = "OAS File to load")]
    file: PathBuf,

    #[arg(long, help = "Output folder")]
    output: PathBuf,

    #[arg(long, help = "TMF Number")]
    tmf: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    info!("Starting {pkg} : v{ver}");

    let args = Args::parse();
    info!("Using input: {}", args.file.display());

    let config = ModuleConfig::new(&args.tmf);
    let code = generate_spec_file(&args.file, &config)?;
    let path = write_output(&args.output, &config, &code)?;
    info!("Wrote {}", path.display());

    Ok(())
}
