# AWS SDK Conformance Report: config

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## config
**Progress:** `1262/1262` files compared · `1258` matched · `4` mismatches · `0` missing · `0` extra · `99.68%` match (100.00% means fully matched)

### `src/client/associate_resource_types.rs`

```diff
--- reference/src/client/associate_resource_types.rs
+++ generated/src/client/associate_resource_types.rs
@@ -6,7 +6,7 @@
     ///   - [`configuration_recorder_arn(impl Into<String>)`](crate::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::configuration_recorder_arn) / [`set_configuration_recorder_arn(Option<String>)`](crate::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::set_configuration_recorder_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the specified configuration recorder.</p><br>
     ///   - [`resource_types(ResourceType)`](crate::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::resource_types) / [`set_resource_types(Option<Vec::<ResourceType>>)`](crate::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::set_resource_types):<br>required: **true**<br><p>The list of resource types you want to add to the recording group of the specified configuration recorder.</p><br>
     /// - On success, responds with [`AssociateResourceTypesOutput`](crate::operation::associate_resource_types::AssociateResourceTypesOutput) with field(s):
-    ///   - [`configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::associate_resource_types::AssociateResourceTypesOutput::configuration_recorder): <p>Records configuration changes to the resource types in scope.</p> <p>For more information about the configuration recorder, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b> </a> in the <i>Config Developer Guide</i>.</p>
+    ///   - [`configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::associate_resource_types::AssociateResourceTypesOutput::configuration_recorder): <p>Records configuration changes to the resource types in scope.</p> <p>For more information about the configuration recorder, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b></a> in the <i>Config Developer Guide</i>.</p>
     /// - On failure, responds with [`SdkError<AssociateResourceTypesError>`](crate::operation::associate_resource_types::AssociateResourceTypesError)
     pub fn associate_resource_types(&self) -> super::super::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder {
         super::super::operation::associate_resource_types::builders::AssociateResourceTypesFluentBuilder::new(self.handle.clone())
```

### `src/client/describe_config_rules.rs`

```diff
--- reference/src/client/describe_config_rules.rs
+++ generated/src/client/describe_config_rules.rs
@@ -5,7 +5,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`config_rule_names(impl Into<String>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::config_rule_names) / [`set_config_rule_names(Option<Vec::<String>>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::set_config_rule_names):<br>required: **false**<br><p>The names of the Config rules for which you want details. If you do not specify any names, Config returns details for all your rules.</p><br>
-    ///   - [`filters(DescribeConfigRulesFilters)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::filters) / [`set_filters(Option<DescribeConfigRulesFilters>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::set_filters):<br>required: **false**<br><p>Returns a list of Detective or Proactive Config rules. By default, this API returns an unfiltered list. For more information on Detective or Proactive Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config-rules.html"> <b>Evaluation Mode</b> </a> in the <i>Config Developer Guide</i>.</p><br>
+    ///   - [`filters(DescribeConfigRulesFilters)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::filters) / [`set_filters(Option<DescribeConfigRulesFilters>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::set_filters):<br>required: **false**<br><p>Returns a list of Detective or Proactive Config rules. By default, this API returns an unfiltered list. For more information on Detective or Proactive Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config-rules.html"> <b>Evaluation Mode</b></a> in the <i>Config Developer Guide</i>.</p><br>
     ///   - [`next_token(impl Into<String>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_config_rules::builders::DescribeConfigRulesFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p><br>
     /// - On success, responds with [`DescribeConfigRulesOutput`](crate::operation::describe_config_rules::DescribeConfigRulesOutput) with field(s):
     ///   - [`config_rules(Option<Vec::<ConfigRule>>)`](crate::operation::describe_config_rules::DescribeConfigRulesOutput::config_rules): <p>The details about your Config rules.</p>
```

### `src/client/disassociate_resource_types.rs`

```diff
--- reference/src/client/disassociate_resource_types.rs
+++ generated/src/client/disassociate_resource_types.rs
@@ -6,7 +6,7 @@
     ///   - [`configuration_recorder_arn(impl Into<String>)`](crate::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::configuration_recorder_arn) / [`set_configuration_recorder_arn(Option<String>)`](crate::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::set_configuration_recorder_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the specified configuration recorder.</p><br>
     ///   - [`resource_types(ResourceType)`](crate::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::resource_types) / [`set_resource_types(Option<Vec::<ResourceType>>)`](crate::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::set_resource_types):<br>required: **true**<br><p>The list of resource types you want to remove from the recording group of the specified configuration recorder.</p><br>
     /// - On success, responds with [`DisassociateResourceTypesOutput`](crate::operation::disassociate_resource_types::DisassociateResourceTypesOutput) with field(s):
-    ///   - [`configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::disassociate_resource_types::DisassociateResourceTypesOutput::configuration_recorder): <p>Records configuration changes to the resource types in scope.</p> <p>For more information about the configuration recorder, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b> </a> in the <i>Config Developer Guide</i>.</p>
+    ///   - [`configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::disassociate_resource_types::DisassociateResourceTypesOutput::configuration_recorder): <p>Records configuration changes to the resource types in scope.</p> <p>For more information about the configuration recorder, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b></a> in the <i>Config Developer Guide</i>.</p>
     /// - On failure, responds with [`SdkError<DisassociateResourceTypesError>`](crate::operation::disassociate_resource_types::DisassociateResourceTypesError)
     pub fn disassociate_resource_types(&self) -> super::super::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder {
         super::super::operation::disassociate_resource_types::builders::DisassociateResourceTypesFluentBuilder::new(self.handle.clone())
```

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -203,9 +203,9 @@

 mod lens;

+mod json_errors;
+
 mod serde_util;

-mod json_errors;
-
 #[doc(inline)]
 pub use client::Client;
```
