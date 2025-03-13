use crate::settings::get_settings;
use refinery::config::{Config, ConfigDbType};
use std::str::FromStr;
use url::Url;

mod settings;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = get_settings()?;
    
    let postgres_url = Url::from_str(settings.database.connection_string.as_str()).unwrap();
    
    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_user(postgres_url.username())
        .set_db_pass(postgres_url.password().unwrap())
        .set_db_host(postgres_url.host_str().unwrap())
        .set_db_name(postgres_url.path().trim_start_matches("/"))
        .set_db_port(postgres_url.port().unwrap_or(5432).to_string().as_str());
    println!("Running migrations");
    embedded::migrations::runner().run(&mut conn)?;
    Ok(())
}