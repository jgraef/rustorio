use factorio_remote::{
    error::Error,
    FactorioRemote,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Connect using environment variables `RCON_ADDRESS` and `RCON_PASSWORD`
    // Alternatively use `RemoteIO::connect(hostname, password)`.
    let remote = FactorioRemote::connect_env().await?;

    // Print `Hello World` to the players' consoles.
    remote.print("Hello World!", None).await?;

    Ok(())
}
