use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommandQuery {
    pub id: String,
}
