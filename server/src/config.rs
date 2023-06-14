use serde::{Deserialize, Serialize};
use config::{ConfigError, File, FileFormat, Config as c};
use std::{collections::HashMap, net::SocketAddr, net::IpAddr, net::Ipv4Addr};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub database_url: String,
    pub currencies: HashMap<String, String>,
    pub currency_api_key: String,
    pub listen_addr: SocketAddr,
    pub expiration_time: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            database_url: (String::default()),
            currencies: HashMap::default(), 
            currency_api_key: String::default(), 
            listen_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080),
            expiration_time: 10,
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let config_file = &"/etc/wtb/config.toml";
        let builder = c::builder()
        .add_source(File::new(config_file, FileFormat::Toml));
        let config = builder.build()?;
        config.cache.try_deserialize()
    }
}