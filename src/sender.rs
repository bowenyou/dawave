use std::io::Write;

use anyhow::Result;
use borsh::to_vec;
use eigenda_proto::disperser::{disperser_client::DisperserClient, DisperseBlobRequest};
use flate2::{write::GzEncoder, Compression};
use kzgpad_rs::convert_by_padding_empty_byte;

use crate::{constants::TESTNET_DISPERSER_URL, message::DAWaveMessage};

pub fn compress_audio(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

pub async fn send_message(message: DAWaveMessage) -> Result<()> {
    let mut disperser_client = DisperserClient::connect(TESTNET_DISPERSER_URL).await?;

    let padded_data = convert_by_padding_empty_byte(&to_vec(&message)?);
    println!("{:?}", padded_data.len());

    let request = tonic::Request::new(DisperseBlobRequest {
        data: padded_data,
        custom_quorum_numbers: vec![],
        account_id: "".into(),
    });

    let response = disperser_client.disperse_blob(request).await?;

    let request_id = response.into_inner().request_id;
    println!("Sent message with request_id: {:?}", request_id);

    Ok(())
}
