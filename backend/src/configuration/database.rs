use super::DatabaseSettings;

pub fn connection_string(database: DatabaseSettings) -> String {
    format!("postgres://{}:{}@postgres:{}/{}",
        database.username,
        database.password,
        database.port,
        database.database_name
    )
}