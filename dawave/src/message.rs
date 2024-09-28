use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DAWaveMessage {
    pub channel: String,
    pub message: String,
}
