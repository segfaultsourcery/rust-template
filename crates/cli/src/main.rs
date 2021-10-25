use color_eyre::Report;
use structopt::StructOpt;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Report> {
    let opt = Opt::from_args();
    setup(&opt)?;

    info!("Hello, world!");

    Ok(())
}

fn setup(opt: &Opt) -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "full")
    }
    color_eyre::install()?;

    if opt.debug {
        std::env::set_var("RUST_LOG", "debug");
    } else if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cli", about = "CLI template.")]
struct Opt {
    /// Debug logging.
    #[structopt(short, long)]
    debug: bool,
}
