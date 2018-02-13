use std::fs::File;
use std::io::Read;
use iron::typemap::Key;
use toml;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub db: String,
}

#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub name: String,
    pub posts_per_page: u64,
    pub threads_per_page: u64,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub admin_pwd: String,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub site: SiteConfig,
    pub database: DbConfig,
    pub server: ServerConfig,
}

pub fn parse(fname: &str) -> Config {
    let mut file = String::new();
    File::open(fname).and_then(|mut f| f.read_to_string(&mut file)).unwrap();
    let decoded: Config = toml::from_str(&file).unwrap();
    decoded
}

impl Key for Config {
    type Value = Config;
}
