use factorio_remote::{
    error::Error,
    FactorioRemote,
};


const BLUEPRINT_DATA: &'static str = "0eNqdmdFuozAQRf/Fz7TC9owN/MqqWiWttUJKnAjoaqOIfy9pXlZaZjVz3yByDsP4Dvcm3N3x9Fmu01gXN9zd+H6psxt+3N08/qqH0+Oz5XYtbnDjUs6ucfVwfpwt06HO18u0vBzLaXFr48b6Uf64wa9vjSt1GZexPEnfJ7ef9fN8LNO2QGI07nqZt69d6uOqG+olpVdu3G07CtvRdo2PcSrvzxVhbf5BBwOabehoQJMNTQZ0lNG0g2YD2tuqTgZ0a0NnPZp7G7ozoLMN3RvQRl371sBmm0S8YR75P8pOe2zDQHI0sg0TyUZte8NIslHc3jCT1Bv30jCUZJS3N0wlJWPdhrEkNurEMJcUjH5jmEuKRrZhLsmo7xAgD96mf92jRYjGAo0gEyeNFgJDLq5jJ8jGWTMfIUNsXd0dlBGk3eshmqCs2EIhQ3Xf0UMpQ8cOUMwQehojRJN6SlCWII1OI0Ns1QzEBGUJqacZokk97aAsItXWQzShNmqhvKHaE/IQW6UlClDe0NUdIbauboIygaAFYogmaSFBWUj1zKMM5RUdu4MSRhC60EO0uE/jFsoUUfXz3UPsoGIHyKmFnnKEaFJPCfL9oNESM+T7OnaCnFrqaYZoUk87yPEkWg/RhDtNLeR4Qm3JQzSptgD9F6DSS4qQf6qeHYkgtq5uhvxTV3eC2KpnXsqQm0o66yA31XW4hyoVNJxbyJt3Kn1rnu8Bhr9eGzTud5nm7wU5Rx98ih35df0CF2TXrQ==";


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Connect using environment variables `RCON_ADDRESS` and `RCON_PASSWORD`
    // Alternatively use `RemoteIO::connect(hostname, password)`.
    let mut remote = FactorioRemote::connect_env().await?;

    // Import the blueprint
    remote.import_blueprint(1, BLUEPRINT_DATA).await?;

    Ok(())
}
