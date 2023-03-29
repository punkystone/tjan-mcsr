use serde::Deserialize;

use super::mcsr_data::McsrData;

#[derive(Deserialize, Debug)]
pub struct McsrResponse {
    pub status: String,
    pub data: Option<McsrData>,
}
