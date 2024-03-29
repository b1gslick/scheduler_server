use clap::Parser;
use dotenv;
use std::env;

/// Scheduler web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// App port
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// Database user
    #[clap(long, default_value = "user")]
    pub database_user: String,
    /// database db_password
    #[clap(short, long, default_value = "user")]
    pub database_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub database_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub database_port: u16,
    /// Database name
    #[clap(long, default_value = "schedulerdb")]
    pub database_name: String,
}
impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        dotenv::dotenv().ok();
        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO key not set");
        }
        let config = Config::parse();

        let port = env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(8080))
            .map_err(|e| handle_errors::Error::ParseError(e))?;

        let db_user = env::var("DATABASE_USER").unwrap_or(config.database_user.to_owned());
        let db_password = env::var("DATABASE_PASSWORD").unwrap();
        let db_host = env::var("DATABASE_HOST").unwrap_or(config.database_host.to_owned());
        let db_port = env::var("DATABASE_PORT").unwrap_or(config.database_port.to_string());
        let db_name = env::var("DATABASE_DB").unwrap_or(config.database_name.to_owned());
        Ok(Config {
            log_level: config.log_level,
            port,
            database_user: db_user,
            database_password: db_password,
            database_host: db_host,
            database_port: db_port
                .parse::<u16>()
                .map_err(|e| handle_errors::Error::ParseError(e))?,
            database_name: db_name,
        })
    }
}
