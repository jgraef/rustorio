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
    let mut remote = FactorioRemote::connect_env().await?;

    println!("{:#?}", remote.get_radar(2).await?);

    Ok(())
}
