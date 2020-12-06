pub mod ir;
pub mod parser;
pub mod compiler;
//pub mod simulator;
pub mod signals;
pub mod blueprint;

use std::{
    path::{PathBuf, Path},
    io::BufWriter,
    fs::OpenOptions,
};

use structopt::StructOpt;
use anyhow::Error;

use rustorio_core::blueprint::Envelope;

use crate::{
    parser::{ParserConfig, Parser},
    ir::Ir,
};
use crate::compiler::Compiler;


#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(name = "compile")]
    Compile {
        /// Paths to look into for imports.
        #[structopt(short="I", long="import")]
        imports: Vec<PathBuf>,

        /// Output intermediate representation to given path.
        #[structopt(short="i", long="ir")]
        ir_out: Option<PathBuf>,

        /// Output blueprint to path. If unspecified, prints the blueprint to STDOUT.
        #[structopt(short="o", long="output")]
        output: Option<PathBuf>,

        /// Which module to use as root.
        #[structopt(short="r", long="root", default_value="main")]
        root: String,

        /// Input source files.
        inputs: Vec<PathBuf>,
    },
}

fn compile_to_ir(inputs: Vec<PathBuf>, mut imports: Vec<PathBuf>, root: String) -> Result<Ir, Error> {
    let mut config = ParserConfig::default();
    config.import_paths.append(&mut imports);

    let mut parser = Parser::new(config);

    for path in &inputs {
        parser.parse_unit_from_file(path)?;
    }

    let ast = parser.into_ast();

    log::debug!("AST: {:#?}", ast);

    let compiler = Compiler::new(&ast);
    let ir = compiler.compile_module(&root.into(), vec![])?;

    log::debug!("IR: {:#?}", ir);

    let blueprint = crate::blueprint::blueprint_from_ir(&ir)?;

    log::debug!("Blueprint: {:#?}", blueprint);
    log::debug!("Blueprint: {}", Envelope::Blueprint(blueprint).encode()?);

    Ok(ir)
}

fn write_ir_to_file<P: AsRef<Path>>(path: P, ir: &Ir) -> Result<(), Error> {
    let file = OpenOptions::new().write(true).truncate(true).open(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, ir)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let opt = Opt::from_args();

    match opt {
        Opt::Compile { imports, ir_out, output, root, inputs } => {
            let ir = compile_to_ir(inputs, imports, root)?;
            if let Some(ir_out) = ir_out {
                write_ir_to_file(ir_out, &ir)?;
            }

            let _ = output;
        }
    }

    Ok(())
}
