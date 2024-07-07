use std::{error::Error, io};

use clap::{Args, Parser, Subcommand};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_trunk::TrunkPreprocessor;
use semver::{Version, VersionReq};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Supports(SupportsArgs),
}

#[derive(Args)]
struct SupportsArgs {
    renderer: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let preprocessor = TrunkPreprocessor::new();

    match &cli.command {
        Some(subcommand) => match subcommand {
            Commands::Supports(args) => handle_supports(&preprocessor, args),
        },
        None => handle_preprocessing(&preprocessor),
    }
}

fn handle_supports(
    preprocessor: &dyn Preprocessor,
    SupportsArgs { renderer }: &SupportsArgs,
) -> Result<(), Box<dyn Error>> {
    match preprocessor.supports_renderer(renderer) {
        true => Ok(()),
        false => Err(format!("Renderer `{renderer}` is not supported.").into()),
    }
}

fn handle_preprocessing(preprocessor: &dyn Preprocessor) -> Result<(), Box<dyn Error>> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, but we're being called from version {}",
            preprocessor.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = preprocessor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
