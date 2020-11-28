use anyhow::Error;

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    if let Err(e) = run_app() {
        eprintln!("\n{}\n", e);
    }
}

fn run_app() -> Result<(), Error> {
    Ok(())
}
