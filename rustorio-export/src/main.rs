mod export;

use std::path::PathBuf;

use color_eyre::eyre::Error;
use rustorio_loader::Loader;
use rustorio_prototype::Prototypes;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long, env = "FACTORIO_DATA")]
    data_dir: PathBuf,

    #[structopt(long, env = "FACTORIO_MODS")]
    mod_dir: Option<PathBuf>,

    #[structopt(short, long)]
    output: PathBuf,

    #[structopt(short, long)]
    pretty: bool,
}

impl Args {
    fn run(self) -> Result<(), Error> {
        let loader = if let Some(mod_dir) = &self.mod_dir {
            Loader::modded(&self.data_dir, mod_dir)?
        }
        else {
            Loader::vanilla(&self.data_dir)?
        };
        let prototypes: Prototypes = loader.data_stage()?;

        export::export(&self.output, self.pretty, &loader, &prototypes)?;

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    color_eyre::install()?;

    let args = Args::from_args();
    args.run()?;

    Ok(())
}
