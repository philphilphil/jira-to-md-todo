use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueryResponse {
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    pub total: u64,
    pub issues: Vec<Issue>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Issue {
    #[serde(rename = "self")]
    pub self_link: String,
    pub key: String,
    pub id: String,
    pub fields: Field,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Field {
    pub summary: String,
    pub priority: Priority,
    pub status: Status,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Priority {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    pub name: String,
}
