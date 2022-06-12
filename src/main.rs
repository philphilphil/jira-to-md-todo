mod configuration;
mod issue;
use configuration::Configuration;
use issue::Issue;
use reqwest::Client;
use std::{collections::HashMap, fs};

use crate::issue::QueryResponse;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Configuration::build();

    let tickets = get_tickets_from_jira(&config).await?;
    write_md_todos(&config, tickets.issues)?;

    Ok(())
}

async fn get_tickets_from_jira(config: &Configuration) -> Result<QueryResponse, anyhow::Error> {
    let mut map = HashMap::new();
    map.insert("username", &config.username);
    map.insert("password", &config.password);

    let client = Client::builder().cookie_store(true).build()?;

    let _res = client
        .post("http://localhost:8080/rest/auth/1/session")
        .json(&map)
        .send()
        .await?;

    let resp = client.get("http://localhost:8080/rest/api/2/search?jql=assignee%3D%27philipp.baum%27+AND+resolution+%3D+Unresolved").send().await?;
    let text = resp.text().await?;

    Ok(serde_json::from_str(&text)?)
}

fn write_md_todos(conf: &Configuration, tickets: Vec<Issue>) -> anyhow::Result<()> {
    let mut md_todo_lines = String::new();

    for t in tickets.iter() {
        md_todo_lines += &format!(
            "- [ ] {} [{}] [{}] - {}\r\n",
            t.key, t.fields.status.name, t.fields.priority.name, t.fields.summary
        );
    }

    fs::write("test.md", md_todo_lines)?;
    Ok(())
}
