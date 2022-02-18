// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Usage allocations allow you to split usage into buckets by tags.</p>
/// <p>Each <code>UsageAllocation</code> indicates the usage quantity for a specific set of tags.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UsageAllocation {
    /// <p>The total quantity allocated to this bucket of usage.</p>
    pub allocated_usage_quantity: std::option::Option<i32>,
    /// <p>The set of tags that define the bucket of usage. For the bucket of items with no tags, this parameter can be left out.</p>
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl UsageAllocation {
    /// <p>The total quantity allocated to this bucket of usage.</p>
    pub fn allocated_usage_quantity(&self) -> std::option::Option<i32> {
        self.allocated_usage_quantity
    }
    /// <p>The set of tags that define the bucket of usage. For the bucket of items with no tags, this parameter can be left out.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::model::Tag]> {
        self.tags.as_deref()
    }
}
impl std::fmt::Debug for UsageAllocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UsageAllocation");
        formatter.field("allocated_usage_quantity", &self.allocated_usage_quantity);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`UsageAllocation`](crate::model::UsageAllocation)
pub mod usage_allocation {
    /// A builder for [`UsageAllocation`](crate::model::UsageAllocation)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) allocated_usage_quantity: std::option::Option<i32>,
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        /// <p>The total quantity allocated to this bucket of usage.</p>
        pub fn allocated_usage_quantity(mut self, input: i32) -> Self {
            self.allocated_usage_quantity = Some(input);
            self
        }
        /// <p>The total quantity allocated to this bucket of usage.</p>
        pub fn set_allocated_usage_quantity(mut self, input: std::option::Option<i32>) -> Self {
            self.allocated_usage_quantity = input;
            self
        }
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The set of tags that define the bucket of usage. For the bucket of items with no tags, this parameter can be left out.</p>
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input);
            self.tags = Some(v);
            self
        }
        /// <p>The set of tags that define the bucket of usage. For the bucket of items with no tags, this parameter can be left out.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`UsageAllocation`](crate::model::UsageAllocation)
        pub fn build(self) -> crate::model::UsageAllocation {
            crate::model::UsageAllocation {
                allocated_usage_quantity: self.allocated_usage_quantity,
                tags: self.tags,
            }
        }
    }
}
impl UsageAllocation {
    /// Creates a new builder-style object to manufacture [`UsageAllocation`](crate::model::UsageAllocation)
    pub fn builder() -> crate::model::usage_allocation::Builder {
        crate::model::usage_allocation::Builder::default()
    }
}

