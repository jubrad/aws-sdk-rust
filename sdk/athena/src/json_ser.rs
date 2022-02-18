// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_get_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.named_query_ids {
        let mut array_2 = object.key("NamedQueryIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_query_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetQueryExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.query_execution_ids {
        let mut array_5 = object.key("QueryExecutionIds").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_data_catalog_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataCatalogInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.name {
        object.key("Name").string(var_7.as_str());
    }
    if let Some(var_8) = &input.r#type {
        object.key("Type").string(var_8.as_str());
    }
    if let Some(var_9) = &input.description {
        object.key("Description").string(var_9.as_str());
    }
    if let Some(var_10) = &input.parameters {
        let mut object_11 = object.key("Parameters").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    if let Some(var_14) = &input.tags {
        let mut array_15 = object.key("Tags").start_array();
        for item_16 in var_14 {
            {
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.name {
        object.key("Name").string(var_18.as_str());
    }
    if let Some(var_19) = &input.description {
        object.key("Description").string(var_19.as_str());
    }
    if let Some(var_20) = &input.database {
        object.key("Database").string(var_20.as_str());
    }
    if let Some(var_21) = &input.query_string {
        object.key("QueryString").string(var_21.as_str());
    }
    if let Some(var_22) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_22.as_str());
    }
    if let Some(var_23) = &input.work_group {
        object.key("WorkGroup").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.statement_name {
        object.key("StatementName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.work_group {
        object.key("WorkGroup").string(var_25.as_str());
    }
    if let Some(var_26) = &input.query_statement {
        object.key("QueryStatement").string(var_26.as_str());
    }
    if let Some(var_27) = &input.description {
        object.key("Description").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_work_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.name {
        object.key("Name").string(var_28.as_str());
    }
    if let Some(var_29) = &input.configuration {
        let mut object_30 = object.key("Configuration").start_object();
        crate::json_ser::serialize_structure_crate_model_work_group_configuration(
            &mut object_30,
            var_29,
        )?;
        object_30.finish();
    }
    if let Some(var_31) = &input.description {
        object.key("Description").string(var_31.as_str());
    }
    if let Some(var_32) = &input.tags {
        let mut array_33 = object.key("Tags").start_array();
        for item_34 in var_32 {
            {
                let mut object_35 = array_33.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_35, item_34)?;
                object_35.finish();
            }
        }
        array_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_data_catalog_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDataCatalogInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.name {
        object.key("Name").string(var_36.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.named_query_id {
        object.key("NamedQueryId").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.statement_name {
        object.key("StatementName").string(var_38.as_str());
    }
    if let Some(var_39) = &input.work_group {
        object.key("WorkGroup").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_work_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteWorkGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.work_group {
        object.key("WorkGroup").string(var_40.as_str());
    }
    if let Some(var_41) = &input.recursive_delete_option {
        object.key("RecursiveDeleteOption").boolean(*var_41);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_database_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDatabaseInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.catalog_name {
        object.key("CatalogName").string(var_42.as_str());
    }
    if let Some(var_43) = &input.database_name {
        object.key("DatabaseName").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_data_catalog_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDataCatalogInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.name {
        object.key("Name").string(var_44.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_named_query_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetNamedQueryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.named_query_id {
        object.key("NamedQueryId").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.statement_name {
        object.key("StatementName").string(var_46.as_str());
    }
    if let Some(var_47) = &input.work_group {
        object.key("WorkGroup").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_query_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetQueryExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.query_execution_id {
        object.key("QueryExecutionId").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_query_results_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetQueryResultsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.query_execution_id {
        object.key("QueryExecutionId").string(var_49.as_str());
    }
    if let Some(var_50) = &input.next_token {
        object.key("NextToken").string(var_50.as_str());
    }
    if let Some(var_51) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_51).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_table_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTableMetadataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.catalog_name {
        object.key("CatalogName").string(var_52.as_str());
    }
    if let Some(var_53) = &input.database_name {
        object.key("DatabaseName").string(var_53.as_str());
    }
    if let Some(var_54) = &input.table_name {
        object.key("TableName").string(var_54.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_work_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetWorkGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.work_group {
        object.key("WorkGroup").string(var_55.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_databases_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatabasesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.catalog_name {
        object.key("CatalogName").string(var_56.as_str());
    }
    if let Some(var_57) = &input.next_token {
        object.key("NextToken").string(var_57.as_str());
    }
    if let Some(var_58) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_58).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_data_catalogs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDataCatalogsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.next_token {
        object.key("NextToken").string(var_59.as_str());
    }
    if let Some(var_60) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_60).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_engine_versions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEngineVersionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.next_token {
        object.key("NextToken").string(var_61.as_str());
    }
    if let Some(var_62) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_62).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_named_queries_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListNamedQueriesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.next_token {
        object.key("NextToken").string(var_63.as_str());
    }
    if let Some(var_64) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_64).into()),
        );
    }
    if let Some(var_65) = &input.work_group {
        object.key("WorkGroup").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_prepared_statements_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPreparedStatementsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.work_group {
        object.key("WorkGroup").string(var_66.as_str());
    }
    if let Some(var_67) = &input.next_token {
        object.key("NextToken").string(var_67.as_str());
    }
    if let Some(var_68) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_68).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_query_executions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListQueryExecutionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.next_token {
        object.key("NextToken").string(var_69.as_str());
    }
    if let Some(var_70) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_70).into()),
        );
    }
    if let Some(var_71) = &input.work_group {
        object.key("WorkGroup").string(var_71.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_table_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTableMetadataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.catalog_name {
        object.key("CatalogName").string(var_72.as_str());
    }
    if let Some(var_73) = &input.database_name {
        object.key("DatabaseName").string(var_73.as_str());
    }
    if let Some(var_74) = &input.expression {
        object.key("Expression").string(var_74.as_str());
    }
    if let Some(var_75) = &input.next_token {
        object.key("NextToken").string(var_75.as_str());
    }
    if let Some(var_76) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_76).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_77) = &input.resource_arn {
        object.key("ResourceARN").string(var_77.as_str());
    }
    if let Some(var_78) = &input.next_token {
        object.key("NextToken").string(var_78.as_str());
    }
    if let Some(var_79) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_79).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_work_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListWorkGroupsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_80) = &input.next_token {
        object.key("NextToken").string(var_80.as_str());
    }
    if let Some(var_81) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_81).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_query_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartQueryExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_82) = &input.query_string {
        object.key("QueryString").string(var_82.as_str());
    }
    if let Some(var_83) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_83.as_str());
    }
    if let Some(var_84) = &input.query_execution_context {
        let mut object_85 = object.key("QueryExecutionContext").start_object();
        crate::json_ser::serialize_structure_crate_model_query_execution_context(
            &mut object_85,
            var_84,
        )?;
        object_85.finish();
    }
    if let Some(var_86) = &input.result_configuration {
        let mut object_87 = object.key("ResultConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_result_configuration(
            &mut object_87,
            var_86,
        )?;
        object_87.finish();
    }
    if let Some(var_88) = &input.work_group {
        object.key("WorkGroup").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_query_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopQueryExecutionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.query_execution_id {
        object.key("QueryExecutionId").string(var_89.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.resource_arn {
        object.key("ResourceARN").string(var_90.as_str());
    }
    if let Some(var_91) = &input.tags {
        let mut array_92 = object.key("Tags").start_array();
        for item_93 in var_91 {
            {
                let mut object_94 = array_92.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_94, item_93)?;
                object_94.finish();
            }
        }
        array_92.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.resource_arn {
        object.key("ResourceARN").string(var_95.as_str());
    }
    if let Some(var_96) = &input.tag_keys {
        let mut array_97 = object.key("TagKeys").start_array();
        for item_98 in var_96 {
            {
                array_97.value().string(item_98.as_str());
            }
        }
        array_97.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_data_catalog_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDataCatalogInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_99) = &input.name {
        object.key("Name").string(var_99.as_str());
    }
    if let Some(var_100) = &input.r#type {
        object.key("Type").string(var_100.as_str());
    }
    if let Some(var_101) = &input.description {
        object.key("Description").string(var_101.as_str());
    }
    if let Some(var_102) = &input.parameters {
        let mut object_103 = object.key("Parameters").start_object();
        for (key_104, value_105) in var_102 {
            {
                object_103.key(key_104).string(value_105.as_str());
            }
        }
        object_103.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_prepared_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePreparedStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_106) = &input.statement_name {
        object.key("StatementName").string(var_106.as_str());
    }
    if let Some(var_107) = &input.work_group {
        object.key("WorkGroup").string(var_107.as_str());
    }
    if let Some(var_108) = &input.query_statement {
        object.key("QueryStatement").string(var_108.as_str());
    }
    if let Some(var_109) = &input.description {
        object.key("Description").string(var_109.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_work_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWorkGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.work_group {
        object.key("WorkGroup").string(var_110.as_str());
    }
    if let Some(var_111) = &input.description {
        object.key("Description").string(var_111.as_str());
    }
    if let Some(var_112) = &input.configuration_updates {
        let mut object_113 = object.key("ConfigurationUpdates").start_object();
        crate::json_ser::serialize_structure_crate_model_work_group_configuration_updates(
            &mut object_113,
            var_112,
        )?;
        object_113.finish();
    }
    if let Some(var_114) = &input.state {
        object.key("State").string(var_114.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_115) = &input.key {
        object.key("Key").string(var_115.as_str());
    }
    if let Some(var_116) = &input.value {
        object.key("Value").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_work_group_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WorkGroupConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.result_configuration {
        let mut object_118 = object.key("ResultConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_result_configuration(
            &mut object_118,
            var_117,
        )?;
        object_118.finish();
    }
    if let Some(var_119) = &input.enforce_work_group_configuration {
        object
            .key("EnforceWorkGroupConfiguration")
            .boolean(*var_119);
    }
    if let Some(var_120) = &input.publish_cloud_watch_metrics_enabled {
        object
            .key("PublishCloudWatchMetricsEnabled")
            .boolean(*var_120);
    }
    if let Some(var_121) = &input.bytes_scanned_cutoff_per_query {
        object.key("BytesScannedCutoffPerQuery").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_121).into()),
        );
    }
    if let Some(var_122) = &input.requester_pays_enabled {
        object.key("RequesterPaysEnabled").boolean(*var_122);
    }
    if let Some(var_123) = &input.engine_version {
        let mut object_124 = object.key("EngineVersion").start_object();
        crate::json_ser::serialize_structure_crate_model_engine_version(&mut object_124, var_123)?;
        object_124.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_query_execution_context(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::QueryExecutionContext,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.database {
        object.key("Database").string(var_125.as_str());
    }
    if let Some(var_126) = &input.catalog {
        object.key("Catalog").string(var_126.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_result_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResultConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.output_location {
        object.key("OutputLocation").string(var_127.as_str());
    }
    if let Some(var_128) = &input.encryption_configuration {
        let mut object_129 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_configuration(
            &mut object_129,
            var_128,
        )?;
        object_129.finish();
    }
    if let Some(var_130) = &input.expected_bucket_owner {
        object.key("ExpectedBucketOwner").string(var_130.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_work_group_configuration_updates(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WorkGroupConfigurationUpdates,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.enforce_work_group_configuration {
        object
            .key("EnforceWorkGroupConfiguration")
            .boolean(*var_131);
    }
    if let Some(var_132) = &input.result_configuration_updates {
        let mut object_133 = object.key("ResultConfigurationUpdates").start_object();
        crate::json_ser::serialize_structure_crate_model_result_configuration_updates(
            &mut object_133,
            var_132,
        )?;
        object_133.finish();
    }
    if let Some(var_134) = &input.publish_cloud_watch_metrics_enabled {
        object
            .key("PublishCloudWatchMetricsEnabled")
            .boolean(*var_134);
    }
    if let Some(var_135) = &input.bytes_scanned_cutoff_per_query {
        object.key("BytesScannedCutoffPerQuery").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_135).into()),
        );
    }
    if let Some(var_136) = &input.remove_bytes_scanned_cutoff_per_query {
        object
            .key("RemoveBytesScannedCutoffPerQuery")
            .boolean(*var_136);
    }
    if let Some(var_137) = &input.requester_pays_enabled {
        object.key("RequesterPaysEnabled").boolean(*var_137);
    }
    if let Some(var_138) = &input.engine_version {
        let mut object_139 = object.key("EngineVersion").start_object();
        crate::json_ser::serialize_structure_crate_model_engine_version(&mut object_139, var_138)?;
        object_139.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_engine_version(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EngineVersion,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_140) = &input.selected_engine_version {
        object.key("SelectedEngineVersion").string(var_140.as_str());
    }
    if let Some(var_141) = &input.effective_engine_version {
        object
            .key("EffectiveEngineVersion")
            .string(var_141.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_142) = &input.encryption_option {
        object.key("EncryptionOption").string(var_142.as_str());
    }
    if let Some(var_143) = &input.kms_key {
        object.key("KmsKey").string(var_143.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_result_configuration_updates(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResultConfigurationUpdates,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_144) = &input.output_location {
        object.key("OutputLocation").string(var_144.as_str());
    }
    if let Some(var_145) = &input.remove_output_location {
        object.key("RemoveOutputLocation").boolean(*var_145);
    }
    if let Some(var_146) = &input.encryption_configuration {
        let mut object_147 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_configuration(
            &mut object_147,
            var_146,
        )?;
        object_147.finish();
    }
    if let Some(var_148) = &input.remove_encryption_configuration {
        object
            .key("RemoveEncryptionConfiguration")
            .boolean(*var_148);
    }
    if let Some(var_149) = &input.expected_bucket_owner {
        object.key("ExpectedBucketOwner").string(var_149.as_str());
    }
    if let Some(var_150) = &input.remove_expected_bucket_owner {
        object.key("RemoveExpectedBucketOwner").boolean(*var_150);
    }
    Ok(())
}
