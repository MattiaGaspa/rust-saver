use super::DatabaseSettings;

pub fn connection_string(database: DatabaseSettings) -> String {
    format!("postgres://{}:{}@{}:{}/{}",
        database.username,
        database.password,
        database.host,
        database.port,
        database.database_name
    )
}