use std::collections::HashMap;

use actix_web::{
    get,
    web::{Data, Query},
    HttpRequest, HttpResponse,
};

use crate::{
    errors::get_rank_error::GetRankError, model::command_query::CommandQuery, repository::get_rank,
};

#[get("/command")]
async fn command(
    req: HttpRequest,
    config: Data<HashMap<String, String>>,
) -> Result<HttpResponse, GetRankError> {
    if let Ok(params) = Query::<CommandQuery>::from_query(req.query_string()) {
        if params.id.is_empty() {
            return Ok(HttpResponse::BadRequest().finish());
        }
        match config.get(&params.id) {
            Some(name) => Ok(HttpResponse::Ok().body(get_rank(name).await?)),
            None => Ok(HttpResponse::BadRequest().finish()),
        }
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}
