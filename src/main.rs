mod sqlite_db;

const DB_PATH: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() {
    //let args = Cli::init();

    //match args.command {}

    let db_service = sqlite_db::SqliteService::connect(DB_PATH).await.unwrap();

    // get the insertion devices from the db
    let insertion_devices = sqlite_db::SqliteService::get_insertion_devices(&db_service)
        .await
        .unwrap();
    for device in insertion_devices {
        println!(
            "uuid: {}, poles: {}, length: {}",
            device.uuid, device.poles, device.length
        );
    }

    // Get all of the devices.
    let device_results = sqlite_db::SqliteService::get_devices(&db_service)
        .await
        .unwrap();
    for device in device_results {
        println!(
            "beamline: {}, device_name: {}, uuid: {}",
            device.beamline, device.device_name, device.uuid
        );
    }
}
