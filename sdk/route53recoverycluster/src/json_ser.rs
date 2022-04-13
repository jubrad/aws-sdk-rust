// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_get_routing_control_state_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRoutingControlStateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.routing_control_arn {
        object.key("RoutingControlArn").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_routing_controls_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRoutingControlsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_2) = &input.control_panel_arn {
        object.key("ControlPanelArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_routing_control_state_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRoutingControlStateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.routing_control_arn {
        object.key("RoutingControlArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.routing_control_state {
        object.key("RoutingControlState").string(var_6.as_str());
    }
    if let Some(var_7) = &input.safety_rules_to_override {
        let mut array_8 = object.key("SafetyRulesToOverride").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_routing_control_states_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRoutingControlStatesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.update_routing_control_state_entries {
        let mut array_11 = object.key("UpdateRoutingControlStateEntries").start_array();
        for item_12 in var_10 {
            {
                let mut object_13 = array_11.value().start_object();
                crate::json_ser::serialize_structure_crate_model_update_routing_control_state_entry(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.safety_rules_to_override {
        let mut array_15 = object.key("SafetyRulesToOverride").start_array();
        for item_16 in var_14 {
            {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_update_routing_control_state_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateRoutingControlStateEntry,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.routing_control_arn {
        object.key("RoutingControlArn").string(var_17.as_str());
    }
    if let Some(var_18) = &input.routing_control_state {
        object.key("RoutingControlState").string(var_18.as_str());
    }
    Ok(())
}
