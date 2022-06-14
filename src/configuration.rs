use config::{Config, ConfigError};
use std::{io::Write, path::PathBuf, str::FromStr};

#[derive(Default)]
pub struct Configuration {
    pub username: String,
    pub password: String,
    pub jira_url: String,
    pub query: String,
    pub md_file_path: PathBuf,
}

impl Configuration {
    pub fn build() -> Result<Configuration, ConfigError> {
        let mut conf = Configuration::default();

        let settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()?;

        conf.username = settings.get_string("username")?;
        conf.password = settings.get_string("password")?;

        if conf.password.is_empty() {
            conf.password = ask_for_password();
        }

        conf.jira_url = settings.get_string("jira_url")?;
        conf.query = settings.get_string("query")?;

        let path_str = settings.get_string("md_file_path")?;
        conf.md_file_path = PathBuf::from_str(&path_str).expect("Invalid md file path");

        Ok(conf)
    }
}

fn ask_for_password() -> String {
    print!("Password: ");
    std::io::stdout().flush().unwrap();
    rpassword::read_password().unwrap()
}
