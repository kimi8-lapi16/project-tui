use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub number: i64,
    pub status: Option<String>,
    pub repository: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketDetail {
    pub id: String,
    pub title: String,
    pub number: i64,
    pub body: Option<String>,
    pub status: Option<String>,
    pub repository: Option<String>,
    pub state: String,
    pub url: String,
}
