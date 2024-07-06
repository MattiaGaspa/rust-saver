use crate::ServerSettings;

pub fn connect(server: ServerSettings) -> (String, u16) {
    (server.host, server.port)
}

pub fn connection_string(server: ServerSettings) -> String {
    format!("{}:{}", server.host, server.port)
}
