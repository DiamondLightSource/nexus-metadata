use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Clone, FromRow, Debug)]
struct InsertionDevice {
    uuid: i64,
    poles: i64,
    length: f64,
}

#[derive(Clone, FromRow, Debug)]
struct Device {
    beamline: String,
    device_name: String,
    uuid: i64,
}

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Created db"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let create_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&create_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

    let ins_results =
        sqlx::query_as::<_, InsertionDevice>("SELECT uuid, poles, length from insertion_device")
            .fetch_all(&db)
            .await
            .unwrap();

    for ins in ins_results {
        println!(
            "uuid: {}, poles: {}, length: {}",
            ins.uuid, ins.poles, ins.length
        );
    }

    let device_results = sqlx::query_as::<_, Device>("SELECT * from devices")
        .fetch_all(&db)
        .await
        .unwrap();

    for device in device_results {
        println!(
            "beamline: {}, device_name: {}, uuid: {}",
            device.beamline, device.device_name, device.uuid
        );
    }
}
