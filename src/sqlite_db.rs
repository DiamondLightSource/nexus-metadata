use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

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
    pub async fn connect(filename: &str) -> Result<Self, sqlx::Error> {
        if !Sqlite::database_exists(filename).await.unwrap_or(false) {
            println!("Creating database {}", filename);
            match Sqlite::create_database(filename).await {
                Ok(_) => println!("Created db"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }

        let pool = SqlitePool::connect(filename).await.unwrap();
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
