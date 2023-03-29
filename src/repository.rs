use hyper::{body::to_bytes, Client, Uri};
use hyper_tls::HttpsConnector;

use crate::{errors::get_rank_error::GetRankError, model::mcsr_response::McsrResponse};

pub async fn get_rank() -> Result<String, GetRankError> {
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let response = client
        .get("https://mcsrranked.com/api/users/tjan".parse::<Uri>()?)
        .await?;

    let response_string = String::from_utf8(to_bytes(response.into_body()).await?.to_vec())?;

    let parsed_response = serde_json::from_str::<McsrResponse>(&response_string)?;
    println!("{parsed_response:?}");
    if let Some(data) = parsed_response.data {
        Ok(format!(
            "MCSR Ranked Rang: #{} (Elo: {}) | W:{} / L:{} / D:{}",
            data.elo_rank,
            data.elo_rate,
            data.records.second.win,
            data.records.second.lose,
            data.records.second.draw,
        ))
    } else {
        Ok("Player not found".to_owned())
    }
}
