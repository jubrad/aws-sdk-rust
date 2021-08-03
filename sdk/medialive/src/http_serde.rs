// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_describe_input_device_thumbnail_describe_input_device_thumbnail_output_body(
    body: &mut smithy_http::body::SdkBody,
) -> std::result::Result<
    smithy_http::byte_stream::ByteStream,
    crate::error::DescribeInputDeviceThumbnailError,
> {
    // replace the body with an empty body
    let body = std::mem::replace(body, smithy_http::body::SdkBody::taken());
    Ok(smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_describe_input_device_thumbnail_describe_input_device_thumbnail_output_content_length(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Length").iter();
    let var_1 = smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_1.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub fn deser_header_describe_input_device_thumbnail_describe_input_device_thumbnail_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::ContentType>,
    smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_describe_input_device_thumbnail_describe_input_device_thumbnail_output_e_tag(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("ETag").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_describe_input_device_thumbnail_describe_input_device_thumbnail_output_last_modified(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<smithy_types::Instant>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Last-Modified").iter();
    let var_2: Vec<smithy_types::Instant> =
        smithy_http::header::many_dates(headers, smithy_types::instant::Format::HttpDate)?;
    if var_2.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_2.len()
        )))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}
