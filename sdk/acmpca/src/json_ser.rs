// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_certificate_authority_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCertificateAuthorityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.certificate_authority_configuration {
        let mut object_2 = object
            .key("CertificateAuthorityConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_certificate_authority_configuration(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.revocation_configuration {
        let mut object_4 = object.key("RevocationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_revocation_configuration(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    if let Some(var_5) = &input.certificate_authority_type {
        object
            .key("CertificateAuthorityType")
            .string(var_5.as_str());
    }
    if let Some(var_6) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.key_storage_security_standard {
        object
            .key("KeyStorageSecurityStandard")
            .string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
            {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_certificate_authority_audit_report_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCertificateAuthorityAuditReportInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_12.as_str());
    }
    if let Some(var_13) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_13.as_str());
    }
    if let Some(var_14) = &input.audit_report_response_format {
        object
            .key("AuditReportResponseFormat")
            .string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_permission_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePermissionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_15.as_str());
    }
    if let Some(var_16) = &input.principal {
        object.key("Principal").string(var_16.as_str());
    }
    if let Some(var_17) = &input.source_account {
        object.key("SourceAccount").string(var_17.as_str());
    }
    if let Some(var_18) = &input.actions {
        let mut array_19 = object.key("Actions").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_certificate_authority_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteCertificateAuthorityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_21.as_str());
    }
    if let Some(var_22) = &input.permanent_deletion_time_in_days {
        object.key("PermanentDeletionTimeInDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_permission_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePermissionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_23.as_str());
    }
    if let Some(var_24) = &input.principal {
        object.key("Principal").string(var_24.as_str());
    }
    if let Some(var_25) = &input.source_account {
        object.key("SourceAccount").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.resource_arn {
        object.key("ResourceArn").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_certificate_authority_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCertificateAuthorityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_certificate_authority_audit_report_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCertificateAuthorityAuditReportInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_28.as_str());
    }
    if let Some(var_29) = &input.audit_report_id {
        object.key("AuditReportId").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCertificateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_30.as_str());
    }
    if let Some(var_31) = &input.certificate_arn {
        object.key("CertificateArn").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_certificate_authority_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCertificateAuthorityCertificateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_certificate_authority_csr_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCertificateAuthorityCsrInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.resource_arn {
        object.key("ResourceArn").string(var_34.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_import_certificate_authority_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportCertificateAuthorityCertificateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_35) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_35.as_str());
    }
    if let Some(var_36) = &input.certificate {
        object
            .key("Certificate")
            .string_unchecked(&aws_smithy_types::base64::encode(var_36));
    }
    if let Some(var_37) = &input.certificate_chain {
        object
            .key("CertificateChain")
            .string_unchecked(&aws_smithy_types::base64::encode(var_37));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_issue_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::IssueCertificateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.api_passthrough {
        let mut object_39 = object.key("ApiPassthrough").start_object();
        crate::json_ser::serialize_structure_crate_model_api_passthrough(&mut object_39, var_38)?;
        object_39.finish();
    }
    if let Some(var_40) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_40.as_str());
    }
    if let Some(var_41) = &input.csr {
        object
            .key("Csr")
            .string_unchecked(&aws_smithy_types::base64::encode(var_41));
    }
    if let Some(var_42) = &input.signing_algorithm {
        object.key("SigningAlgorithm").string(var_42.as_str());
    }
    if let Some(var_43) = &input.template_arn {
        object.key("TemplateArn").string(var_43.as_str());
    }
    if let Some(var_44) = &input.validity {
        let mut object_45 = object.key("Validity").start_object();
        crate::json_ser::serialize_structure_crate_model_validity(&mut object_45, var_44)?;
        object_45.finish();
    }
    if let Some(var_46) = &input.validity_not_before {
        let mut object_47 = object.key("ValidityNotBefore").start_object();
        crate::json_ser::serialize_structure_crate_model_validity(&mut object_47, var_46)?;
        object_47.finish();
    }
    if let Some(var_48) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_certificate_authorities_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCertificateAuthoritiesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.next_token {
        object.key("NextToken").string(var_49.as_str());
    }
    if let Some(var_50) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_50).into()),
        );
    }
    if let Some(var_51) = &input.resource_owner {
        object.key("ResourceOwner").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_permissions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPermissionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_52.as_str());
    }
    if let Some(var_53) = &input.next_token {
        object.key("NextToken").string(var_53.as_str());
    }
    if let Some(var_54) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_54).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_55.as_str());
    }
    if let Some(var_56) = &input.next_token {
        object.key("NextToken").string(var_56.as_str());
    }
    if let Some(var_57) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_57).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.resource_arn {
        object.key("ResourceArn").string(var_58.as_str());
    }
    if let Some(var_59) = &input.policy {
        object.key("Policy").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_restore_certificate_authority_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RestoreCertificateAuthorityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_revoke_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RevokeCertificateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_61.as_str());
    }
    if let Some(var_62) = &input.certificate_serial {
        object.key("CertificateSerial").string(var_62.as_str());
    }
    if let Some(var_63) = &input.revocation_reason {
        object.key("RevocationReason").string(var_63.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_certificate_authority_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagCertificateAuthorityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_64.as_str());
    }
    if let Some(var_65) = &input.tags {
        let mut array_66 = object.key("Tags").start_array();
        for item_67 in var_65 {
            {
                let mut object_68 = array_66.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_68, item_67)?;
                object_68.finish();
            }
        }
        array_66.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_certificate_authority_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagCertificateAuthorityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_69.as_str());
    }
    if let Some(var_70) = &input.tags {
        let mut array_71 = object.key("Tags").start_array();
        for item_72 in var_70 {
            {
                let mut object_73 = array_71.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_73, item_72)?;
                object_73.finish();
            }
        }
        array_71.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_certificate_authority_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCertificateAuthorityInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_74.as_str());
    }
    if let Some(var_75) = &input.revocation_configuration {
        let mut object_76 = object.key("RevocationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_revocation_configuration(
            &mut object_76,
            var_75,
        )?;
        object_76.finish();
    }
    if let Some(var_77) = &input.status {
        object.key("Status").string(var_77.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_certificate_authority_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CertificateAuthorityConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.key_algorithm {
        object.key("KeyAlgorithm").string(var_78.as_str());
    }
    if let Some(var_79) = &input.signing_algorithm {
        object.key("SigningAlgorithm").string(var_79.as_str());
    }
    if let Some(var_80) = &input.subject {
        let mut object_81 = object.key("Subject").start_object();
        crate::json_ser::serialize_structure_crate_model_asn1_subject(&mut object_81, var_80)?;
        object_81.finish();
    }
    if let Some(var_82) = &input.csr_extensions {
        let mut object_83 = object.key("CsrExtensions").start_object();
        crate::json_ser::serialize_structure_crate_model_csr_extensions(&mut object_83, var_82)?;
        object_83.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_revocation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RevocationConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.crl_configuration {
        let mut object_85 = object.key("CrlConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_crl_configuration(&mut object_85, var_84)?;
        object_85.finish();
    }
    if let Some(var_86) = &input.ocsp_configuration {
        let mut object_87 = object.key("OcspConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_ocsp_configuration(
            &mut object_87,
            var_86,
        )?;
        object_87.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_88) = &input.key {
        object.key("Key").string(var_88.as_str());
    }
    if let Some(var_89) = &input.value {
        object.key("Value").string(var_89.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_api_passthrough(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ApiPassthrough,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.extensions {
        let mut object_91 = object.key("Extensions").start_object();
        crate::json_ser::serialize_structure_crate_model_extensions(&mut object_91, var_90)?;
        object_91.finish();
    }
    if let Some(var_92) = &input.subject {
        let mut object_93 = object.key("Subject").start_object();
        crate::json_ser::serialize_structure_crate_model_asn1_subject(&mut object_93, var_92)?;
        object_93.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_validity(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Validity,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.value {
        object.key("Value").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_94).into()),
        );
    }
    if let Some(var_95) = &input.r#type {
        object.key("Type").string(var_95.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_asn1_subject(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Asn1Subject,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_96) = &input.country {
        object.key("Country").string(var_96.as_str());
    }
    if let Some(var_97) = &input.organization {
        object.key("Organization").string(var_97.as_str());
    }
    if let Some(var_98) = &input.organizational_unit {
        object.key("OrganizationalUnit").string(var_98.as_str());
    }
    if let Some(var_99) = &input.distinguished_name_qualifier {
        object
            .key("DistinguishedNameQualifier")
            .string(var_99.as_str());
    }
    if let Some(var_100) = &input.state {
        object.key("State").string(var_100.as_str());
    }
    if let Some(var_101) = &input.common_name {
        object.key("CommonName").string(var_101.as_str());
    }
    if let Some(var_102) = &input.serial_number {
        object.key("SerialNumber").string(var_102.as_str());
    }
    if let Some(var_103) = &input.locality {
        object.key("Locality").string(var_103.as_str());
    }
    if let Some(var_104) = &input.title {
        object.key("Title").string(var_104.as_str());
    }
    if let Some(var_105) = &input.surname {
        object.key("Surname").string(var_105.as_str());
    }
    if let Some(var_106) = &input.given_name {
        object.key("GivenName").string(var_106.as_str());
    }
    if let Some(var_107) = &input.initials {
        object.key("Initials").string(var_107.as_str());
    }
    if let Some(var_108) = &input.pseudonym {
        object.key("Pseudonym").string(var_108.as_str());
    }
    if let Some(var_109) = &input.generation_qualifier {
        object.key("GenerationQualifier").string(var_109.as_str());
    }
    if let Some(var_110) = &input.custom_attributes {
        let mut array_111 = object.key("CustomAttributes").start_array();
        for item_112 in var_110 {
            {
                let mut object_113 = array_111.value().start_object();
                crate::json_ser::serialize_structure_crate_model_custom_attribute(
                    &mut object_113,
                    item_112,
                )?;
                object_113.finish();
            }
        }
        array_111.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_csr_extensions(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CsrExtensions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_114) = &input.key_usage {
        let mut object_115 = object.key("KeyUsage").start_object();
        crate::json_ser::serialize_structure_crate_model_key_usage(&mut object_115, var_114)?;
        object_115.finish();
    }
    if let Some(var_116) = &input.subject_information_access {
        let mut array_117 = object.key("SubjectInformationAccess").start_array();
        for item_118 in var_116 {
            {
                let mut object_119 = array_117.value().start_object();
                crate::json_ser::serialize_structure_crate_model_access_description(
                    &mut object_119,
                    item_118,
                )?;
                object_119.finish();
            }
        }
        array_117.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_crl_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrlConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_120) = &input.enabled {
        object.key("Enabled").boolean(*var_120);
    }
    if let Some(var_121) = &input.expiration_in_days {
        object.key("ExpirationInDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_121).into()),
        );
    }
    if let Some(var_122) = &input.custom_cname {
        object.key("CustomCname").string(var_122.as_str());
    }
    if let Some(var_123) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_123.as_str());
    }
    if let Some(var_124) = &input.s3_object_acl {
        object.key("S3ObjectAcl").string(var_124.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ocsp_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OcspConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.enabled {
        object.key("Enabled").boolean(*var_125);
    }
    if let Some(var_126) = &input.ocsp_custom_cname {
        object.key("OcspCustomCname").string(var_126.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_extensions(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Extensions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.certificate_policies {
        let mut array_128 = object.key("CertificatePolicies").start_array();
        for item_129 in var_127 {
            {
                let mut object_130 = array_128.value().start_object();
                crate::json_ser::serialize_structure_crate_model_policy_information(
                    &mut object_130,
                    item_129,
                )?;
                object_130.finish();
            }
        }
        array_128.finish();
    }
    if let Some(var_131) = &input.extended_key_usage {
        let mut array_132 = object.key("ExtendedKeyUsage").start_array();
        for item_133 in var_131 {
            {
                let mut object_134 = array_132.value().start_object();
                crate::json_ser::serialize_structure_crate_model_extended_key_usage(
                    &mut object_134,
                    item_133,
                )?;
                object_134.finish();
            }
        }
        array_132.finish();
    }
    if let Some(var_135) = &input.key_usage {
        let mut object_136 = object.key("KeyUsage").start_object();
        crate::json_ser::serialize_structure_crate_model_key_usage(&mut object_136, var_135)?;
        object_136.finish();
    }
    if let Some(var_137) = &input.subject_alternative_names {
        let mut array_138 = object.key("SubjectAlternativeNames").start_array();
        for item_139 in var_137 {
            {
                let mut object_140 = array_138.value().start_object();
                crate::json_ser::serialize_structure_crate_model_general_name(
                    &mut object_140,
                    item_139,
                )?;
                object_140.finish();
            }
        }
        array_138.finish();
    }
    if let Some(var_141) = &input.custom_extensions {
        let mut array_142 = object.key("CustomExtensions").start_array();
        for item_143 in var_141 {
            {
                let mut object_144 = array_142.value().start_object();
                crate::json_ser::serialize_structure_crate_model_custom_extension(
                    &mut object_144,
                    item_143,
                )?;
                object_144.finish();
            }
        }
        array_142.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_custom_attribute(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CustomAttribute,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_145) = &input.object_identifier {
        object.key("ObjectIdentifier").string(var_145.as_str());
    }
    if let Some(var_146) = &input.value {
        object.key("Value").string(var_146.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_key_usage(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KeyUsage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.digital_signature {
        object
            .key("DigitalSignature")
            .boolean(input.digital_signature);
    }
    if input.non_repudiation {
        object.key("NonRepudiation").boolean(input.non_repudiation);
    }
    if input.key_encipherment {
        object
            .key("KeyEncipherment")
            .boolean(input.key_encipherment);
    }
    if input.data_encipherment {
        object
            .key("DataEncipherment")
            .boolean(input.data_encipherment);
    }
    if input.key_agreement {
        object.key("KeyAgreement").boolean(input.key_agreement);
    }
    if input.key_cert_sign {
        object.key("KeyCertSign").boolean(input.key_cert_sign);
    }
    if input.crl_sign {
        object.key("CRLSign").boolean(input.crl_sign);
    }
    if input.encipher_only {
        object.key("EncipherOnly").boolean(input.encipher_only);
    }
    if input.decipher_only {
        object.key("DecipherOnly").boolean(input.decipher_only);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_access_description(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AccessDescription,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_147) = &input.access_method {
        let mut object_148 = object.key("AccessMethod").start_object();
        crate::json_ser::serialize_structure_crate_model_access_method(&mut object_148, var_147)?;
        object_148.finish();
    }
    if let Some(var_149) = &input.access_location {
        let mut object_150 = object.key("AccessLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_general_name(&mut object_150, var_149)?;
        object_150.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_policy_information(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PolicyInformation,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_151) = &input.cert_policy_id {
        object.key("CertPolicyId").string(var_151.as_str());
    }
    if let Some(var_152) = &input.policy_qualifiers {
        let mut array_153 = object.key("PolicyQualifiers").start_array();
        for item_154 in var_152 {
            {
                let mut object_155 = array_153.value().start_object();
                crate::json_ser::serialize_structure_crate_model_policy_qualifier_info(
                    &mut object_155,
                    item_154,
                )?;
                object_155.finish();
            }
        }
        array_153.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_extended_key_usage(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExtendedKeyUsage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_156) = &input.extended_key_usage_type {
        object.key("ExtendedKeyUsageType").string(var_156.as_str());
    }
    if let Some(var_157) = &input.extended_key_usage_object_identifier {
        object
            .key("ExtendedKeyUsageObjectIdentifier")
            .string(var_157.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_general_name(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::GeneralName,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_158) = &input.other_name {
        let mut object_159 = object.key("OtherName").start_object();
        crate::json_ser::serialize_structure_crate_model_other_name(&mut object_159, var_158)?;
        object_159.finish();
    }
    if let Some(var_160) = &input.rfc822_name {
        object.key("Rfc822Name").string(var_160.as_str());
    }
    if let Some(var_161) = &input.dns_name {
        object.key("DnsName").string(var_161.as_str());
    }
    if let Some(var_162) = &input.directory_name {
        let mut object_163 = object.key("DirectoryName").start_object();
        crate::json_ser::serialize_structure_crate_model_asn1_subject(&mut object_163, var_162)?;
        object_163.finish();
    }
    if let Some(var_164) = &input.edi_party_name {
        let mut object_165 = object.key("EdiPartyName").start_object();
        crate::json_ser::serialize_structure_crate_model_edi_party_name(&mut object_165, var_164)?;
        object_165.finish();
    }
    if let Some(var_166) = &input.uniform_resource_identifier {
        object
            .key("UniformResourceIdentifier")
            .string(var_166.as_str());
    }
    if let Some(var_167) = &input.ip_address {
        object.key("IpAddress").string(var_167.as_str());
    }
    if let Some(var_168) = &input.registered_id {
        object.key("RegisteredId").string(var_168.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_custom_extension(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CustomExtension,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_169) = &input.object_identifier {
        object.key("ObjectIdentifier").string(var_169.as_str());
    }
    if let Some(var_170) = &input.value {
        object.key("Value").string(var_170.as_str());
    }
    if let Some(var_171) = &input.critical {
        object.key("Critical").boolean(*var_171);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_access_method(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AccessMethod,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_172) = &input.custom_object_identifier {
        object
            .key("CustomObjectIdentifier")
            .string(var_172.as_str());
    }
    if let Some(var_173) = &input.access_method_type {
        object.key("AccessMethodType").string(var_173.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_policy_qualifier_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PolicyQualifierInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_174) = &input.policy_qualifier_id {
        object.key("PolicyQualifierId").string(var_174.as_str());
    }
    if let Some(var_175) = &input.qualifier {
        let mut object_176 = object.key("Qualifier").start_object();
        crate::json_ser::serialize_structure_crate_model_qualifier(&mut object_176, var_175)?;
        object_176.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_other_name(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OtherName,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_177) = &input.type_id {
        object.key("TypeId").string(var_177.as_str());
    }
    if let Some(var_178) = &input.value {
        object.key("Value").string(var_178.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_edi_party_name(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EdiPartyName,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_179) = &input.party_name {
        object.key("PartyName").string(var_179.as_str());
    }
    if let Some(var_180) = &input.name_assigner {
        object.key("NameAssigner").string(var_180.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_qualifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Qualifier,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_181) = &input.cps_uri {
        object.key("CpsUri").string(var_181.as_str());
    }
    Ok(())
}
