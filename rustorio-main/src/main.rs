#![allow(dead_code)]

mod cheat_sheet;
mod config;
mod materials;
mod time;

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
use rustorio_core::{
    mod_loader::ModLoader,
    prototypes::{
        item::ItemPrototype,
        technology::TechnologyPrototype,
        HasPrototypes,
        InheritsBase,
        Prototypes,
    },
};
use structopt::StructOpt;

use crate::{
    cheat_sheet::CheatSheet,
    config::Config,
};

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
enum Args {
    Export {
        #[structopt(short, long)]
        output: Option<PathBuf>,

        #[structopt(short, long)]
        pretty: bool,
    },
    CheatSheet {
        #[structopt(short, long)]
        output: Option<PathBuf>,

        config: PathBuf,
    },
    ListTechnologies,
    ListItems,
}

impl Args {
    fn run(self) -> Result<(), Error> {
        match self {
            Self::Export { output, pretty } => {
                let mut loader = ModLoader::new_with_base("data/core", "data/base")?;
                loader.check_dependencies()?;

                let prototypes: Prototypes = loader.data_stage()?;

                let output_file = OutputFile::open(output)?;

                if pretty {
                    serde_json::to_writer_pretty(output_file, &prototypes)?;
                }
                else {
                    serde_json::to_writer(output_file, &prototypes)?;
                }
            }
            Self::CheatSheet { output: _, config } => {
                let config: Config = toml::from_str(&std::fs::read_to_string(&config)?)?;

                let mut loader = ModLoader::new_with_base("data/core", "data/base")?;
                loader.check_dependencies()?;
                let prototypes: Prototypes = loader.data_stage()?;

                let cheat_sheet = CheatSheet::generate(&config, &prototypes)?;
                println!("{:#?}", cheat_sheet.research.entries);

                //let output_file = OutputFile::open(output)?;
                //cheat_sheet.write(output_file)?;
            }
            Self::ListTechnologies => {
                let mut loader = ModLoader::new_with_base("data/core", "data/base")?;
                loader.check_dependencies()?;
                let prototypes: Prototypes = loader.data_stage()?;

                let mut technologies =
                    HasPrototypes::<TechnologyPrototype>::iter(&prototypes).collect::<Vec<_>>();
                technologies.sort_by_cached_key(|t| &t.base().order);
                for technology in technologies {
                    println!("{}", technology.base().name);
                }
            }
            Self::ListItems => {
                let mut loader = ModLoader::new_with_base("data/core", "data/base")?;
                loader.check_dependencies()?;
                let prototypes: Prototypes = loader.data_stage()?;

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
