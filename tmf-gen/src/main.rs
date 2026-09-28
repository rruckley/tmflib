use clap::Parser;
use log::info;
use std::path::PathBuf;

use tmf_gen::{
    config_for_feature, generate_spec_file, write_output, ModuleConfig, SUPPORTED_MODULES,
};

#[derive(Parser, Debug)]
#[command(about = "Generate tmflib model code from TMF OAS specifications")]
struct Args {
    /// OAS file to load. Required unless `--all` is used.
    #[arg(long, required_unless_present = "all", help = "OAS File to load")]
    file: Option<PathBuf>,

    /// Output folder for the generated module file(s).
    #[arg(long, help = "Output folder")]
    output: PathBuf,

    /// TMF module number, e.g. `tmf628`. Required unless `--all` is used.
    #[arg(long, required_unless_present = "all", help = "TMF Number")]
    tmf: Option<String>,

    /// Generate every configured module using its registered spec file,
    /// writing `<output>/<tmf>.rs` for each. Use with `--output src` to
    /// regenerate tmflib's committed modules.
    #[arg(long, help = "Generate all configured modules")]
    all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    info!("Starting {pkg} : v{ver}");

    let args = Args::parse();

    if args.all {
        for feature in SUPPORTED_MODULES {
            let config = config_for_feature(feature)
                .unwrap_or_else(|| panic!("configured module {feature} has no config"));
            let spec_file = config
                .spec_file
                .as_ref()
                .unwrap_or_else(|| panic!("module {feature} has no spec file"));
            // Specs live in the parent crate's open_api/ folder.
            let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("open_api")
                .join(spec_file);
            info!("Generating {feature} from {}", spec_path.display());
            let code = generate_spec_file(&spec_path, &config)?;
            let path = write_output(&args.output, &config, &code)?;
            info!("Wrote {}", path.display());
        }
        return Ok(());
    }

    let file = args.file.expect("file required without --all");
    let tmf = args.tmf.expect("tmf required without --all");
    info!("Using input: {}", file.display());

    let config = config_for_feature(&tmf).unwrap_or_else(|| ModuleConfig::new(&tmf));
    let code = generate_spec_file(&file, &config)?;
    let path = write_output(&args.output, &config, &code)?;
    info!("Wrote {}", path.display());

    Ok(())
}
