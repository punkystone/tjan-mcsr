use serde::Deserialize;

use super::mcsr_records::McsrRecords;

#[derive(Deserialize, Debug)]
pub struct McsrData {
    pub elo_rate: isize,
    pub elo_rank: Option<isize>,
    pub records: McsrRecords,
}
