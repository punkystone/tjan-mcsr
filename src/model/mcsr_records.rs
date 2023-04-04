use serde::Deserialize;

use super::mcsr_record::McsrRecord;

#[derive(Deserialize, Debug)]
pub struct McsrRecords {
    #[serde(alias = "2")]
    pub second: McsrRecord,
}
