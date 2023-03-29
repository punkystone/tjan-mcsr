use actix_web::{get, HttpResponse};

use crate::{errors::get_rank_error::GetRankError, repository::get_rank};

#[get("/command")]
async fn command() -> Result<HttpResponse, GetRankError> {
    Ok(HttpResponse::Ok().body(get_rank().await?))
}
