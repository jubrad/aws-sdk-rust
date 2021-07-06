// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_payload_put_snapshot_block_input(
    payload: smithy_http::byte_stream::ByteStream,
) -> std::result::Result<smithy_http::body::SdkBody, smithy_http::operation::BuildError> {
    #[allow(clippy::useless_conversion)]
    Ok(smithy_http::body::SdkBody::from(payload.into_inner()))
}

pub fn serialize_operation_start_snapshot(
    input: &crate::input::StartSnapshotInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_start_snapshot_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}
