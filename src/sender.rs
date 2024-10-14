use anyhow::Result;
use borsh::to_vec;
use eigenda_proto::disperser::{disperser_client::DisperserClient, DisperseBlobRequest};
use kzgpad_rs::convert_by_padding_empty_byte;

use crate::{constants::TESTNET_DISPERSER_URL, message::DAWaveMessage};

pub async fn send_message(message: DAWaveMessage) -> Result<()> {
    let mut disperser_client = DisperserClient::connect(TESTNET_DISPERSER_URL).await?;

    let padded_data = convert_by_padding_empty_byte(&to_vec(&message)?);

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
