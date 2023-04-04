use std::{collections::HashMap, env::current_dir, fs::read_to_string};

use hyper::{body::to_bytes, Client, Uri};
use hyper_tls::HttpsConnector;

use crate::{
    errors::{get_config_error::GetConfigError, get_rank_error::GetRankError},
    model::mcsr_response::McsrResponse,
};

pub async fn get_rank(user: &String) -> Result<String, GetRankError> {
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let response = client
        .get(format!("https://mcsrranked.com/api/users/{user}").parse::<Uri>()?)
        .await?;

    let response_string = String::from_utf8(to_bytes(response.into_body()).await?.to_vec())?;
    let parsed_response = serde_json::from_str::<McsrResponse>(&response_string)?;
    if let Some(data) = parsed_response.data {
        Ok(format!(
            "MCSR Ranked Rang: #{} (Elo: {}) | W:{} / L:{} / D:{}",
            data.elo_rank.unwrap_or(0),
            data.elo_rate,
            data.records.second.win,
            data.records.second.lose,
            data.records.second.draw,
        ))
    } else {
        Ok("Player not found".to_owned())
    }
}

pub fn get_config() -> Result<HashMap<String, String>, GetConfigError> {
    let current_directory = current_dir()?;
    let content = read_to_string(current_directory.join("config.json"))?;
    Ok(serde_json::from_str(&content)?)
}