/// <p>Metadata assigned to an allocation. Each tag is made up of a <code>key</code> and a <code>value</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>One part of a key-value pair that makes up a <code>tag</code>. A <code>key</code> is a label that acts like a category for the specific tag values.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>One part of a key-value pair that makes up a <code>tag</code>. A <code>value</code> acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    pub value: std::option::Option<std::string::String>,
}
impl Tag {
    /// <p>One part of a key-value pair that makes up a <code>tag</code>. A <code>key</code> is a label that acts like a category for the specific tag values.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>One part of a key-value pair that makes up a <code>tag</code>. A <code>value</code> acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    pub fn value(&self) -> std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {
    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>One part of a key-value pair that makes up a <code>tag</code>. A <code>key</code> is a label that acts like a category for the specific tag values.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p>One part of a key-value pair that makes up a <code>tag</code>. A <code>key</code> is a label that acts like a category for the specific tag values.</p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>One part of a key-value pair that makes up a <code>tag</code>. A <code>value</code> acts as a descriptor within a tag category (key). The value can be empty or null.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>One part of a key-value pair that makes up a <code>tag</code>. A <code>value</code> acts as a descriptor within a tag category (key). The value can be empty or null.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// <p>A <code>UsageRecord</code> indicates a quantity of usage for a given product, customer, dimension and time.</p>
/// <p>Multiple requests with the same <code>UsageRecords</code> as input will be de-duplicated to prevent double charges.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UsageRecord {
    /// <p>Timestamp, in UTC, for which the usage is being reported.</p>
    /// <p>Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
    pub timestamp: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The <code>CustomerIdentifier</code> is obtained through the <code>ResolveCustomer</code> operation and represents an individual buyer in your application.</p>
    pub customer_identifier: std::option::Option<std::string::String>,
    /// <p>During the process of registering a product on AWS Marketplace, dimensions are specified. These represent different units of value in your application.</p>
    pub dimension: std::option::Option<std::string::String>,
    /// <p>The quantity of usage consumed by the customer for the given dimension and time. Defaults to <code>0</code> if not specified.</p>
    pub quantity: std::option::Option<i32>,
    /// <p>The set of <code>UsageAllocations</code> to submit. The sum of all <code>UsageAllocation</code> quantities must equal the Quantity of the <code>UsageRecord</code>.</p>
    pub usage_allocations: std::option::Option<std::vec::Vec<crate::model::UsageAllocation>>,
}
impl UsageRecord {
    /// <p>Timestamp, in UTC, for which the usage is being reported.</p>
    /// <p>Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
    pub fn timestamp(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p>The <code>CustomerIdentifier</code> is obtained through the <code>ResolveCustomer</code> operation and represents an individual buyer in your application.</p>
    pub fn customer_identifier(&self) -> std::option::Option<&str> {
        self.customer_identifier.as_deref()
    }
    /// <p>During the process of registering a product on AWS Marketplace, dimensions are specified. These represent different units of value in your application.</p>
    pub fn dimension(&self) -> std::option::Option<&str> {
        self.dimension.as_deref()
    }
    /// <p>The quantity of usage consumed by the customer for the given dimension and time. Defaults to <code>0</code> if not specified.</p>
    pub fn quantity(&self) -> std::option::Option<i32> {
        self.quantity
    }
    /// <p>The set of <code>UsageAllocations</code> to submit. The sum of all <code>UsageAllocation</code> quantities must equal the Quantity of the <code>UsageRecord</code>.</p>
    pub fn usage_allocations(&self) -> std::option::Option<&[crate::model::UsageAllocation]> {
        self.usage_allocations.as_deref()
    }
}
impl std::fmt::Debug for UsageRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UsageRecord");
        formatter.field("timestamp", &self.timestamp);
        formatter.field("customer_identifier", &self.customer_identifier);
        formatter.field("dimension", &self.dimension);
        formatter.field("quantity", &self.quantity);
        formatter.field("usage_allocations", &self.usage_allocations);
        formatter.finish()
    }
}
/// See [`UsageRecord`](crate::model::UsageRecord)
pub mod usage_record {
    /// A builder for [`UsageRecord`](crate::model::UsageRecord)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) timestamp: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) customer_identifier: std::option::Option<std::string::String>,
        pub(crate) dimension: std::option::Option<std::string::String>,
        pub(crate) quantity: std::option::Option<i32>,
        pub(crate) usage_allocations:
            std::option::Option<std::vec::Vec<crate::model::UsageAllocation>>,
    }
    impl Builder {
        /// <p>Timestamp, in UTC, for which the usage is being reported.</p>
        /// <p>Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
        pub fn timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.timestamp = Some(input);
            self
        }
        /// <p>Timestamp, in UTC, for which the usage is being reported.</p>
        /// <p>Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
        pub fn set_timestamp(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.timestamp = input;
            self
        }
        /// <p>The <code>CustomerIdentifier</code> is obtained through the <code>ResolveCustomer</code> operation and represents an individual buyer in your application.</p>
        pub fn customer_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.customer_identifier = Some(input.into());
            self
        }
        /// <p>The <code>CustomerIdentifier</code> is obtained through the <code>ResolveCustomer</code> operation and represents an individual buyer in your application.</p>
        pub fn set_customer_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.customer_identifier = input;
            self
        }
        /// <p>During the process of registering a product on AWS Marketplace, dimensions are specified. These represent different units of value in your application.</p>
        pub fn dimension(mut self, input: impl Into<std::string::String>) -> Self {
            self.dimension = Some(input.into());
            self
        }
        /// <p>During the process of registering a product on AWS Marketplace, dimensions are specified. These represent different units of value in your application.</p>
        pub fn set_dimension(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.dimension = input;
            self
        }
        /// <p>The quantity of usage consumed by the customer for the given dimension and time. Defaults to <code>0</code> if not specified.</p>
        pub fn quantity(mut self, input: i32) -> Self {
            self.quantity = Some(input);
            self
        }
        /// <p>The quantity of usage consumed by the customer for the given dimension and time. Defaults to <code>0</code> if not specified.</p>
        pub fn set_quantity(mut self, input: std::option::Option<i32>) -> Self {
            self.quantity = input;
            self
        }
        /// Appends an item to `usage_allocations`.
        ///
        /// To override the contents of this collection use [`set_usage_allocations`](Self::set_usage_allocations).
        ///
        /// <p>The set of <code>UsageAllocations</code> to submit. The sum of all <code>UsageAllocation</code> quantities must equal the Quantity of the <code>UsageRecord</code>.</p>
        pub fn usage_allocations(mut self, input: crate::model::UsageAllocation) -> Self {
            let mut v = self.usage_allocations.unwrap_or_default();
            v.push(input);
            self.usage_allocations = Some(v);
            self
        }
        /// <p>The set of <code>UsageAllocations</code> to submit. The sum of all <code>UsageAllocation</code> quantities must equal the Quantity of the <code>UsageRecord</code>.</p>
        pub fn set_usage_allocations(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UsageAllocation>>,
        ) -> Self {
            self.usage_allocations = input;
            self
        }
        /// Consumes the builder and constructs a [`UsageRecord`](crate::model::UsageRecord)
        pub fn build(self) -> crate::model::UsageRecord {
            crate::model::UsageRecord {
                timestamp: self.timestamp,
                customer_identifier: self.customer_identifier,
                dimension: self.dimension,
                quantity: self.quantity,
                usage_allocations: self.usage_allocations,
            }
        }
    }
}
impl UsageRecord {
    /// Creates a new builder-style object to manufacture [`UsageRecord`](crate::model::UsageRecord)
    pub fn builder() -> crate::model::usage_record::Builder {
        crate::model::usage_record::Builder::default()
    }
}

