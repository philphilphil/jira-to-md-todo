use config::Config;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

#[derive(Default)]
pub struct Configuration {
    pub username: String,
    pub password: String,
    pub jira_url: String,
    pub query: String,
    pub md_file_path: PathBuf,
}

impl Configuration {
    pub fn build() -> Configuration {
        let mut conf = Configuration::default();

        let settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()
            .unwrap()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();

        conf.username = settings
            .get("username")
            .expect("Username missing.")
            .to_string();
        conf.password = settings
            .get("password")
            .expect("Password missing.")
            .to_string();
        conf.jira_url = settings
            .get("jira_url")
            .expect("Jira Url missing.")
            .to_string();
        conf.query = settings.get("query").expect("Query missing.").to_string();

        let path_str = settings
            .get("md_file_path")
            .expect("Md file path missing.")
            .to_string();
        conf.md_file_path = PathBuf::from_str(&path_str).expect("Invalid md file path");

        conf
    }
}
