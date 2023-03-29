use crate::errors::get_rank_error::GetRankError;

pub async fn get_rank() -> Result<String, GetRankError> {
    Ok(String::from("test"))
}
