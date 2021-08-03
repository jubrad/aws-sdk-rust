// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_list_tags_for_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
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

pub fn deser_header_list_tags_for_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Retry-After").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_tag_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_2 = smithy_http::header::read_many_primitive::<i64>(headers)?;
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

pub fn deser_header_tag_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Retry-After").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_untag_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_3 = smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_3.len() > 1 {
        Err(smithy_http::header::ParseError::new_with_message(format!(
            "expected one item but found {}",
            var_3.len()
        )))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_untag_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Retry-After").iter();
    smithy_http::header::one_or_none(headers)
}
