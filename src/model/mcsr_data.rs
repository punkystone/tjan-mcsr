use serde::Deserialize;

use super::mcsr_records::McsrRecords;

#[derive(Deserialize, Debug)]
pub struct McsrData {
    pub elo_rate: usize,
    pub elo_rank: usize,
    pub records: McsrRecords,
}
