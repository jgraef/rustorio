#![allow(dead_code)]

use std::{
    fs::File,
    io::{
        stdout,
        BufWriter,
        Stdout,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};

use color_eyre::eyre::Error;
use rustorio_loader::Loader;
use rustorio_prototype::{
    item::ItemPrototype,
    technology::TechnologyPrototype,
    HasPrototypes,
    InheritsBase,
    Prototypes,
};
use structopt::StructOpt;

pub enum OutputFile {
    File(BufWriter<File>),
    Stdout(Stdout),
}

impl OutputFile {
    pub fn open<P: AsRef<Path>>(path: Option<P>) -> std::io::Result<Self> {
        if let Some(path) = path {
            let path = path.as_ref();
            if path.to_str() == Some("-") {
                Ok(Self::Stdout(stdout()))
            }
            else {
                Ok(Self::File(BufWriter::new(File::create(path)?)))
            }
        }
        else {
            Ok(Self::Stdout(stdout()))
        }
    }
}

impl Write for OutputFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            OutputFile::File(file) => file.write(buf),
            OutputFile::Stdout(stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            OutputFile::File(file) => file.flush(),
            OutputFile::Stdout(stdout) => stdout.flush(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long, env = "FACTORIO_DATA")]
    data_dir: PathBuf,

    #[structopt(long, env = "FACTORIO_MODS")]
    mod_dir: Option<PathBuf>,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    ListTechnologies,
    ListItems,
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

        match self.command {
            Command::ListTechnologies => {
                let mut technologies =
                    HasPrototypes::<TechnologyPrototype>::iter(&prototypes).collect::<Vec<_>>();
                technologies.sort_by_cached_key(|t| &t.base().order);
                for technology in technologies {
                    println!("{}", technology.base().name);
                }
            }
            Command::ListItems => {
                let mut items =
                    HasPrototypes::<ItemPrototype>::iter(&prototypes).collect::<Vec<_>>();
                items.sort_by_cached_key(|t| &t.base().name);
                for item in items {
                    println!("{}", item.base().name);
                }
            }
        }

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
