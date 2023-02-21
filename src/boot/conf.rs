use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub config_path: Option<String>,
    pub server: Server,
    pub postgres: Option<Postgres>,
    pub mysql: Option<Mysql>,
    pub watch_path: String,
    pub jwt: Jwt,
    pub white_list: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u32,
    pub env: Option<String>,
    // pub context_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub dsn: String,
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    pub dsn: String,
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub key: String,
}

impl Conf {
    #[allow(dead_code)]
    pub fn get_env(&self) -> String {
        // self.server.env.clone()
        "".to_string()
    }

    #[allow(dead_code)]
    pub fn addr(&self) -> String {
        format!("0.0.0.0:{}", self.server.port)
    }
}
