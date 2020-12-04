use factorio_remote::{
    error::Error,
    types::{TrainSchedule, TrainScheduleRecord, WaitCondition, CircuitCondition, WaitConditionType, SignalID, SignalType},
    FactorioRemote,
};


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Connect using environment variables `RCON_ADDRESS` and `RCON_PASSWORD`
    // Alternatively use `RemoteIO::connect(hostname, password)`.
    let mut remote = FactorioRemote::connect_env().await?;


    let mut train_schedule = TrainSchedule::default();

    train_schedule.records.push(TrainScheduleRecord::station(
        "Station A",
        WaitCondition::single(WaitConditionType::ItemCount { condition: CircuitCondition {
            comparator: ">".to_owned(),
            first_signal: SignalID {
                name: "iron-ore".to_string(),
                ty: SignalType::Item,
            },
            second_signal: None,
            constant: Some(2000)
        } }),
        false
    ));
    train_schedule.records.push(TrainScheduleRecord::station(
        "Station B",
        WaitCondition::single(WaitConditionType::Empty),
        false
    ));
    train_schedule.records.push(TrainScheduleRecord::rail(
        -93., -3.,
        WaitCondition::single(WaitConditionType::Time { ticks: 120 }),
        false,
    ));

    println!("Current train schedule {:#?}", remote.get_train_schedule(1).await?);
    println!("Our train schedule: {:#?}", train_schedule);

    remote.set_train_schedule(1, &train_schedule).await?;

    Ok(())
}
