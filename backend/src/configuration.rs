#[derive(serde::Deserialize, Clone, PartialEq)]
pub struct Config {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone, PartialEq)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Clone, PartialEq)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl ServerSettings {
    pub fn connect(&self) -> (String, u16) {
        (self.host.to_owned(), self.port)
    }

    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }    
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@postgres:{}/{}",
            self.username,
            self.password,
            self.port,
            self.database_name
        )
    }
}

pub fn get_config(filename: String) -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to read current directory.");
    let configuration = base_path.join(filename);

    let config = config::Config::builder()
        .add_source(config::File::from(configuration))
        .build()?;
    config.try_deserialize::<Config>()
}
