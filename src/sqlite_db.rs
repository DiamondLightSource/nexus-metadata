use sqlx::{sqlite::SqliteConnectOptions, FromRow, SqlitePool};

#[derive(Clone, FromRow, Debug)]
pub struct InsertionDevice {
    pub uuid: i64,
    pub poles: i64,
    pub length: f64,
}

#[derive(Clone, FromRow, Debug)]
pub struct Device {
    pub beamline: String,
    pub device_name: String,
    pub uuid: i64,
}

pub struct SqliteService {
    pool: SqlitePool,
}
impl SqliteService {
    pub async fn connect(path: &str) -> Result<Self, sqlx::Error> {
        println!("Connecting to SQLite database");
        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(path);
        let pool = SqlitePool::connect_with(options).await.unwrap();
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }
    pub async fn get_insertion_devices(&self) -> Result<Vec<InsertionDevice>, sqlx::Error> {
        let ins_results = sqlx::query_as::<_, InsertionDevice>(
            "SELECT uuid, poles, length from insertion_device",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(ins_results)
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>, sqlx::Error> {
        let device_results = sqlx::query_as::<_, Device>("SELECT * from devices")
            .fetch_all(&self.pool)
            .await?;
        Ok(device_results)
    }
}
