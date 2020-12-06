use factorio_remote::{
    error::Error,
    types::{SignalType, Signal},
    FactorioRemote,
};


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Connect using environment variables `RCON_ADDRESS` and `RCON_PASSWORD`
    // Alternatively use `RemoteIO::connect(hostname, password)`.
    let remote = FactorioRemote::connect_env().await?;

    // Send some signals on channel 1
    remote.send_signals(1, &[
        Signal::new("iron-plate".to_owned(), SignalType::Item, 40),
        Signal::new("signal-A".to_owned(), SignalType::Virtual, 1337),
    ]).await?;

    Ok(())
}
