use std::time::Duration;

use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use alloy_rpc_types_eth::Filter;
use alloy_sol_macro::sol;
use alloy_sol_types::SolEvent;
use anyhow::Result;
use borsh::from_slice;
use eigenda_proto::disperser::{disperser_client::DisperserClient, RetrieveBlobRequest};
use futures::StreamExt;
use kzgpad_rs::remove_empty_byte_from_padded_bytes;

use crate::{
    constants::{TESTNET_DISPERSER_URL, TESTNET_EIGENDA_ADDRESS},
    message::DAWaveMessage,
};

sol!(
    contract IEigenDAServiceManager {
        #[derive(Debug)]
        event BatchConfirmed(bytes32 indexed batch_header_hash, uint32 batch_id);
    }
);

pub async fn listen_to_blobs(url: &str, channel: &str) -> Result<()> {
    let mut disperser_client = DisperserClient::connect(TESTNET_DISPERSER_URL).await?;

    let ws = WsConnect::new(url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    println!("Connected");
    let filter = Filter::new()
        .address(TESTNET_EIGENDA_ADDRESS)
        .event_signature(IEigenDAServiceManager::BatchConfirmed::SIGNATURE_HASH);
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        let IEigenDAServiceManager::BatchConfirmed {
            batch_header_hash,
            batch_id,
        } = log.log_decode()?.inner.data;

        let height = log.block_number.expect("should always have a block number");

        let mut blob_index = 1_u32;

        // blob isn't immediately available after the event is emitted
        tokio::time::sleep(Duration::from_secs(10)).await;

        loop {
            let request = tonic::Request::new(RetrieveBlobRequest {
                batch_header_hash: batch_header_hash.to_vec(),
                blob_index,
            });

            let response = disperser_client.retrieve_blob(request).await;
            if let Ok(r) = response {
                let unpadded_data = remove_empty_byte_from_padded_bytes(&r.into_inner().data);
                if let Ok(message) = from_slice::<DAWaveMessage>(&unpadded_data) {
                    if message.channel.eq(channel) {
                        println!(
                            "Message received in channel {:?}: {:?}",
                            message.channel, message.message
                        )
                    }
                }
                blob_index += 1;
            } else {
                break;
            }
        }

        println!(
            "new batch processed: batch_header_hash: {:?}, batch_id: {:?} at height: {:?}",
            batch_header_hash, batch_id, height
        );
    }

    Ok(())
}
