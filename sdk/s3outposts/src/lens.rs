// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_endpoints_output_next_token(
    input: &crate::output::ListEndpointsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_shared_endpoints_output_next_token(
    input: &crate::output::ListSharedEndpointsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_endpoints_output_endpoints(
    input: crate::output::ListEndpointsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Endpoint>> {
    let input = match input.endpoints {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_shared_endpoints_output_endpoints(
    input: crate::output::ListSharedEndpointsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Endpoint>> {
    let input = match input.endpoints {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
