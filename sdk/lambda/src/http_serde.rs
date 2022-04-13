// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_invoke(
    input: &crate::input::InvokeInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.invocation_type {
        let formatted_2 = AsRef::<str>::as_ref(inner_1);
        if !formatted_2.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "invocation_type",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amz-Invocation-Type", header_value);
        }
    }
    if let Some(inner_3) = &input.log_type {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "log_type",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amz-Log-Type", header_value);
        }
    }
    if let Some(inner_5) = &input.client_context {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "client_context",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amz-Client-Context", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_header_add_layer_version_permission_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_add_permission_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_alias_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_event_source_mapping_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_function_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_function_url_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_alias_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_event_source_mapping_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_function_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_function_code_signing_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_function_concurrency_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_function_event_invoke_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_function_url_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_layer_version_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_delete_provisioned_concurrency_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_account_settings_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_alias_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_event_source_mapping_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_function_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_function_code_signing_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_function_concurrency_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_function_configuration_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_function_event_invoke_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_function_url_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_layer_version_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_layer_version_by_arn_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_layer_version_policy_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_policy_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_provisioned_concurrency_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_invoke_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_invoke_invoke_output_executed_version(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("X-Amz-Executed-Version").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_invoke_invoke_output_function_error(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("X-Amz-Function-Error").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_invoke_invoke_output_log_result(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("X-Amz-Log-Result").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_payload_invoke_invoke_output_payload(
    body: &[u8],
) -> std::result::Result<std::option::Option<aws_smithy_types::Blob>, crate::error::InvokeError> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_header_list_aliases_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_event_source_mappings_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_function_event_invoke_configs_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_functions_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_function_url_configs_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_layers_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_layer_versions_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_provisioned_concurrency_configs_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_tags_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_list_versions_by_function_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_publish_layer_version_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_publish_version_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_function_code_signing_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_function_concurrency_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_function_event_invoke_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_provisioned_concurrency_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_remove_layer_version_permission_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_remove_permission_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_tag_resource_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_untag_resource_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_update_alias_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_update_event_source_mapping_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_update_function_code_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_update_function_configuration_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_update_function_event_invoke_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_update_function_url_config_too_many_requests_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}
