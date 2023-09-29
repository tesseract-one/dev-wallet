use scale_info::{form::PortableForm, PortableRegistry};
use subxt::ext::{
    codec::Decode,
    frame_metadata::v14::ExtrinsicMetadata,
    scale_value::scale::decode_as_type,
};

use serde::Serialize;
use serde_json::{to_value, Value};

use super::error::Result;

#[derive(Serialize)]
struct TxInfo {
    call: Value,
    extra: Vec<Value>,
    additional: Vec<Value>
}

pub (super) fn parse_transaction(
    transaction: &[u8],
    mut extrinsic_metadata: &[u8],
    mut extrinsic_types: &[u8],
) -> Result<String> {
    let mut transaction = transaction;
    let extrinsic_metadata: ExtrinsicMetadata<PortableForm> =
        Decode::decode(&mut extrinsic_metadata)?;
    let types: PortableRegistry = Decode::decode(&mut extrinsic_types)?;
    let call = decode_as_type(&mut transaction, extrinsic_metadata.ty.id, &types)?;
    let json_call = to_value(call)?;
    let extra_res: Result<Vec<Value>> = extrinsic_metadata.signed_extensions.iter().map(|ext| {
        decode_as_type(&mut transaction, ext.ty.id, &types)
            .map_err(|err| err.into())
            .and_then(|v| to_value(v).map_err(|err| err.into()))
    }).collect();
    let extra = extra_res?;
    let additional_res: Result<Vec<Value>> = extrinsic_metadata.signed_extensions.iter().map(|ext| {
        decode_as_type(&mut transaction, ext.additional_signed.id, &types)
            .map_err(|err| err.into())
            .and_then(|v| to_value(v).map_err(|err| err.into()))
    }).collect();
    let additional = additional_res?;
    let info = TxInfo { call: json_call, extra, additional };
    let json = serde_json::to_string(&info)?;
    Ok(json)
}