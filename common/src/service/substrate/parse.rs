use subxt::ext::{
    frame_metadata::ExtrinsicMetadata,
    scale_value::scale::decode_as_type,
    sp_core::Decode,
    sp_runtime::scale_info::{form::PortableForm, PortableRegistry},
};

use serde_json::to_value;

use super::error::Result;

pub (super) fn parse_transaction(
    transaction: &[u8],
    mut extrinsic_metadata: &[u8],
    mut extrinsic_types: &[u8],
) -> Result<String> {
    let mut transaction = transaction.split_at(1).1;
    let extrinsic_metadata: ExtrinsicMetadata<PortableForm> =
        Decode::decode(&mut extrinsic_metadata)?;
    let types: PortableRegistry = Decode::decode(&mut extrinsic_types)?;
    let value = decode_as_type(&mut transaction, extrinsic_metadata.ty, &types)?;
    let json = to_value(value)?.to_string();
    Ok(json)
}