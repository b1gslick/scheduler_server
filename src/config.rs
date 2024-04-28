use clap::Parser;
use std::env;

/// Scheduler web service API
#[derive(Parser, Debug, PartialEq)]
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
        dotenvy::dotenv().ok();
        if env::var("PASETO_KEY").is_err() {
            panic!("PASETO key not set");
        }
        let config = Config::parse();

        let port = env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(8080))
            .map_err(handle_errors::Error::ParseError)?;

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
                .map_err(handle_errors::Error::ParseError)?,
            database_name: db_name,
        })
    }
}
#[cfg(test)]
mod config_tests {
    use super::*;

    fn set_env() {
        env::set_var("PASETO_KEY", "yes");
        env::set_var("DATABASE_USER", "user");
        env::set_var("DATABASE_PASSWORD", "pass");
        env::set_var("DATABASE_DB", "userdb");
        env::set_var("DATABASE_HOST", "localhost");
        env::set_var("DATABASE_PORT", "5432");
    }

    #[test]
    fn small_test_set_paseto_key() {
        set_env();

        let expexted = Config {
            log_level: "warn".to_string(),
            port: 8080,
            database_user: "user".to_string(),
            database_password: "pass".to_string(),
            database_host: "localhost".to_string(),
            database_port: 5432,
            database_name: "userdb".to_string(),
        };
        let config = Config::new().unwrap();
        assert_eq!(config, expexted);
    }
}
