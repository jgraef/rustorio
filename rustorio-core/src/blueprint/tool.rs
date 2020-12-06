use std::{
    path::PathBuf,
    fs::read_to_string
};

use rustorio_core::blueprint::Envelope;

use structopt::StructOpt;
use anyhow::Error;

#[derive(Debug, StructOpt)]
enum Opt {
    Show {
        input: PathBuf,
    },
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    match opt {
        Opt::Show { input } => {
            let bp_string = read_to_string(input)?;
            match Envelope::decode(&bp_string)? {
                Envelope::Blueprint(bp) => println!("{:#?}", bp),
                Envelope::BlueprintBook(book) => println!("{:#?}", book),
            }
        }
    }

    Ok(())
}