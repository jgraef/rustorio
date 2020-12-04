use factorio_remote::{
    error::Error,
    types::ChannelId,
    FactorioRemote,
};


async fn print_channel_signals(remote: &FactorioRemote, channel: ChannelId) -> Result<(), Error> {
    // Receive signals on channel
    let signals = remote.receive_signals(channel).await?;

    if signals.is_empty() {
        println!("Received no signals on channel #{}", channel);
    }
    else {
        println!("Received {} signals on channel #{}:", signals.len(), channel);
        for signal in signals {
            println!(" - type={:?}, name={}, count={}", signal.ty, signal.name, signal.count);
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Connect using environment variables `RCON_ADDRESS` and `RCON_PASSWORD`
    // Alternatively use `RemoteIO::connect(hostname, password)`.
    let mut remote = FactorioRemote::connect_env().await?;

    // Print signals on channel 1 and 2. Note that we don't get the signals back that we sent, but only the signals
    // that are send by senders (TODO: entity name?).
    print_channel_signals(&mut remote, 1).await?;
    //print_channel_signals(&mut remote, 2).await?;


    Ok(())
}
