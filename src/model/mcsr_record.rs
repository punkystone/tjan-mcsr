use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct McsrRecord {
    pub win: usize,
    pub lose: usize,
    pub draw: usize,
}