/// <p>A <code>UsageRecordResult</code> indicates the status of a given <code>UsageRecord</code> processed by <code>BatchMeterUsage</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UsageRecordResult {
    /// <p>The <code>UsageRecord</code> that was part of the <code>BatchMeterUsage</code> request.</p>
    pub usage_record: std::option::Option<crate::model::UsageRecord>,
    /// <p>The <code>MeteringRecordId</code> is a unique identifier for this metering event.</p>
    pub metering_record_id: std::option::Option<std::string::String>,
    /// <p>The <code>UsageRecordResult</code> <code>Status</code> indicates the status of an individual <code>UsageRecord</code> processed by <code>BatchMeterUsage</code>.</p>
    /// <ul>
    /// <li> <p> <i>Success</i>- The <code>UsageRecord</code> was accepted and honored by <code>BatchMeterUsage</code>.</p> </li>
    /// <li> <p> <i>CustomerNotSubscribed</i>- The <code>CustomerIdentifier</code> specified is not able to use your product. The <code>UsageRecord</code> was not honored. There are three causes for this result:</p>
    /// <ul>
    /// <li> <p>The customer identifier is invalid.</p> </li>
    /// <li> <p>The customer identifier provided in the metering record does not have an active agreement or subscription with this product. Future <code>UsageRecords</code> for this customer will fail until the customer subscribes to your product.</p> </li>
    /// <li> <p>The customer's AWS account was suspended.</p> </li>
    /// </ul> </li>
    /// <li> <p> <i>DuplicateRecord</i>- Indicates that the <code>UsageRecord</code> was invalid and not honored. A previously metered <code>UsageRecord</code> had the same customer, dimension, and time, but a different quantity.</p> </li>
    /// </ul>
    pub status: std::option::Option<crate::model::UsageRecordResultStatus>,
}
impl UsageRecordResult {
    /// <p>The <code>UsageRecord</code> that was part of the <code>BatchMeterUsage</code> request.</p>
    pub fn usage_record(&self) -> std::option::Option<&crate::model::UsageRecord> {
        self.usage_record.as_ref()
    }
    /// <p>The <code>MeteringRecordId</code> is a unique identifier for this metering event.</p>
    pub fn metering_record_id(&self) -> std::option::Option<&str> {
        self.metering_record_id.as_deref()
    }
    /// <p>The <code>UsageRecordResult</code> <code>Status</code> indicates the status of an individual <code>UsageRecord</code> processed by <code>BatchMeterUsage</code>.</p>
    /// <ul>
    /// <li> <p> <i>Success</i>- The <code>UsageRecord</code> was accepted and honored by <code>BatchMeterUsage</code>.</p> </li>
    /// <li> <p> <i>CustomerNotSubscribed</i>- The <code>CustomerIdentifier</code> specified is not able to use your product. The <code>UsageRecord</code> was not honored. There are three causes for this result:</p>
    /// <ul>
    /// <li> <p>The customer identifier is invalid.</p> </li>
    /// <li> <p>The customer identifier provided in the metering record does not have an active agreement or subscription with this product. Future <code>UsageRecords</code> for this customer will fail until the customer subscribes to your product.</p> </li>
    /// <li> <p>The customer's AWS account was suspended.</p> </li>
    /// </ul> </li>
    /// <li> <p> <i>DuplicateRecord</i>- Indicates that the <code>UsageRecord</code> was invalid and not honored. A previously metered <code>UsageRecord</code> had the same customer, dimension, and time, but a different quantity.</p> </li>
    /// </ul>
    pub fn status(&self) -> std::option::Option<&crate::model::UsageRecordResultStatus> {
        self.status.as_ref()
    }
}
impl std::fmt::Debug for UsageRecordResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UsageRecordResult");
        formatter.field("usage_record", &self.usage_record);
        formatter.field("metering_record_id", &self.metering_record_id);
        formatter.field("status", &self.status);
        formatter.finish()
    }
}
/// See [`UsageRecordResult`](crate::model::UsageRecordResult)
pub mod usage_record_result {
    /// A builder for [`UsageRecordResult`](crate::model::UsageRecordResult)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) usage_record: std::option::Option<crate::model::UsageRecord>,
        pub(crate) metering_record_id: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::UsageRecordResultStatus>,
    }
    impl Builder {
        /// <p>The <code>UsageRecord</code> that was part of the <code>BatchMeterUsage</code> request.</p>
        pub fn usage_record(mut self, input: crate::model::UsageRecord) -> Self {
            self.usage_record = Some(input);
            self
        }
        /// <p>The <code>UsageRecord</code> that was part of the <code>BatchMeterUsage</code> request.</p>
        pub fn set_usage_record(
            mut self,
            input: std::option::Option<crate::model::UsageRecord>,
        ) -> Self {
            self.usage_record = input;
            self
        }
        /// <p>The <code>MeteringRecordId</code> is a unique identifier for this metering event.</p>
        pub fn metering_record_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.metering_record_id = Some(input.into());
            self
        }
        /// <p>The <code>MeteringRecordId</code> is a unique identifier for this metering event.</p>
        pub fn set_metering_record_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.metering_record_id = input;
            self
        }
        /// <p>The <code>UsageRecordResult</code> <code>Status</code> indicates the status of an individual <code>UsageRecord</code> processed by <code>BatchMeterUsage</code>.</p>
        /// <ul>
        /// <li> <p> <i>Success</i>- The <code>UsageRecord</code> was accepted and honored by <code>BatchMeterUsage</code>.</p> </li>
        /// <li> <p> <i>CustomerNotSubscribed</i>- The <code>CustomerIdentifier</code> specified is not able to use your product. The <code>UsageRecord</code> was not honored. There are three causes for this result:</p>
        /// <ul>
        /// <li> <p>The customer identifier is invalid.</p> </li>
        /// <li> <p>The customer identifier provided in the metering record does not have an active agreement or subscription with this product. Future <code>UsageRecords</code> for this customer will fail until the customer subscribes to your product.</p> </li>
        /// <li> <p>The customer's AWS account was suspended.</p> </li>
        /// </ul> </li>
        /// <li> <p> <i>DuplicateRecord</i>- Indicates that the <code>UsageRecord</code> was invalid and not honored. A previously metered <code>UsageRecord</code> had the same customer, dimension, and time, but a different quantity.</p> </li>
        /// </ul>
        pub fn status(mut self, input: crate::model::UsageRecordResultStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The <code>UsageRecordResult</code> <code>Status</code> indicates the status of an individual <code>UsageRecord</code> processed by <code>BatchMeterUsage</code>.</p>
        /// <ul>
        /// <li> <p> <i>Success</i>- The <code>UsageRecord</code> was accepted and honored by <code>BatchMeterUsage</code>.</p> </li>
        /// <li> <p> <i>CustomerNotSubscribed</i>- The <code>CustomerIdentifier</code> specified is not able to use your product. The <code>UsageRecord</code> was not honored. There are three causes for this result:</p>
        /// <ul>
        /// <li> <p>The customer identifier is invalid.</p> </li>
        /// <li> <p>The customer identifier provided in the metering record does not have an active agreement or subscription with this product. Future <code>UsageRecords</code> for this customer will fail until the customer subscribes to your product.</p> </li>
        /// <li> <p>The customer's AWS account was suspended.</p> </li>
        /// </ul> </li>
        /// <li> <p> <i>DuplicateRecord</i>- Indicates that the <code>UsageRecord</code> was invalid and not honored. A previously metered <code>UsageRecord</code> had the same customer, dimension, and time, but a different quantity.</p> </li>
        /// </ul>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::UsageRecordResultStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// Consumes the builder and constructs a [`UsageRecordResult`](crate::model::UsageRecordResult)
        pub fn build(self) -> crate::model::UsageRecordResult {
            crate::model::UsageRecordResult {
                usage_record: self.usage_record,
                metering_record_id: self.metering_record_id,
                status: self.status,
            }
        }
    }
}
impl UsageRecordResult {
    /// Creates a new builder-style object to manufacture [`UsageRecordResult`](crate::model::UsageRecordResult)
    pub fn builder() -> crate::model::usage_record_result::Builder {
        crate::model::usage_record_result::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum UsageRecordResultStatus {
    #[allow(missing_docs)] // documentation missing in model
    CustomerNotSubscribed,
    #[allow(missing_docs)] // documentation missing in model
    DuplicateRecord,
    #[allow(missing_docs)] // documentation missing in model
    Success,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for UsageRecordResultStatus {
    fn from(s: &str) -> Self {
        match s {
            "CustomerNotSubscribed" => UsageRecordResultStatus::CustomerNotSubscribed,
            "DuplicateRecord" => UsageRecordResultStatus::DuplicateRecord,
            "Success" => UsageRecordResultStatus::Success,
            other => UsageRecordResultStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for UsageRecordResultStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(UsageRecordResultStatus::from(s))
    }
}
impl UsageRecordResultStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            UsageRecordResultStatus::CustomerNotSubscribed => "CustomerNotSubscribed",
            UsageRecordResultStatus::DuplicateRecord => "DuplicateRecord",
            UsageRecordResultStatus::Success => "Success",
            UsageRecordResultStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CustomerNotSubscribed", "DuplicateRecord", "Success"]
    }
}
impl AsRef<str> for UsageRecordResultStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
