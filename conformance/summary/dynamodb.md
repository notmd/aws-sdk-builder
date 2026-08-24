# AWS SDK Conformance Report: dynamodb

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## dynamodb
**Progress:** `882/882` files compared · `587` matched · `295` mismatches · `0` missing · `0` extra · `66.55%` match (100.00% means fully matched)

### `src/client/batch_get_item.rs`

```diff
--- reference/src/client/batch_get_item.rs
+++ generated/src/client/batch_get_item.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`BatchGetItem`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`request_items(impl Into<String>, KeysAndAttributes)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::request_items) / [`set_request_items(Option<HashMap::<String, KeysAndAttributes>>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_request_items):<br>required: **true**<br><p>A map of one or more table names or table ARNs and, for each table, a map that describes one or more items to retrieve from that table. Each table name or ARN can be used only once per <code>BatchGetItem</code> request.</p> <p>Each element in the map of items to retrieve consists of the following:</p> <ul>  <li>   <p><code>ConsistentRead</code> - If <code>true</code>, a strongly consistent read is used; if <code>false</code> (the default), an eventually consistent read is used.</p></li>  <li>   <p><code>ExpressionAttributeNames</code> - One or more substitution tokens for attribute names in the <code>ProjectionExpression</code> parameter. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>   <ul>    <li>     <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>    <li>     <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>    <li>     <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>   </ul>   <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>   <ul>    <li>     <p><code>Percentile</code></p></li>   </ul>   <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>   <ul>    <li>     <p><code>{"#P":"Percentile"}</code></p></li>   </ul>   <p>You could then use this substitution in an expression, as in this example:</p>   <ul>    <li>     <p><code>#P = :val</code></p></li>   </ul><note>    <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>   </note>   <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>  <li>   <p><code>Keys</code> - An array of primary key attribute values that define specific items in the table. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key value. For a composite key, you must provide <i>both</i> the partition key value and the sort key value.</p></li>  <li>   <p><code>ProjectionExpression</code> - A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>   <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p>   <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>  <li>   <p><code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li> </ul><br>
+    ///   - [`request_items(impl Into<String>, KeysAndAttributes)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::request_items) / [`set_request_items(Option<HashMap::<String, KeysAndAttributes>>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_request_items):<br>required: **true**<br><p>A map of one or more table names or table ARNs and, for each table, a map that describes one or more items to retrieve from that table. Each table name or ARN can be used only once per <code>BatchGetItem</code> request.</p> <p>Each element in the map of items to retrieve consists of the following:</p> <ul>  <li>   <p><code>ConsistentRead</code> - If <code>true</code>, a strongly consistent read is used; if <code>false</code> (the default), an eventually consistent read is used.</p></li>  <li>   <p><code>ExpressionAttributeNames</code> - One or more substitution tokens for attribute names in the <code>ProjectionExpression</code> parameter. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>   <ul>    <li>     <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>    <li>     <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>    <li>     <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>   </ul>   <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>   <ul>    <li>     <p><code>Percentile</code></p></li>   </ul>   <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>   <ul>    <li>     <p><code>{"#P":"Percentile"}</code></p></li>   </ul>   <p>You could then use this substitution in an expression, as in this example:</p>   <ul>    <li>     <p><code>#P = :val</code></p></li>   </ul> <note>    <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>   </note>   <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>  <li>   <p><code>Keys</code> - An array of primary key attribute values that define specific items in the table. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key value. For a composite key, you must provide <i>both</i> the partition key value and the sort key value.</p></li>  <li>   <p><code>ProjectionExpression</code> - A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>   <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p>   <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>  <li>   <p><code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li> </ul><br>
     ///   - [`return_consumed_capacity(ReturnConsumedCapacity)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<ReturnConsumedCapacity>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_return_consumed_capacity):<br>required: **false**<br><p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p> <ul>  <li>   <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>   <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>  <li>   <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>  <li>   <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li> </ul><br>
     /// - On success, responds with [`BatchGetItemOutput`](crate::operation::batch_get_item::BatchGetItemOutput) with field(s):
     ///   - [`responses(Option<HashMap::<String, Vec::<HashMap::<String, AttributeValue>>>>)`](crate::operation::batch_get_item::BatchGetItemOutput::responses): <p>A map of table name or table ARN to a list of items. Each object in <code>Responses</code> consists of a table name or ARN, along with a map of attribute data consisting of the data type and attribute value.</p>
```

### `src/client/create_table.rs`

```diff
--- reference/src/client/create_table.rs
+++ generated/src/client/create_table.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`attribute_definitions(AttributeDefinition)`](crate::operation::create_table::builders::CreateTableFluentBuilder::attribute_definitions) / [`set_attribute_definitions(Option<Vec::<AttributeDefinition>>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::set_attribute_definitions):<br>required: **false**<br><p>An array of attributes that describe the key schema for the table and indexes.</p><br>
     ///   - [`table_name(impl Into<String>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::set_table_name):<br>required: **true**<br><p>The name of the table to create. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p><br>
-    ///   - [`key_schema(KeySchemaElement)`](crate::operation::create_table::builders::CreateTableFluentBuilder::key_schema) / [`set_key_schema(Option<Vec::<KeySchemaElement>>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::set_key_schema):<br>required: **false**<br><p>Specifies the attributes that make up the primary key for a table or an index. The attributes in <code>KeySchema</code> must also be defined in the <code>AttributeDefinitions</code> array. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html">Data Model</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Each <code>KeySchemaElement</code> in the array is composed of:</p> <ul>  <li>   <p><code>AttributeName</code> - The name of this key attribute.</p></li>  <li>   <p><code>KeyType</code> - The role that the key attribute will assume:</p>   <ul>    <li>     <p><code>HASH</code> - partition key</p></li>    <li>     <p><code>RANGE</code> - sort key</p></li>   </ul></li> </ul><note>  <p>The partition key of an item is also known as its <i>hash attribute</i>. The term "hash attribute" derives from the DynamoDB usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p>  <p>The sort key of an item is also known as its <i>range attribute</i>. The term "range attribute" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note> <p>For a simple primary key (partition key), you must provide exactly one element with a <code>KeyType</code> of <code>HASH</code>.</p> <p>For a composite primary key (partition key and sort key), you must provide exactly two elements, in this order: The first element must have a <code>KeyType</code> of <code>HASH</code>, and the second element must have a <code>KeyType</code> of <code>RANGE</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTables.primary.key">Working with Tables</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`key_schema(KeySchemaElement)`](crate::operation::create_table::builders::CreateTableFluentBuilder::key_schema) / [`set_key_schema(Option<Vec::<KeySchemaElement>>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::set_key_schema):<br>required: **false**<br><p>Specifies the attributes that make up the primary key for a table or an index. The attributes in <code>KeySchema</code> must also be defined in the <code>AttributeDefinitions</code> array. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html">Data Model</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Each <code>KeySchemaElement</code> in the array is composed of:</p> <ul>  <li>   <p><code>AttributeName</code> - The name of this key attribute.</p></li>  <li>   <p><code>KeyType</code> - The role that the key attribute will assume:</p>   <ul>    <li>     <p><code>HASH</code> - partition key</p></li>    <li>     <p><code>RANGE</code> - sort key</p></li>   </ul></li> </ul> <note>  <p>The partition key of an item is also known as its <i>hash attribute</i>. The term "hash attribute" derives from the DynamoDB usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p>  <p>The sort key of an item is also known as its <i>range attribute</i>. The term "range attribute" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note> <p>For a simple primary key (partition key), you must provide exactly one element with a <code>KeyType</code> of <code>HASH</code>.</p> <p>For a composite primary key (partition key and sort key), you must provide exactly two elements, in this order: The first element must have a <code>KeyType</code> of <code>HASH</code>, and the second element must have a <code>KeyType</code> of <code>RANGE</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTables.primary.key">Working with Tables</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`local_secondary_indexes(LocalSecondaryIndex)`](crate::operation::create_table::builders::CreateTableFluentBuilder::local_secondary_indexes) / [`set_local_secondary_indexes(Option<Vec::<LocalSecondaryIndex>>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::set_local_secondary_indexes):<br>required: **false**<br><p>One or more local secondary indexes (the maximum is 5) to be created on the table. Each index is scoped to a given partition key value. There is a 10 GB size limit per partition key value; otherwise, the size of a local secondary index is unconstrained.</p> <p>Each local secondary index in the array includes the following:</p> <ul>  <li>   <p><code>IndexName</code> - The name of the local secondary index. Must be unique only for this table.</p>   <p></p></li>  <li>   <p><code>KeySchema</code> - Specifies the key schema for the local secondary index. The key schema must begin with the same partition key as the table.</p></li>  <li>   <p><code>Projection</code> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p>   <ul>    <li>     <p><code>ProjectionType</code> - One of the following:</p>     <ul>      <li>       <p><code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p></li>      <li>       <p><code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes is in <code>NonKeyAttributes</code>.</p></li>      <li>       <p><code>ALL</code> - All of the table attributes are projected into the index.</p></li>     </ul></li>    <li>     <p><code>NonKeyAttributes</code> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <code>NonKeyAttributes</code>, summed across all of the secondary indexes, must not exceed 100. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total. This limit only applies when you specify the ProjectionType of <code>INCLUDE</code>. You still can specify the ProjectionType of <code>ALL</code> to project all attributes from the source table, even if the table has more than 100 attributes.</p></li>   </ul></li> </ul><br>
     ///   - [`global_secondary_indexes(GlobalSecondaryIndex)`](crate::operation::create_table::builders::CreateTableFluentBuilder::global_secondary_indexes) / [`set_global_secondary_indexes(Option<Vec::<GlobalSecondaryIndex>>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::set_global_secondary_indexes):<br>required: **false**<br><p>One or more global secondary indexes (the maximum is 20) to be created on the table. Each global secondary index in the array includes the following:</p> <ul>  <li>   <p><code>IndexName</code> - The name of the global secondary index. Must be unique only for this table.</p>   <p></p></li>  <li>   <p><code>KeySchema</code> - Specifies the key schema for the global secondary index. Each global secondary index supports up to 4 partition keys and up to 4 sort keys.</p></li>  <li>   <p><code>Projection</code> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p>   <ul>    <li>     <p><code>ProjectionType</code> - One of the following:</p>     <ul>      <li>       <p><code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p></li>      <li>       <p><code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes is in <code>NonKeyAttributes</code>.</p></li>      <li>       <p><code>ALL</code> - All of the table attributes are projected into the index.</p></li>     </ul></li>    <li>     <p><code>NonKeyAttributes</code> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <code>NonKeyAttributes</code>, summed across all of the secondary indexes, must not exceed 100. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total. This limit only applies when you specify the ProjectionType of <code>INCLUDE</code>. You still can specify the ProjectionType of <code>ALL</code> to project all attributes from the source table, even if the table has more than 100 attributes.</p></li>   </ul></li>  <li>   <p><code>ProvisionedThroughput</code> - The provisioned throughput settings for the global secondary index, consisting of read and write capacity units.</p></li> </ul><br>
     ///   - [`billing_mode(BillingMode)`](crate::operation::create_table::builders::CreateTableFluentBuilder::billing_mode) / [`set_billing_mode(Option<BillingMode>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::set_billing_mode):<br>required: **false**<br><p>Controls how you are charged for read and write throughput and how you manage capacity. This setting can be changed later.</p> <ul>  <li>   <p><code>PAY_PER_REQUEST</code> - We recommend using <code>PAY_PER_REQUEST</code> for most DynamoDB workloads. <code>PAY_PER_REQUEST</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/on-demand-capacity-mode.html">On-demand capacity mode</a>.</p></li>  <li>   <p><code>PROVISIONED</code> - We recommend using <code>PROVISIONED</code> for steady workloads with predictable growth where capacity requirements can be reliably forecasted. <code>PROVISIONED</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/provisioned-capacity-mode.html">Provisioned capacity mode</a>.</p></li> </ul><br>
```

### `src/client/delete_item.rs`

```diff
--- reference/src/client/delete_item.rs
+++ generated/src/client/delete_item.rs
@@ -11,7 +11,7 @@
     ///   - [`return_consumed_capacity(ReturnConsumedCapacity)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<ReturnConsumedCapacity>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::set_return_consumed_capacity):<br>required: **false**<br><p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p> <ul>  <li>   <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>   <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>  <li>   <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>  <li>   <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li> </ul><br>
     ///   - [`return_item_collection_metrics(ReturnItemCollectionMetrics)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::return_item_collection_metrics) / [`set_return_item_collection_metrics(Option<ReturnItemCollectionMetrics>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::set_return_item_collection_metrics):<br>required: **false**<br><p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p><br>
     ///   - [`condition_expression(impl Into<String>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::condition_expression) / [`set_condition_expression(Option<String>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::set_condition_expression):<br>required: **false**<br><p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p> <p>An expression can contain any of the following:</p> <ul>  <li>   <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code></p>   <p>These function names are case-sensitive.</p></li>  <li>   <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code></p></li>  <li>   <p>Logical operators: <code>AND | OR | NOT</code></p></li> </ul> <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`expression_attribute_values(impl Into<String>, AttributeValue)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap::<String, AttributeValue>>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::set_expression_attribute_values):<br>required: **false**<br><p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following:</p> <p><code>Available | Backordered | Discontinued</code></p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p> <p>You could then use these values in an expression, such as this:</p> <p><code>ProductStatus IN (:avail, :back, :disc)</code></p> <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`return_values_on_condition_check_failure(ReturnValuesOnConditionCheckFailure)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::return_values_on_condition_check_failure) / [`set_return_values_on_condition_check_failure(Option<ReturnValuesOnConditionCheckFailure>)`](crate::operation::delete_item::builders::DeleteItemFluentBuilder::set_return_values_on_condition_check_failure):<br>required: **false**<br><p>An optional parameter that returns the item attributes for a <code>DeleteItem</code> operation that failed a condition check.</p> <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p><br>
     /// - On success, responds with [`DeleteItemOutput`](crate::operation::delete_item::DeleteItemOutput) with field(s):
```

### `src/client/describe_continuous_backups.rs`

```diff
--- reference/src/client/describe_continuous_backups.rs
+++ generated/src/client/describe_continuous_backups.rs
@@ -7,7 +7,9 @@
     /// - On success, responds with [`DescribeContinuousBackupsOutput`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput) with field(s):
     ///   - [`continuous_backups_description(Option<ContinuousBackupsDescription>)`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput::continuous_backups_description): <p>Represents the continuous backups and point in time recovery settings on the table.</p>
     /// - On failure, responds with [`SdkError<DescribeContinuousBackupsError>`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsError)
-    pub fn describe_continuous_backups(&self) -> super::super::operation::describe_continuous_backups::builders::DescribeContinuousBackupsFluentBuilder {
+    pub fn describe_continuous_backups(
+        &self,
+    ) -> super::super::operation::describe_continuous_backups::builders::DescribeContinuousBackupsFluentBuilder {
         super::super::operation::describe_continuous_backups::builders::DescribeContinuousBackupsFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/describe_kinesis_streaming_destination.rs`

```diff
--- reference/src/client/describe_kinesis_streaming_destination.rs
+++ generated/src/client/describe_kinesis_streaming_destination.rs
@@ -11,6 +11,8 @@
     pub fn describe_kinesis_streaming_destination(
         &self,
     ) -> super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationFluentBuilder {
-        super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationFluentBuilder::new(self.handle.clone())
+        super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/disable_kinesis_streaming_destination.rs`

```diff
--- reference/src/client/disable_kinesis_streaming_destination.rs
+++ generated/src/client/disable_kinesis_streaming_destination.rs
@@ -15,6 +15,8 @@
     pub fn disable_kinesis_streaming_destination(
         &self,
     ) -> super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder {
-        super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::new(self.handle.clone())
+        super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/enable_kinesis_streaming_destination.rs`

```diff
--- reference/src/client/enable_kinesis_streaming_destination.rs
+++ generated/src/client/enable_kinesis_streaming_destination.rs
@@ -15,6 +15,8 @@
     pub fn enable_kinesis_streaming_destination(
         &self,
     ) -> super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder {
-        super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::new(self.handle.clone())
+        super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/export_table_to_point_in_time.rs`

```diff
--- reference/src/client/export_table_to_point_in_time.rs
+++ generated/src/client/export_table_to_point_in_time.rs
@@ -17,7 +17,9 @@
     /// - On success, responds with [`ExportTableToPointInTimeOutput`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput) with field(s):
     ///   - [`export_description(Option<ExportDescription>)`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput::export_description): <p>Contains a description of the table export.</p>
     /// - On failure, responds with [`SdkError<ExportTableToPointInTimeError>`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError)
-    pub fn export_table_to_point_in_time(&self) -> super::super::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder {
+    pub fn export_table_to_point_in_time(
+        &self,
+    ) -> super::super::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder {
         super::super::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/get_item.rs`

```diff
--- reference/src/client/get_item.rs
+++ generated/src/client/get_item.rs
@@ -9,7 +9,7 @@
     ///   - [`consistent_read(bool)`](crate::operation::get_item::builders::GetItemFluentBuilder::consistent_read) / [`set_consistent_read(Option<bool>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_consistent_read):<br>required: **false**<br><p>Determines the read consistency model: If set to <code>true</code>, then the operation uses strongly consistent reads; otherwise, the operation uses eventually consistent reads.</p><br>
     ///   - [`return_consumed_capacity(ReturnConsumedCapacity)`](crate::operation::get_item::builders::GetItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<ReturnConsumedCapacity>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_return_consumed_capacity):<br>required: **false**<br><p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p> <ul>  <li>   <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>   <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>  <li>   <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>  <li>   <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li> </ul><br>
     ///   - [`projection_expression(impl Into<String>)`](crate::operation::get_item::builders::GetItemFluentBuilder::projection_expression) / [`set_projection_expression(Option<String>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_projection_expression):<br>required: **false**<br><p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::get_item::builders::GetItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::get_item::builders::GetItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     /// - On success, responds with [`GetItemOutput`](crate::operation::get_item::GetItemOutput) with field(s):
     ///   - [`item(Option<HashMap::<String, AttributeValue>>)`](crate::operation::get_item::GetItemOutput::item): <p>A map of attribute names to <code>AttributeValue</code> objects, as specified by <code>ProjectionExpression</code>.</p>
     ///   - [`consumed_capacity(Option<ConsumedCapacity>)`](crate::operation::get_item::GetItemOutput::consumed_capacity): <p>The capacity units consumed by the <code>GetItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#read-operation-consumption">Capacity unit consumption for read operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
```

### `src/client/put_item.rs`

```diff
--- reference/src/client/put_item.rs
+++ generated/src/client/put_item.rs
@@ -11,7 +11,7 @@
     ///   - [`return_item_collection_metrics(ReturnItemCollectionMetrics)`](crate::operation::put_item::builders::PutItemFluentBuilder::return_item_collection_metrics) / [`set_return_item_collection_metrics(Option<ReturnItemCollectionMetrics>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_return_item_collection_metrics):<br>required: **false**<br><p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p><br>
     ///   - [`conditional_operator(ConditionalOperator)`](crate::operation::put_item::builders::PutItemFluentBuilder::conditional_operator) / [`set_conditional_operator(Option<ConditionalOperator>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_conditional_operator):<br>required: **false**<br><p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`condition_expression(impl Into<String>)`](crate::operation::put_item::builders::PutItemFluentBuilder::condition_expression) / [`set_condition_expression(Option<String>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_condition_expression):<br>required: **false**<br><p>A condition that must be satisfied in order for a conditional <code>PutItem</code> operation to succeed.</p> <p>An expression can contain any of the following:</p> <ul>  <li>   <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code></p>   <p>These function names are case-sensitive.</p></li>  <li>   <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code></p></li>  <li>   <p>Logical operators: <code>AND | OR | NOT</code></p></li> </ul> <p>For more information on condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::put_item::builders::PutItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::put_item::builders::PutItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`expression_attribute_values(impl Into<String>, AttributeValue)`](crate::operation::put_item::builders::PutItemFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap::<String, AttributeValue>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_expression_attribute_values):<br>required: **false**<br><p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following:</p> <p><code>Available | Backordered | Discontinued</code></p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p> <p>You could then use these values in an expression, such as this:</p> <p><code>ProductStatus IN (:avail, :back, :disc)</code></p> <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`return_values_on_condition_check_failure(ReturnValuesOnConditionCheckFailure)`](crate::operation::put_item::builders::PutItemFluentBuilder::return_values_on_condition_check_failure) / [`set_return_values_on_condition_check_failure(Option<ReturnValuesOnConditionCheckFailure>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_return_values_on_condition_check_failure):<br>required: **false**<br><p>An optional parameter that returns the item attributes for a <code>PutItem</code> operation that failed a condition check.</p> <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p><br>
     /// - On success, responds with [`PutItemOutput`](crate::operation::put_item::PutItemOutput) with field(s):
```

### `src/client/query.rs`

```diff
--- reference/src/client/query.rs
+++ generated/src/client/query.rs
@@ -19,7 +19,7 @@
     ///   - [`projection_expression(impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::projection_expression) / [`set_projection_expression(Option<String>)`](crate::operation::query::builders::QueryFluentBuilder::set_projection_expression):<br>required: **false**<br><p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`filter_expression(impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::filter_expression) / [`set_filter_expression(Option<String>)`](crate::operation::query::builders::QueryFluentBuilder::set_filter_expression):<br>required: **false**<br><p>A string that contains conditions that DynamoDB applies after the <code>Query</code> operation, but before the data is returned to you. Items that do not satisfy the <code>FilterExpression</code> criteria are not returned.</p> <p>A <code>FilterExpression</code> does not allow key attributes. You cannot define a filter expression based on a partition key or a sort key.</p><note>  <p>A <code>FilterExpression</code> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.FilterExpression.html">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`key_condition_expression(impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::key_condition_expression) / [`set_key_condition_expression(Option<String>)`](crate::operation::query::builders::QueryFluentBuilder::set_key_condition_expression):<br>required: **false**<br><p>The condition that specifies the key values for items to be retrieved by the <code>Query</code> action.</p> <p>The condition must perform an equality test on a single partition key value.</p> <p>The condition can optionally perform one of several comparison tests on a single sort key value. This allows <code>Query</code> to retrieve one item with a given partition key value and sort key value, or several items that have the same partition key value but different sort key values.</p> <p>The partition key equality test is required, and must be specified in the following format:</p> <p><code>partitionKeyName</code> <i>=</i> <code>:partitionkeyval</code></p> <p>If you also want to provide a condition for the sort key, it must be combined using <code>AND</code> with the condition for the sort key. Following is an example, using the <b>=</b> comparison operator for the sort key:</p> <p><code>partitionKeyName</code> <code>=</code> <code>:partitionkeyval</code> <code>AND</code> <code>sortKeyName</code> <code>=</code> <code>:sortkeyval</code></p> <p>Valid comparisons for the sort key condition are as follows:</p> <ul>  <li>   <p><code>sortKeyName</code> <code>=</code> <code>:sortkeyval</code> - true if the sort key value is equal to <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>&lt;</code> <code>:sortkeyval</code> - true if the sort key value is less than <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>&lt;=</code> <code>:sortkeyval</code> - true if the sort key value is less than or equal to <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>&gt;</code> <code>:sortkeyval</code> - true if the sort key value is greater than <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>&gt;= </code> <code>:sortkeyval</code> - true if the sort key value is greater than or equal to <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>BETWEEN</code> <code>:sortkeyval1</code> <code>AND</code> <code>:sortkeyval2</code> - true if the sort key value is greater than or equal to <code>:sortkeyval1</code>, and less than or equal to <code>:sortkeyval2</code>.</p></li>  <li>   <p><code>begins_with (</code> <code>sortKeyName</code>, <code>:sortkeyval</code> <code>)</code> - true if the sort key value begins with a particular operand. (You cannot use this function with a sort key that is of type Number.) Note that the function name <code>begins_with</code> is case-sensitive.</p></li> </ul> <p>Use the <code>ExpressionAttributeValues</code> parameter to replace tokens such as <code>:partitionval</code> and <code>:sortval</code> with actual values at runtime.</p> <p>You can optionally use the <code>ExpressionAttributeNames</code> parameter to replace the names of the partition key and sort key with placeholder tokens. This option might be necessary if an attribute name conflicts with a DynamoDB reserved word. For example, the following <code>KeyConditionExpression</code> parameter causes an error because <i>Size</i> is a reserved word:</p> <ul>  <li>   <p><code>Size = :myval</code></p></li> </ul> <p>To work around this, define a placeholder (such a <code>#S</code>) to represent the attribute name <i>Size</i>. <code>KeyConditionExpression</code> then is as follows:</p> <ul>  <li>   <p><code>#S = :myval</code></p></li> </ul> <p>For a list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>For more information on <code>ExpressionAttributeNames</code> and <code>ExpressionAttributeValues</code>, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ExpressionPlaceholders.html">Using Placeholders for Attribute Names and Values</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::query::builders::QueryFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::query::builders::QueryFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`expression_attribute_values(impl Into<String>, AttributeValue)`](crate::operation::query::builders::QueryFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap::<String, AttributeValue>>)`](crate::operation::query::builders::QueryFluentBuilder::set_expression_attribute_values):<br>required: **false**<br><p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following:</p> <p><code>Available | Backordered | Discontinued</code></p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p> <p>You could then use these values in an expression, such as this:</p> <p><code>ProductStatus IN (:avail, :back, :disc)</code></p> <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     /// - On success, responds with [`QueryOutput`](crate::operation::query::QueryOutput) with field(s):
     ///   - [`items(Option<Vec::<HashMap::<String, AttributeValue>>>)`](crate::operation::query::QueryOutput::items): <p>An array of item attributes that match the query criteria. Each element in this array consists of an attribute name and the value for that attribute.</p>
```

### `src/client/scan.rs`

```diff
--- reference/src/client/scan.rs
+++ generated/src/client/scan.rs
@@ -17,7 +17,7 @@
     ///   - [`segment(i32)`](crate::operation::scan::builders::ScanFluentBuilder::segment) / [`set_segment(Option<i32>)`](crate::operation::scan::builders::ScanFluentBuilder::set_segment):<br>required: **false**<br><p>For a parallel <code>Scan</code> request, <code>Segment</code> identifies an individual segment to be scanned by an application worker.</p> <p>Segment IDs are zero-based, so the first segment is always 0. For example, if you want to use four application threads to scan a table or an index, then the first thread specifies a <code>Segment</code> value of 0, the second thread specifies 1, and so on.</p> <p>The value of <code>LastEvaluatedKey</code> returned from a parallel <code>Scan</code> request must be used as <code>ExclusiveStartKey</code> with the same segment ID in a subsequent <code>Scan</code> operation.</p> <p>The value for <code>Segment</code> must be greater than or equal to 0, and less than the value provided for <code>TotalSegments</code>.</p> <p>If you provide <code>Segment</code>, you must also provide <code>TotalSegments</code>.</p><br>
     ///   - [`projection_expression(impl Into<String>)`](crate::operation::scan::builders::ScanFluentBuilder::projection_expression) / [`set_projection_expression(Option<String>)`](crate::operation::scan::builders::ScanFluentBuilder::set_projection_expression):<br>required: **false**<br><p>A string that identifies one or more attributes to retrieve from the specified table or index. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`filter_expression(impl Into<String>)`](crate::operation::scan::builders::ScanFluentBuilder::filter_expression) / [`set_filter_expression(Option<String>)`](crate::operation::scan::builders::ScanFluentBuilder::set_filter_expression):<br>required: **false**<br><p>A string that contains conditions that DynamoDB applies after the <code>Scan</code> operation, but before the data is returned to you. Items that do not satisfy the <code>FilterExpression</code> criteria are not returned.</p><note>  <p>A <code>FilterExpression</code> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.FilterExpression">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::scan::builders::ScanFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::scan::builders::ScanFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::scan::builders::ScanFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::scan::builders::ScanFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`expression_attribute_values(impl Into<String>, AttributeValue)`](crate::operation::scan::builders::ScanFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap::<String, AttributeValue>>)`](crate::operation::scan::builders::ScanFluentBuilder::set_expression_attribute_values):<br>required: **false**<br><p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <code>ProductStatus</code> attribute was one of the following:</p> <p><code>Available | Backordered | Discontinued</code></p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p> <p>You could then use these values in an expression, such as this:</p> <p><code>ProductStatus IN (:avail, :back, :disc)</code></p> <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`consistent_read(bool)`](crate::operation::scan::builders::ScanFluentBuilder::consistent_read) / [`set_consistent_read(Option<bool>)`](crate::operation::scan::builders::ScanFluentBuilder::set_consistent_read):<br>required: **false**<br><p>A Boolean value that determines the read consistency model during the scan:</p> <ul>  <li>   <p>If <code>ConsistentRead</code> is <code>false</code>, then the data returned from <code>Scan</code> might not contain the results from other recently completed write operations (<code>PutItem</code>, <code>UpdateItem</code>, or <code>DeleteItem</code>).</p></li>  <li>   <p>If <code>ConsistentRead</code> is <code>true</code>, then all of the write operations that completed before the <code>Scan</code> began are guaranteed to be contained in the <code>Scan</code> response.</p></li> </ul> <p>The default setting for <code>ConsistentRead</code> is <code>false</code>.</p> <p>The <code>ConsistentRead</code> parameter is not supported on global secondary indexes. If you scan a global secondary index with <code>ConsistentRead</code> set to true, you will receive a <code>ValidationException</code>.</p><br>
     /// - On success, responds with [`ScanOutput`](crate::operation::scan::ScanOutput) with field(s):
```

### `src/client/update_contributor_insights.rs`

```diff
--- reference/src/client/update_contributor_insights.rs
+++ generated/src/client/update_contributor_insights.rs
@@ -13,7 +13,9 @@
     ///   - [`contributor_insights_status(Option<ContributorInsightsStatus>)`](crate::operation::update_contributor_insights::UpdateContributorInsightsOutput::contributor_insights_status): <p>The status of contributor insights</p>
     ///   - [`contributor_insights_mode(Option<ContributorInsightsMode>)`](crate::operation::update_contributor_insights::UpdateContributorInsightsOutput::contributor_insights_mode): <p>The updated mode of CloudWatch Contributor Insights that determines whether to monitor all access and throttled events or to track throttled events exclusively.</p>
     /// - On failure, responds with [`SdkError<UpdateContributorInsightsError>`](crate::operation::update_contributor_insights::UpdateContributorInsightsError)
-    pub fn update_contributor_insights(&self) -> super::super::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder {
+    pub fn update_contributor_insights(
+        &self,
+    ) -> super::super::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder {
         super::super::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/update_global_table_settings.rs`

```diff
--- reference/src/client/update_global_table_settings.rs
+++ generated/src/client/update_global_table_settings.rs
@@ -13,7 +13,9 @@
     ///   - [`global_table_name(Option<String>)`](crate::operation::update_global_table_settings::UpdateGlobalTableSettingsOutput::global_table_name): <p>The name of the global table.</p>
     ///   - [`replica_settings(Option<Vec::<ReplicaSettingsDescription>>)`](crate::operation::update_global_table_settings::UpdateGlobalTableSettingsOutput::replica_settings): <p>The Region-specific settings for the global table.</p>
     /// - On failure, responds with [`SdkError<UpdateGlobalTableSettingsError>`](crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError)
-    pub fn update_global_table_settings(&self) -> super::super::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder {
+    pub fn update_global_table_settings(
+        &self,
+    ) -> super::super::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder {
         super::super::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/update_item.rs`

```diff
--- reference/src/client/update_item.rs
+++ generated/src/client/update_item.rs
@@ -11,9 +11,9 @@
     ///   - [`return_values(ReturnValue)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::return_values) / [`set_return_values(Option<ReturnValue>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_return_values):<br>required: **false**<br><p>Use <code>ReturnValues</code> if you want to get the item attributes as they appear before or after they are successfully updated. For <code>UpdateItem</code>, the valid values are:</p> <ul>  <li>   <p><code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p></li>  <li>   <p><code>ALL_OLD</code> - Returns all of the attributes of the item, as they appeared before the UpdateItem operation.</p></li>  <li>   <p><code>UPDATED_OLD</code> - Returns only the updated attributes, as they appeared before the UpdateItem operation.</p></li>  <li>   <p><code>ALL_NEW</code> - Returns all of the attributes of the item, as they appear after the UpdateItem operation.</p></li>  <li>   <p><code>UPDATED_NEW</code> - Returns only the updated attributes, as they appear after the UpdateItem operation.</p></li> </ul> <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p> <p>The values returned are strongly consistent.</p><br>
     ///   - [`return_consumed_capacity(ReturnConsumedCapacity)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<ReturnConsumedCapacity>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_return_consumed_capacity):<br>required: **false**<br><p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p> <ul>  <li>   <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>   <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>  <li>   <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>  <li>   <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li> </ul><br>
     ///   - [`return_item_collection_metrics(ReturnItemCollectionMetrics)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::return_item_collection_metrics) / [`set_return_item_collection_metrics(Option<ReturnItemCollectionMetrics>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_return_item_collection_metrics):<br>required: **false**<br><p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p><br>
-    ///   - [`update_expression(impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::update_expression) / [`set_update_expression(Option<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_update_expression):<br>required: **false**<br><p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new values for them.</p> <p>The following action values are available for <code>UpdateExpression</code>.</p> <ul>  <li>   <p><code>SET</code> - Adds one or more attributes and values to an item. If any of these attributes already exist, they are replaced by the new values. You can also use <code>SET</code> to add or subtract from an attribute that is of type Number. For example: <code>SET myNum = myNum + :val</code></p>   <p><code>SET</code> supports the following functions:</p>   <ul>    <li>     <p><code>if_not_exists (path, operand)</code> - if the item does not contain an attribute at the specified path, then <code>if_not_exists</code> evaluates to operand; otherwise, it evaluates to path. You can use this function to avoid overwriting an attribute that may already be present in the item.</p></li>    <li>     <p><code>list_append (operand, operand)</code> - evaluates to a list with a new element added to it. You can append the new element to the start or the end of the list by reversing the order of the operands.</p></li>   </ul>   <p>These function names are case-sensitive.</p></li>  <li>   <p><code>REMOVE</code> - Removes one or more attributes from an item.</p></li>  <li>   <p><code>ADD</code> - Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p>   <ul>    <li>     <p>If the existing attribute is a number, and if <code>Value</code> is also a number, then <code>Value</code> is mathematically added to the existing attribute. If <code>Value</code> is a negative number, then it is subtracted from the existing attribute.</p><note>      <p>If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value.</p>      <p>Similarly, if you use <code>ADD</code> for an existing item to increment or decrement an attribute value that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update doesn't have an attribute named <code>itemcount</code>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway. DynamoDB will create the <code>itemcount</code> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <code>itemcount</code> attribute in the item, with a value of <code>3</code>.</p>     </note></li>    <li>     <p>If the existing data type is a set and if <code>Value</code> is also a set, then <code>Value</code> is added to the existing set. For example, if the attribute value is the set <code>\[1,2\]</code>, and the <code>ADD</code> action specified <code>\[3\]</code>, then the final attribute value is <code>\[1,2,3\]</code>. An error occurs if an <code>ADD</code> action is specified for a set attribute and the attribute type specified does not match the existing set type.</p>     <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <code>Value</code> must also be a set of strings.</p></li>   </ul><important>    <p>The <code>ADD</code> action only supports Number and set data types.</p>   </important></li>  <li>   <p><code>DELETE</code> - Deletes an element from a set.</p>   <p>If a set of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>\[a,b,c\]</code> and the <code>DELETE</code> action specifies <code>\[a,c\]</code>, then the final attribute value is <code>\[b\]</code>. Specifying an empty set is an error.</p><important>    <p>The <code>DELETE</code> action only supports set data types.</p>   </important></li> </ul> <p>You can have many actions in a single expression, such as the following: <code>SET a=:value1, b=:value2 DELETE :value3, :value4, :value5</code></p> <p>For more information on update expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Modifying.html">Modifying Items and Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`update_expression(impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::update_expression) / [`set_update_expression(Option<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_update_expression):<br>required: **false**<br><p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new values for them.</p> <p>The following action values are available for <code>UpdateExpression</code>.</p> <ul>  <li>   <p><code>SET</code> - Adds one or more attributes and values to an item. If any of these attributes already exist, they are replaced by the new values. You can also use <code>SET</code> to add or subtract from an attribute that is of type Number. For example: <code>SET myNum = myNum + :val</code></p>   <p><code>SET</code> supports the following functions:</p>   <ul>    <li>     <p><code>if_not_exists (path, operand)</code> - if the item does not contain an attribute at the specified path, then <code>if_not_exists</code> evaluates to operand; otherwise, it evaluates to path. You can use this function to avoid overwriting an attribute that may already be present in the item.</p></li>    <li>     <p><code>list_append (operand, operand)</code> - evaluates to a list with a new element added to it. You can append the new element to the start or the end of the list by reversing the order of the operands.</p></li>   </ul>   <p>These function names are case-sensitive.</p></li>  <li>   <p><code>REMOVE</code> - Removes one or more attributes from an item.</p></li>  <li>   <p><code>ADD</code> - Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p>   <ul>    <li>     <p>If the existing attribute is a number, and if <code>Value</code> is also a number, then <code>Value</code> is mathematically added to the existing attribute. If <code>Value</code> is a negative number, then it is subtracted from the existing attribute.</p><note>      <p>If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value.</p>      <p>Similarly, if you use <code>ADD</code> for an existing item to increment or decrement an attribute value that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update doesn't have an attribute named <code>itemcount</code>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway. DynamoDB will create the <code>itemcount</code> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <code>itemcount</code> attribute in the item, with a value of <code>3</code>.</p>     </note></li>    <li>     <p>If the existing data type is a set and if <code>Value</code> is also a set, then <code>Value</code> is added to the existing set. For example, if the attribute value is the set <code>\[1,2\]</code>, and the <code>ADD</code> action specified <code>\[3\]</code>, then the final attribute value is <code>\[1,2,3\]</code>. An error occurs if an <code>ADD</code> action is specified for a set attribute and the attribute type specified does not match the existing set type.</p>     <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <code>Value</code> must also be a set of strings.</p></li>   </ul> <important>    <p>The <code>ADD</code> action only supports Number and set data types.</p>   </important></li>  <li>   <p><code>DELETE</code> - Deletes an element from a set.</p>   <p>If a set of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>\[a,b,c\]</code> and the <code>DELETE</code> action specifies <code>\[a,c\]</code>, then the final attribute value is <code>\[b\]</code>. Specifying an empty set is an error.</p><important>    <p>The <code>DELETE</code> action only supports set data types.</p>   </important></li> </ul> <p>You can have many actions in a single expression, such as the following: <code>SET a=:value1, b=:value2 DELETE :value3, :value4, :value5</code></p> <p>For more information on update expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Modifying.html">Modifying Items and Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`condition_expression(impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::condition_expression) / [`set_condition_expression(Option<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_condition_expression):<br>required: **false**<br><p>A condition that must be satisfied in order for a conditional update to succeed.</p> <p>An expression can contain any of the following:</p> <ul>  <li>   <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code></p>   <p>These function names are case-sensitive.</p></li>  <li>   <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code></p></li>  <li>   <p>Logical operators: <code>AND | OR | NOT</code></p></li> </ul> <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.) To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.) To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`expression_attribute_values(impl Into<String>, AttributeValue)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap::<String, AttributeValue>>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_expression_attribute_values):<br>required: **false**<br><p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <code>ProductStatus</code> attribute was one of the following:</p> <p><code>Available | Backordered | Discontinued</code></p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p> <p>You could then use these values in an expression, such as this:</p> <p><code>ProductStatus IN (:avail, :back, :disc)</code></p> <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`return_values_on_condition_check_failure(ReturnValuesOnConditionCheckFailure)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::return_values_on_condition_check_failure) / [`set_return_values_on_condition_check_failure(Option<ReturnValuesOnConditionCheckFailure>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_return_values_on_condition_check_failure):<br>required: **false**<br><p>An optional parameter that returns the item attributes for an <code>UpdateItem</code> operation that failed a condition check.</p> <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p><br>
     /// - On success, responds with [`UpdateItemOutput`](crate::operation::update_item::UpdateItemOutput) with field(s):
```

### `src/client/update_kinesis_streaming_destination.rs`

```diff
--- reference/src/client/update_kinesis_streaming_destination.rs
+++ generated/src/client/update_kinesis_streaming_destination.rs
@@ -15,6 +15,8 @@
     pub fn update_kinesis_streaming_destination(
         &self,
     ) -> super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder {
-        super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::new(self.handle.clone())
+        super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/config/endpoint.rs`

```diff
--- reference/src/config/endpoint.rs
+++ generated/src/config/endpoint.rs
@@ -29,7 +29,10 @@
 /// Endpoint resolver trait specific to Amazon DynamoDB
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a super::super::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a super::super::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -153,422 +156,396 @@
             match current_ref {
                 ref_val if ref_val >= 100_000_000 => {
                     return match (ref_val - 100_000_000) as usize {
-                                        0 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched")) as ::aws_smithy_runtime_api::box_error::BoxError),
-1 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: FIPS and custom endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-2 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Dualstack and custom endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-3 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Endpoint override is not supported for dual-stack endpoints. Please enable dual-stack functionality by enabling the configuration. For more details, see: https://docs.aws.amazon.com/sdkref/latest/guide/feature-endpoints.html"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-4 => {
+                        0 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "No endpoint rule matched",
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        1 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        2 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        3 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Endpoint override is not supported for dual-stack endpoints. Please enable dual-stack functionality by enabling the configuration. For more details, see: https://docs.aws.amazon.com/sdkref/latest/guide/feature-endpoints.html".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        4 => {
                             let endpoint = params.endpoint.as_deref().unwrap_or_default();
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url(endpoint.to_owned())
-.build())
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&endpoint.as_ref());
+                                        out
+                                    }).build())
                         },
-5 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: FIPS and local endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-6 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Dualstack and local endpoint are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-7 => {
-
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url("http://localhost:8000"
-.to_string())
-.auth_scheme(::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity("sigv4"
-.to_string(), 2)
-.put("signingName", "dynamodb")
-.put("signingRegion", "us-east-1")
-)
-.build())
-                        },
-8 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: AccountIdEndpointMode is required and FIPS is enabled, but FIPS account endpoints are not supported"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
+                        5 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: FIPS and local endpoint are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        6 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: Dualstack and local endpoint are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        7 => {
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url("http://localhost:8000".to_string()).auth_scheme(
+                                ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity("sigv4".to_string(), 2)
+                                    .put("signingName", "dynamodb")
+                                    .put("signingRegion", "us-east-1")
+                            ).build())
                         },
-9 => {
+                        8 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: AccountIdEndpointMode is required and FIPS is enabled, but FIPS account endpoints are not supported".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        9 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://search-dynamodb-fips.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://search-dynamodb-fips.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).build())
                         },
-10 => {
+                        10 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://dynamodb-fips.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.build())
-                        },
-11 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS and DualStack are enabled, but this partition does not support one or both"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://dynamodb-fips.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).build())
                         },
-12 => {
+                        11 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "FIPS and DualStack are enabled, but this partition does not support one or both".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        12 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://search-dynamodb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://search-dynamodb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).build())
                         },
-13 => {
+                        13 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://dynamodb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://dynamodb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).build())
                         },
-14 => {
+                        14 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://search-dynamodb-fips.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://search-dynamodb-fips.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).build())
                         },
-15 => {
+                        15 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://dynamodb-fips.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://dynamodb-fips.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).build())
                         },
-16 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS is enabled but this partition does not support FIPS"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-17 => {
+                        16 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "FIPS is enabled but this partition does not support FIPS".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        17 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_2.account_id());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_2.account_id());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-18 => {
+                        18 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_2.account_id());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_2.account_id());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-19 => {
+                        19 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_1.account_id());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_1.account_id());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-20 => {
+                        20 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_1.account_id());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_1.account_id());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-21 => {
+                        21 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let account_id = params.account_id.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&account_id.as_ref());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let account_id = params.account_id.as_deref().unwrap_or_default();
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&account_id.as_ref());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-22 => {
+                        22 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let account_id = params.account_id.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&account_id.as_ref());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
-                        },
-23 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Credentials-sourced account ID parameter is invalid"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-24 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("AccountIdEndpointMode is required but no AccountID was provided or able to be loaded"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
+                            let account_id = params.account_id.as_deref().unwrap_or_default();
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&account_id.as_ref());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-25 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: AccountIdEndpointMode is required but account endpoints are not supported in this partition"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-26 => {
+                        23 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Credentials-sourced account ID parameter is invalid".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        24 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "AccountIdEndpointMode is required but no AccountID was provided or able to be loaded".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        25 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: AccountIdEndpointMode is required but account endpoints are not supported in this partition".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        26 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://search-dynamodb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://search-dynamodb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).build())
                         },
-27 => {
+                        27 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://dynamodb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dual_stack_dns_suffix());
-out })
-.build())
-                        },
-28 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("DualStack is enabled but this partition does not support DualStack"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://dynamodb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dual_stack_dns_suffix());
+                                        out
+                                    }).build())
                         },
-29 => {
+                        28 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "DualStack is enabled but this partition does not support DualStack".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        29 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_2.account_id());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_2.account_id());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-30 => {
+                        30 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_2.account_id());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_2 = context.parsed_arn_ssa_2.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_2.account_id());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-31 => {
+                        31 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_1.account_id());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_1.account_id());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-32 => {
+                        32 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&parsed_arn_ssa_1.account_id());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let parsed_arn_ssa_1 = context.parsed_arn_ssa_1.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&parsed_arn_ssa_1.account_id());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-33 => {
+                        33 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let account_id = params.account_id.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&account_id.as_ref());
-out.push_str(".search-ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
+                            let account_id = params.account_id.as_deref().unwrap_or_default();
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&account_id.as_ref());
+                                        out.push_str(".search-ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-34 => {
+                        34 => {
                             let region = params.region.as_deref().unwrap_or_default();
-let account_id = params.account_id.as_deref().unwrap_or_default();
-let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
-out.push_str("https://");
-#[allow(clippy::needless_borrow)]
-out.push_str(&account_id.as_ref());
-out.push_str(".ddb.");
-#[allow(clippy::needless_borrow)]
-out.push_str(&region.as_ref());
-out.push_str(".");
-#[allow(clippy::needless_borrow)]
-out.push_str(&partition_result.dns_suffix());
-out })
-.property("metricValues", vec![::aws_smithy_types::Document::from("O"
-.to_string()),])
-.build())
-                        },
-35 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Missing Region"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
+                            let account_id = params.account_id.as_deref().unwrap_or_default();
+                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        out.push_str("https://");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&account_id.as_ref());
+                                        out.push_str(".ddb.");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&region.as_ref());
+                                        out.push_str(".");
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&partition_result.dns_suffix());
+                                        out
+                                    }).property("metricValues", vec![::aws_smithy_types::Document::from("O".to_string())]).build())
                         },
-                                        _ => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched")) as ::aws_smithy_runtime_api::box_error::BoxError),
-                                    };
+                        35 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "Invalid Configuration: Missing Region".to_string(),
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        _ => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "No endpoint rule matched",
+                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                    };
                 }
                 1 | -1 => {
                     return ::std::result::Result::Err(
@@ -626,10 +603,8 @@
                             (&{
                                 let mut out = String::new();
                                 out.push_str("dynamodb.");
-                                #[allow(clippy::needless_borrow)]
                                 out.push_str(&region.as_deref().unwrap_or_default());
                                 out.push_str(".");
-                                #[allow(clippy::needless_borrow)]
                                 out.push_str(&if let Some(inner) = partition_result {
                                     inner.dual_stack_dns_suffix()
                                 } else {
@@ -658,10 +633,8 @@
                             (&{
                                 let mut out = String::new();
                                 out.push_str("search-dynamodb.");
-                                #[allow(clippy::needless_borrow)]
                                 out.push_str(&region.as_deref().unwrap_or_default());
                                 out.push_str(".");
-                                #[allow(clippy::needless_borrow)]
                                 out.push_str(&if let Some(inner) = partition_result {
                                     inner.dual_stack_dns_suffix()
                                 } else {
@@ -704,7 +677,7 @@
                         16 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_2 = &context.parsed_arn_ssa_2;
                             let partition_resolver = &self.partition_resolver;
-                            &mut Some(
+                            (&mut Some(
                                 (if let Some(inner) = parsed_arn_ssa_2 {
                                     inner.region()
                                 } else {
@@ -711,7 +684,7 @@
                                     return false;
                                 }
                                 .into()),
-                            ) == (region)
+                            )) == region
                         })(&mut _diagnostic_collector),
                         17 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_2 = &context.parsed_arn_ssa_2;
@@ -754,10 +727,11 @@
                             let partition_resolver = &self.partition_resolver;
                             {
                                 *first_arn = if let Some(inner) = resource_arn_list {
-                                    inner.first().map(|s| s.as_str())
+                                    inner.first().cloned()
                                 } else {
                                     return false;
-                                };
+                                }
+                                .map(|inner| inner.into());
                                 first_arn.is_some()
                             }
                         })(&mut _diagnostic_collector),
@@ -777,7 +751,7 @@
                         23 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_1 = &context.parsed_arn_ssa_1;
                             let partition_resolver = &self.partition_resolver;
-                            &mut Some(
+                            (&mut Some(
                                 (if let Some(inner) = parsed_arn_ssa_1 {
                                     inner.region()
                                 } else {
@@ -784,7 +758,7 @@
                                     return false;
                                 }
                                 .into()),
-                            ) == (region)
+                            )) == region
                         })(&mut _diagnostic_collector),
                         24 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_1 = &context.parsed_arn_ssa_1;
@@ -857,7 +831,10 @@
 }

 impl super::super::config::endpoint::ResolveEndpoint for DefaultResolver {
-    fn resolve_endpoint<'a>(&'a self, params: &'a super::super::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a super::super::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
         // Check single-entry cache (lock-free read via ArcSwap)
         let cached = self.endpoint_cache.load();
         if let Some((cached_params, cached_endpoint)) = cached.as_ref() {
```

### `src/operation/batch_execute_statement/_batch_execute_statement_input.rs`

```diff
--- reference/src/operation/batch_execute_statement/_batch_execute_statement_input.rs
+++ generated/src/operation/batch_execute_statement/_batch_execute_statement_input.rs
@@ -117,8 +117,10 @@
     /// Consumes the builder and constructs a [`BatchExecuteStatementInput`](crate::operation::batch_execute_statement::BatchExecuteStatementInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::batch_execute_statement::BatchExecuteStatementInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::batch_execute_statement::BatchExecuteStatementInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::batch_execute_statement::BatchExecuteStatementInput {
             statements: self.statements,
             return_consumed_capacity: self.return_consumed_capacity,
```

### `src/operation/batch_execute_statement.rs`

```diff
--- reference/src/operation/batch_execute_statement.rs
+++ generated/src/operation/batch_execute_statement.rs
@@ -212,9 +212,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_batch_execute_statement::ser_batch_execute_statement_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_batch_execute_statement::ser_batch_execute_statement_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/batch_get_item/_batch_get_item_input.rs`

```diff
--- reference/src/operation/batch_get_item/_batch_get_item_input.rs
+++ generated/src/operation/batch_get_item/_batch_get_item_input.rs
@@ -102,7 +102,9 @@
     /// <li>
     /// <p><code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>
     /// </ul>
-    pub fn request_items(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>> {
+    pub fn request_items(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>> {
         self.request_items.as_ref()
     }
     /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
@@ -130,7 +132,8 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct BatchGetItemInputBuilder {
-    pub(crate) request_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>>,
+    pub(crate) request_items:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>>,
     pub(crate) return_consumed_capacity: ::std::option::Option<super::super::super::types::ReturnConsumedCapacity>,
 }
 impl BatchGetItemInputBuilder {
@@ -277,7 +280,9 @@
     /// <li>
     /// <p><code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>
     /// </ul>
-    pub fn get_request_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>> {
+    pub fn get_request_items(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>> {
         &self.request_items
     }
     /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
@@ -324,7 +329,8 @@
     /// Consumes the builder and constructs a [`BatchGetItemInput`](crate::operation::batch_get_item::BatchGetItemInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::batch_get_item::BatchGetItemInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::super::operation::batch_get_item::BatchGetItemInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::super::operation::batch_get_item::BatchGetItemInput {
             request_items: self.request_items,
             return_consumed_capacity: self.return_consumed_capacity,
```

### `src/operation/batch_get_item/_batch_get_item_output.rs`

```diff
--- reference/src/operation/batch_get_item/_batch_get_item_output.rs
+++ generated/src/operation/batch_get_item/_batch_get_item_output.rs
@@ -57,7 +57,9 @@
     /// <p><code>ConsistentRead</code> - The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p></li>
     /// </ul>
     /// <p>If there are no unprocessed keys remaining, the response contains an empty <code>UnprocessedKeys</code> map.</p>
-    pub fn unprocessed_keys(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>> {
+    pub fn unprocessed_keys(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>> {
         self.unprocessed_keys.as_ref()
     }
     /// <p>The read capacity units consumed by the entire <code>BatchGetItem</code> operation.</p>
@@ -96,7 +98,8 @@
             ::std::vec::Vec<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
         >,
     >,
-    pub(crate) unprocessed_keys: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>>,
+    pub(crate) unprocessed_keys:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>>,
     pub(crate) consumed_capacity: ::std::option::Option<::std::vec::Vec<super::super::super::types::ConsumedCapacity>>,
     _request_id: Option<String>,
 }
```

### `src/operation/batch_get_item/builders.rs`

```diff
--- reference/src/operation/batch_get_item/builders.rs
+++ generated/src/operation/batch_get_item/builders.rs
@@ -264,7 +264,9 @@
     /// <li>
     /// <p><code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>
     /// </ul>
-    pub fn get_request_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>> {
+    pub fn get_request_items(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::KeysAndAttributes>> {
         self.inner.get_request_items()
     }
     /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
```

### `src/operation/batch_write_item/_batch_write_item_input.rs`

```diff
--- reference/src/operation/batch_write_item/_batch_write_item_input.rs
+++ generated/src/operation/batch_write_item/_batch_write_item_input.rs
@@ -20,7 +20,8 @@
     /// <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p></li>
     /// </ul></li>
     /// </ul>
-    pub request_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::WriteRequest>>>,
+    pub request_items:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::WriteRequest>>>,
     /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
     /// <ul>
     /// <li>
@@ -86,7 +87,8 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct BatchWriteItemInputBuilder {
-    pub(crate) request_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::WriteRequest>>>,
+    pub(crate) request_items:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::WriteRequest>>>,
     pub(crate) return_consumed_capacity: ::std::option::Option<super::super::super::types::ReturnConsumedCapacity>,
     pub(crate) return_item_collection_metrics: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
 }
@@ -111,7 +113,11 @@
     /// <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p></li>
     /// </ul></li>
     /// </ul>
-    pub fn request_items(mut self, k: impl ::std::convert::Into<::std::string::String>, v: ::std::vec::Vec<super::super::super::types::WriteRequest>) -> Self {
+    pub fn request_items(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: ::std::vec::Vec<super::super::super::types::WriteRequest>,
+    ) -> Self {
         let mut hash_map = self.request_items.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.request_items = ::std::option::Option::Some(hash_map);
@@ -208,7 +214,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.return_item_collection_metrics = input;
         self
     }
@@ -219,7 +228,8 @@
     /// Consumes the builder and constructs a [`BatchWriteItemInput`](crate::operation::batch_write_item::BatchWriteItemInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::batch_write_item::BatchWriteItemInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::super::operation::batch_write_item::BatchWriteItemInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::super::operation::batch_write_item::BatchWriteItemInput {
             request_items: self.request_items,
             return_consumed_capacity: self.return_consumed_capacity,
```

### `src/operation/batch_write_item/_batch_write_item_output.rs`

```diff
--- reference/src/operation/batch_write_item/_batch_write_item_output.rs
+++ generated/src/operation/batch_write_item/_batch_write_item_output.rs
@@ -22,7 +22,8 @@
     /// </ul></li>
     /// </ul>
     /// <p>If there are no unprocessed items remaining, the response contains an empty <code>UnprocessedItems</code> map.</p>
-    pub unprocessed_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::WriteRequest>>>,
+    pub unprocessed_items:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::WriteRequest>>>,
     /// <p>A list of tables that were processed by <code>BatchWriteItem</code> and, for each table, information about any item collections that were affected by individual <code>DeleteItem</code> or <code>PutItem</code> operations.</p>
     /// <p>Each entry consists of the following subelements:</p>
     /// <ul>
@@ -81,7 +82,8 @@
     /// </ul>
     pub fn item_collection_metrics(
         &self,
-    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>> {
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>>
+    {
         self.item_collection_metrics.as_ref()
     }
     /// <p>The capacity units consumed by the entire <code>BatchWriteItem</code> operation.</p>
@@ -145,7 +147,11 @@
     /// </ul></li>
     /// </ul>
     /// <p>If there are no unprocessed items remaining, the response contains an empty <code>UnprocessedItems</code> map.</p>
-    pub fn unprocessed_items(mut self, k: impl ::std::convert::Into<::std::string::String>, v: ::std::vec::Vec<super::super::super::types::WriteRequest>) -> Self {
+    pub fn unprocessed_items(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: ::std::vec::Vec<super::super::super::types::WriteRequest>,
+    ) -> Self {
         let mut hash_map = self.unprocessed_items.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.unprocessed_items = ::std::option::Option::Some(hash_map);
@@ -233,7 +239,9 @@
     /// </ul>
     pub fn set_item_collection_metrics(
         mut self,
-        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>>,
+        input: ::std::option::Option<
+            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>,
+        >,
     ) -> Self {
         self.item_collection_metrics = input;
         self
@@ -249,7 +257,8 @@
     /// </ul>
     pub fn get_item_collection_metrics(
         &self,
-    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>> {
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>>
+    {
         &self.item_collection_metrics
     }
     /// Appends an item to `consumed_capacity`.
```

### `src/operation/batch_write_item/builders.rs`

```diff
--- reference/src/operation/batch_write_item/builders.rs
+++ generated/src/operation/batch_write_item/builders.rs
@@ -158,7 +158,11 @@
     /// <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p></li>
     /// </ul></li>
     /// </ul>
-    pub fn request_items(mut self, k: impl ::std::convert::Into<::std::string::String>, v: ::std::vec::Vec<super::super::super::types::WriteRequest>) -> Self {
+    pub fn request_items(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: ::std::vec::Vec<super::super::super::types::WriteRequest>,
+    ) -> Self {
         self.inner = self.inner.request_items(k.into(), v);
         self
     }
@@ -253,7 +257,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.inner = self.inner.set_return_item_collection_metrics(input);
         self
     }
```

### `src/operation/create_global_table/_create_global_table_input.rs`

```diff
--- reference/src/operation/create_global_table/_create_global_table_input.rs
+++ generated/src/operation/create_global_table/_create_global_table_input.rs
@@ -73,7 +73,10 @@
     /// Consumes the builder and constructs a [`CreateGlobalTableInput`](crate::operation::create_global_table::CreateGlobalTableInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::create_global_table::CreateGlobalTableInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_global_table::CreateGlobalTableInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::create_global_table::CreateGlobalTableInput {
             global_table_name: self.global_table_name,
             replication_group: self.replication_group,
```

### `src/operation/create_global_table.rs`

```diff
--- reference/src/operation/create_global_table.rs
+++ generated/src/operation/create_global_table.rs
@@ -258,7 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_global_table::ser_create_global_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_create_global_table::ser_create_global_table_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/create_table/_create_table_input.rs`

```diff
--- reference/src/operation/create_table/_create_table_input.rs
+++ generated/src/operation/create_table/_create_table_input.rs
@@ -390,7 +390,10 @@
         self
     }
     /// <p>An array of attributes that describe the key schema for the table and indexes.</p>
-    pub fn set_attribute_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeDefinition>>) -> Self {
+    pub fn set_attribute_definitions(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeDefinition>>,
+    ) -> Self {
         self.attribute_definitions = input;
         self
     }
@@ -550,7 +553,10 @@
     /// <p><code>NonKeyAttributes</code> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <code>NonKeyAttributes</code>, summed across all of the secondary indexes, must not exceed 100. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total. This limit only applies when you specify the ProjectionType of <code>INCLUDE</code>. You still can specify the ProjectionType of <code>ALL</code> to project all attributes from the source table, even if the table has more than 100 attributes.</p></li>
     /// </ul></li>
     /// </ul>
-    pub fn set_local_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>) -> Self {
+    pub fn set_local_secondary_indexes(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>,
+    ) -> Self {
         self.local_secondary_indexes = input;
         self
     }
@@ -644,7 +650,10 @@
     /// <li>
     /// <p><code>ProvisionedThroughput</code> - The provisioned throughput settings for the global secondary index, consisting of read and write capacity units.</p></li>
     /// </ul>
-    pub fn set_global_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>) -> Self {
+    pub fn set_global_secondary_indexes(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>,
+    ) -> Self {
         self.global_secondary_indexes = input;
         self
     }
@@ -935,7 +944,9 @@
         self
     }
     /// <p>Controls the settings synchronization mode for the global table. For multi-account global tables, this parameter is required and the only supported value is ENABLED. For same-account global tables, this parameter is set to ENABLED_WITH_OVERRIDES.</p>
-    pub fn get_global_table_settings_replication_mode(&self) -> &::std::option::Option<super::super::super::types::GlobalTableSettingsReplicationMode> {
+    pub fn get_global_table_settings_replication_mode(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::GlobalTableSettingsReplicationMode> {
         &self.global_table_settings_replication_mode
     }
     /// Appends an item to `vector_indexes`.
@@ -1001,7 +1012,9 @@
         &self.vector_indexes
     }
     /// Consumes the builder and constructs a [`CreateTableInput`](crate::operation::create_table::CreateTableInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::create_table::CreateTableInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_table::CreateTableInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::create_table::CreateTableInput {
             attribute_definitions: self.attribute_definitions,
             table_name: self.table_name,
```

### `src/operation/create_table/builders.rs`

```diff
--- reference/src/operation/create_table/builders.rs
+++ generated/src/operation/create_table/builders.rs
@@ -122,7 +122,10 @@
         self
     }
     /// <p>An array of attributes that describe the key schema for the table and indexes.</p>
-    pub fn set_attribute_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeDefinition>>) -> Self {
+    pub fn set_attribute_definitions(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeDefinition>>,
+    ) -> Self {
         self.inner = self.inner.set_attribute_definitions(input);
         self
     }
@@ -279,7 +282,10 @@
     /// <p><code>NonKeyAttributes</code> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <code>NonKeyAttributes</code>, summed across all of the secondary indexes, must not exceed 100. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total. This limit only applies when you specify the ProjectionType of <code>INCLUDE</code>. You still can specify the ProjectionType of <code>ALL</code> to project all attributes from the source table, even if the table has more than 100 attributes.</p></li>
     /// </ul></li>
     /// </ul>
-    pub fn set_local_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>) -> Self {
+    pub fn set_local_secondary_indexes(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>,
+    ) -> Self {
         self.inner = self.inner.set_local_secondary_indexes(input);
         self
     }
@@ -372,7 +378,10 @@
     /// <li>
     /// <p><code>ProvisionedThroughput</code> - The provisioned throughput settings for the global secondary index, consisting of read and write capacity units.</p></li>
     /// </ul>
-    pub fn set_global_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>) -> Self {
+    pub fn set_global_secondary_indexes(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>,
+    ) -> Self {
         self.inner = self.inner.set_global_secondary_indexes(input);
         self
     }
@@ -662,7 +671,9 @@
         self
     }
     /// <p>Controls the settings synchronization mode for the global table. For multi-account global tables, this parameter is required and the only supported value is ENABLED. For same-account global tables, this parameter is set to ENABLED_WITH_OVERRIDES.</p>
-    pub fn get_global_table_settings_replication_mode(&self) -> &::std::option::Option<super::super::super::types::GlobalTableSettingsReplicationMode> {
+    pub fn get_global_table_settings_replication_mode(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::GlobalTableSettingsReplicationMode> {
         self.inner.get_global_table_settings_replication_mode()
     }
     ///
```

### `src/operation/delete_item/_delete_item_input.rs`

```diff
--- reference/src/operation/delete_item/_delete_item_input.rs
+++ generated/src/operation/delete_item/_delete_item_input.rs
@@ -86,7 +86,8 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     /// <p>An optional parameter that returns the item attributes for a <code>DeleteItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
     pub return_values_on_condition_check_failure: ::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure>,
@@ -102,7 +103,9 @@
         self.key.as_ref()
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expected(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn expected(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         self.expected.as_ref()
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -199,7 +202,9 @@
     }
     /// <p>An optional parameter that returns the item attributes for a <code>DeleteItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn return_values_on_condition_check_failure(&self) -> ::std::option::Option<&super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn return_values_on_condition_check_failure(
+        &self,
+    ) -> ::std::option::Option<&super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         self.return_values_on_condition_check_failure.as_ref()
     }
 }
@@ -216,7 +221,8 @@
 pub struct DeleteItemInputBuilder {
     pub(crate) table_name: ::std::option::Option<::std::string::String>,
     pub(crate) key: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
-    pub(crate) expected: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>>,
+    pub(crate) expected:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>>,
     pub(crate) conditional_operator: ::std::option::Option<super::super::super::types::ConditionalOperator>,
     pub(crate) return_values: ::std::option::Option<super::super::super::types::ReturnValue>,
     pub(crate) return_consumed_capacity: ::std::option::Option<super::super::super::types::ReturnConsumedCapacity>,
@@ -223,7 +229,8 @@
     pub(crate) return_item_collection_metrics: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
     pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) return_values_on_condition_check_failure: ::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure>,
 }
 impl DeleteItemInputBuilder {
@@ -256,7 +263,10 @@
     }
     /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
     /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    ) -> Self {
         self.key = input;
         self
     }
@@ -285,7 +295,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn get_expected(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         &self.expected
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -390,7 +402,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.return_item_collection_metrics = input;
         self
     }
@@ -564,7 +579,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
@@ -615,11 +634,15 @@
     }
     /// <p>An optional parameter that returns the item attributes for a <code>DeleteItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn get_return_values_on_condition_check_failure(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         &self.return_values_on_condition_check_failure
     }
     /// Consumes the builder and constructs a [`DeleteItemInput`](crate::operation::delete_item::DeleteItemInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::delete_item::DeleteItemInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::delete_item::DeleteItemInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::delete_item::DeleteItemInput {
             table_name: self.table_name,
             key: self.key,
```

### `src/operation/delete_item/_delete_item_output.rs`

```diff
--- reference/src/operation/delete_item/_delete_item_output.rs
+++ generated/src/operation/delete_item/_delete_item_output.rs
@@ -23,7 +23,9 @@
 }
 impl DeleteItemOutput {
     /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the item as it appeared before the <code>DeleteItem</code> operation. This map appears in the response only if <code>ReturnValues</code> was specified as <code>ALL_OLD</code> in the request.</p>
-    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn attributes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         self.attributes.as_ref()
     }
     /// <p>The capacity units consumed by the <code>DeleteItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/provisioned-capacity-mode.html">Provisioned capacity mode</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -86,7 +88,9 @@
         self
     }
     /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the item as it appeared before the <code>DeleteItem</code> operation. This map appears in the response only if <code>ReturnValues</code> was specified as <code>ALL_OLD</code> in the request.</p>
-    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn get_attributes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         &self.attributes
     }
     /// <p>The capacity units consumed by the <code>DeleteItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/provisioned-capacity-mode.html">Provisioned capacity mode</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
```

### `src/operation/delete_item/builders.rs`

```diff
--- reference/src/operation/delete_item/builders.rs
+++ generated/src/operation/delete_item/builders.rs
@@ -138,7 +138,10 @@
     }
     /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
     /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    ) -> Self {
         self.inner = self.inner.set_key(input);
         self
     }
@@ -166,7 +169,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn get_expected(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         self.inner.get_expected()
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -271,7 +276,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.inner = self.inner.set_return_item_collection_metrics(input);
         self
     }
@@ -445,7 +453,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         self.inner = self.inner.expression_attribute_values(k.into(), v);
         self
     }
@@ -494,7 +506,9 @@
     }
     /// <p>An optional parameter that returns the item attributes for a <code>DeleteItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn get_return_values_on_condition_check_failure(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         self.inner.get_return_values_on_condition_check_failure()
     }
 }
```

### `src/operation/delete_resource_policy/_delete_resource_policy_input.rs`

```diff
--- reference/src/operation/delete_resource_policy/_delete_resource_policy_input.rs
+++ generated/src/operation/delete_resource_policy/_delete_resource_policy_input.rs
@@ -65,8 +65,10 @@
     /// Consumes the builder and constructs a [`DeleteResourcePolicyInput`](crate::operation::delete_resource_policy::DeleteResourcePolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::delete_resource_policy::DeleteResourcePolicyInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::delete_resource_policy::DeleteResourcePolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::delete_resource_policy::DeleteResourcePolicyInput {
             resource_arn: self.resource_arn,
             expected_revision_id: self.expected_revision_id,
```

### `src/operation/delete_resource_policy.rs`

```diff
--- reference/src/operation/delete_resource_policy.rs
+++ generated/src/operation/delete_resource_policy.rs
@@ -263,9 +263,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_delete_resource_policy::ser_delete_resource_policy_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_delete_resource_policy::ser_delete_resource_policy_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/delete_table/_delete_table_input.rs`

```diff
--- reference/src/operation/delete_table/_delete_table_input.rs
+++ generated/src/operation/delete_table/_delete_table_input.rs
@@ -43,7 +43,9 @@
         &self.table_name
     }
     /// Consumes the builder and constructs a [`DeleteTableInput`](crate::operation::delete_table::DeleteTableInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::delete_table::DeleteTableInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::delete_table::DeleteTableInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::delete_table::DeleteTableInput { table_name: self.table_name })
     }
 }
```

### `src/operation/describe_continuous_backups/_describe_continuous_backups_input.rs`

```diff
--- reference/src/operation/describe_continuous_backups/_describe_continuous_backups_input.rs
+++ generated/src/operation/describe_continuous_backups/_describe_continuous_backups_input.rs
@@ -53,6 +53,8 @@
         super::super::super::operation::describe_continuous_backups::DescribeContinuousBackupsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::describe_continuous_backups::DescribeContinuousBackupsInput { table_name: self.table_name })
+        ::std::result::Result::Ok(
+            super::super::super::operation::describe_continuous_backups::DescribeContinuousBackupsInput { table_name: self.table_name },
+        )
     }
 }
```

### `src/operation/describe_continuous_backups/_describe_continuous_backups_output.rs`

```diff
--- reference/src/operation/describe_continuous_backups/_describe_continuous_backups_output.rs
+++ generated/src/operation/describe_continuous_backups/_describe_continuous_backups_output.rs
@@ -39,7 +39,10 @@
         self
     }
     /// <p>Represents the continuous backups and point in time recovery settings on the table.</p>
-    pub fn set_continuous_backups_description(mut self, input: ::std::option::Option<super::super::super::types::ContinuousBackupsDescription>) -> Self {
+    pub fn set_continuous_backups_description(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ContinuousBackupsDescription>,
+    ) -> Self {
         self.continuous_backups_description = input;
         self
     }
```

### `src/operation/describe_contributor_insights/_describe_contributor_insights_input.rs`

```diff
--- reference/src/operation/describe_contributor_insights/_describe_contributor_insights_input.rs
+++ generated/src/operation/describe_contributor_insights/_describe_contributor_insights_input.rs
@@ -69,9 +69,11 @@
         super::super::super::operation::describe_contributor_insights::DescribeContributorInsightsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::describe_contributor_insights::DescribeContributorInsightsInput {
-            table_name: self.table_name,
-            index_name: self.index_name,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::describe_contributor_insights::DescribeContributorInsightsInput {
+                table_name: self.table_name,
+                index_name: self.index_name,
+            },
+        )
     }
 }
```

### `src/operation/describe_endpoints/_describe_endpoints_input.rs`

```diff
--- reference/src/operation/describe_endpoints/_describe_endpoints_input.rs
+++ generated/src/operation/describe_endpoints/_describe_endpoints_input.rs
@@ -18,7 +18,10 @@
     /// Consumes the builder and constructs a [`DescribeEndpointsInput`](crate::operation::describe_endpoints::DescribeEndpointsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::describe_endpoints::DescribeEndpointsInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::describe_endpoints::DescribeEndpointsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::describe_endpoints::DescribeEndpointsInput {})
     }
 }
```

### `src/operation/describe_endpoints/_describe_endpoints_output.rs`

```diff
--- reference/src/operation/describe_endpoints/_describe_endpoints_output.rs
+++ generated/src/operation/describe_endpoints/_describe_endpoints_output.rs
@@ -68,7 +68,10 @@
     /// - [`endpoints`](crate::operation::describe_endpoints::builders::DescribeEndpointsOutputBuilder::endpoints)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::describe_endpoints::DescribeEndpointsOutput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::describe_endpoints::DescribeEndpointsOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::describe_endpoints::DescribeEndpointsOutput {
             endpoints: self.endpoints.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/operation/describe_endpoints.rs`

```diff
--- reference/src/operation/describe_endpoints.rs
+++ generated/src/operation/describe_endpoints.rs
@@ -212,7 +212,7 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_describe_endpoints::ser_describe_endpoints_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/describe_global_table/_describe_global_table_input.rs`

```diff
--- reference/src/operation/describe_global_table/_describe_global_table_input.rs
+++ generated/src/operation/describe_global_table/_describe_global_table_input.rs
@@ -44,8 +44,10 @@
     /// Consumes the builder and constructs a [`DescribeGlobalTableInput`](crate::operation::describe_global_table::DescribeGlobalTableInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::describe_global_table::DescribeGlobalTableInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::describe_global_table::DescribeGlobalTableInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::describe_global_table::DescribeGlobalTableInput {
             global_table_name: self.global_table_name,
         })
```

### `src/operation/describe_global_table.rs`

```diff
--- reference/src/operation/describe_global_table.rs
+++ generated/src/operation/describe_global_table.rs
@@ -258,9 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_describe_global_table::ser_describe_global_table_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_describe_global_table::ser_describe_global_table_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/describe_global_table_settings/_describe_global_table_settings_input.rs`

```diff
--- reference/src/operation/describe_global_table_settings/_describe_global_table_settings_input.rs
+++ generated/src/operation/describe_global_table_settings/_describe_global_table_settings_input.rs
@@ -48,8 +48,10 @@
         super::super::super::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput {
-            global_table_name: self.global_table_name,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput {
+                global_table_name: self.global_table_name,
+            },
+        )
     }
 }
```

### `src/operation/describe_global_table_settings/_describe_global_table_settings_output.rs`

```diff
--- reference/src/operation/describe_global_table_settings/_describe_global_table_settings_output.rs
+++ generated/src/operation/describe_global_table_settings/_describe_global_table_settings_output.rs
@@ -68,7 +68,10 @@
         self
     }
     /// <p>The Region-specific settings for the global table.</p>
-    pub fn set_replica_settings(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsDescription>>) -> Self {
+    pub fn set_replica_settings(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsDescription>>,
+    ) -> Self {
         self.replica_settings = input;
         self
     }
```

### `src/operation/describe_kinesis_streaming_destination/_describe_kinesis_streaming_destination_input.rs`

```diff
--- reference/src/operation/describe_kinesis_streaming_destination/_describe_kinesis_streaming_destination_input.rs
+++ generated/src/operation/describe_kinesis_streaming_destination/_describe_kinesis_streaming_destination_input.rs
@@ -14,7 +14,8 @@
 }
 impl DescribeKinesisStreamingDestinationInput {
     /// Creates a new builder-style object to manufacture [`DescribeKinesisStreamingDestinationInput`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput).
-    pub fn builder() -> super::super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder {
+    pub fn builder(
+    ) -> super::super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder {
         super::super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder::default()
     }
 }
@@ -49,7 +50,9 @@
         ::aws_smithy_types::error::operation::BuildError,
     > {
         ::std::result::Result::Ok(
-            super::super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput { table_name: self.table_name },
+            super::super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput {
+                table_name: self.table_name,
+            },
         )
     }
 }
```

### `src/operation/describe_kinesis_streaming_destination/_describe_kinesis_streaming_destination_output.rs`

```diff
--- reference/src/operation/describe_kinesis_streaming_destination/_describe_kinesis_streaming_destination_output.rs
+++ generated/src/operation/describe_kinesis_streaming_destination/_describe_kinesis_streaming_destination_output.rs
@@ -28,7 +28,8 @@
 }
 impl DescribeKinesisStreamingDestinationOutput {
     /// Creates a new builder-style object to manufacture [`DescribeKinesisStreamingDestinationOutput`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput).
-    pub fn builder() -> super::super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationOutputBuilder {
+    pub fn builder(
+    ) -> super::super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationOutputBuilder {
         super::super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationOutputBuilder::default()
     }
 }
@@ -76,7 +77,9 @@
         self
     }
     /// <p>The list of replica structures for the table being described.</p>
-    pub fn get_kinesis_data_stream_destinations(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::KinesisDataStreamDestination>> {
+    pub fn get_kinesis_data_stream_destinations(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::KinesisDataStreamDestination>> {
         &self.kinesis_data_stream_destinations
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/describe_kinesis_streaming_destination/builders.rs`

```diff
--- reference/src/operation/describe_kinesis_streaming_destination/builders.rs
+++ generated/src/operation/describe_kinesis_streaming_destination/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the DescribeKinesisStreamingDestination as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -87,7 +89,11 @@
                 &self.handle.conf,
                 self.config_override,
             );
-        super::super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestination::orchestrate(&runtime_plugins, input).await
+        super::super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestination::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/describe_limits.rs`

```diff
--- reference/src/operation/describe_limits.rs
+++ generated/src/operation/describe_limits.rs
@@ -212,7 +212,7 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_describe_limits::ser_describe_limits_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/describe_table_replica_auto_scaling/_describe_table_replica_auto_scaling_output.rs`

```diff
--- reference/src/operation/describe_table_replica_auto_scaling/_describe_table_replica_auto_scaling_output.rs
+++ generated/src/operation/describe_table_replica_auto_scaling/_describe_table_replica_auto_scaling_output.rs
@@ -39,7 +39,10 @@
         self
     }
     /// <p>Represents the auto scaling properties of the table.</p>
-    pub fn set_table_auto_scaling_description(mut self, input: ::std::option::Option<super::super::super::types::TableAutoScalingDescription>) -> Self {
+    pub fn set_table_auto_scaling_description(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::TableAutoScalingDescription>,
+    ) -> Self {
         self.table_auto_scaling_description = input;
         self
     }
```

### `src/operation/describe_table_replica_auto_scaling/builders.rs`

```diff
--- reference/src/operation/describe_table_replica_auto_scaling/builders.rs
+++ generated/src/operation/describe_table_replica_auto_scaling/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the DescribeTableReplicaAutoScaling as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScaling::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScaling::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScaling::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScaling::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
```

### `src/operation/describe_table_replica_auto_scaling.rs`

```diff
--- reference/src/operation/describe_table_replica_auto_scaling.rs
+++ generated/src/operation/describe_table_replica_auto_scaling.rs
@@ -213,7 +213,9 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_describe_table_replica_auto_scaling::de_describe_table_replica_auto_scaling_http_error(status, headers, body)
+            super::super::protocol_serde::shape_describe_table_replica_auto_scaling::de_describe_table_replica_auto_scaling_http_error(
+                status, headers, body,
+            )
         } else {
             super::super::protocol_serde::shape_describe_table_replica_auto_scaling::de_describe_table_replica_auto_scaling_http_response(
                 status, headers, body,
```

### `src/operation/describe_time_to_live/_describe_time_to_live_input.rs`

```diff
--- reference/src/operation/describe_time_to_live/_describe_time_to_live_input.rs
+++ generated/src/operation/describe_time_to_live/_describe_time_to_live_input.rs
@@ -44,8 +44,10 @@
     /// Consumes the builder and constructs a [`DescribeTimeToLiveInput`](crate::operation::describe_time_to_live::DescribeTimeToLiveInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::describe_time_to_live::DescribeTimeToLiveInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::describe_time_to_live::DescribeTimeToLiveInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::describe_time_to_live::DescribeTimeToLiveInput { table_name: self.table_name })
     }
 }
```

### `src/operation/describe_time_to_live.rs`

```diff
--- reference/src/operation/describe_time_to_live.rs
+++ generated/src/operation/describe_time_to_live.rs
@@ -258,9 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_describe_time_to_live::ser_describe_time_to_live_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_describe_time_to_live::ser_describe_time_to_live_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/disable_kinesis_streaming_destination/_disable_kinesis_streaming_destination_input.rs`

```diff
--- reference/src/operation/disable_kinesis_streaming_destination/_disable_kinesis_streaming_destination_input.rs
+++ generated/src/operation/disable_kinesis_streaming_destination/_disable_kinesis_streaming_destination_input.rs
@@ -26,7 +26,8 @@
 }
 impl DisableKinesisStreamingDestinationInput {
     /// Creates a new builder-style object to manufacture [`DisableKinesisStreamingDestinationInput`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationInput).
-    pub fn builder() -> super::super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationInputBuilder {
+    pub fn builder() -> super::super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationInputBuilder
+    {
         super::super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationInputBuilder::default()
     }
 }
@@ -84,7 +85,9 @@
         self
     }
     /// <p>The source for the Kinesis streaming information that is being enabled.</p>
-    pub fn get_enable_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
+    pub fn get_enable_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
         &self.enable_kinesis_streaming_configuration
     }
     /// Consumes the builder and constructs a [`DisableKinesisStreamingDestinationInput`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationInput).
```

### `src/operation/disable_kinesis_streaming_destination/_disable_kinesis_streaming_destination_output.rs`

```diff
--- reference/src/operation/disable_kinesis_streaming_destination/_disable_kinesis_streaming_destination_output.rs
+++ generated/src/operation/disable_kinesis_streaming_destination/_disable_kinesis_streaming_destination_output.rs
@@ -38,7 +38,8 @@
 }
 impl DisableKinesisStreamingDestinationOutput {
     /// Creates a new builder-style object to manufacture [`DisableKinesisStreamingDestinationOutput`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput).
-    pub fn builder() -> super::super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationOutputBuilder {
+    pub fn builder(
+    ) -> super::super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationOutputBuilder {
         super::super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationOutputBuilder::default()
     }
 }
@@ -110,7 +111,9 @@
         self
     }
     /// <p>The destination for the Kinesis streaming information that is being enabled.</p>
-    pub fn get_enable_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
+    pub fn get_enable_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
         &self.enable_kinesis_streaming_configuration
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/disable_kinesis_streaming_destination/builders.rs`

```diff
--- reference/src/operation/disable_kinesis_streaming_destination/builders.rs
+++ generated/src/operation/disable_kinesis_streaming_destination/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the DisableKinesisStreamingDestination as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,17 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestination::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestination::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestination::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestination::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -150,7 +157,9 @@
         self
     }
     /// <p>The source for the Kinesis streaming information that is being enabled.</p>
-    pub fn get_enable_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
+    pub fn get_enable_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
         self.inner.get_enable_kinesis_streaming_configuration()
     }
 }
```

### `src/operation/enable_kinesis_streaming_destination/_enable_kinesis_streaming_destination_input.rs`

```diff
--- reference/src/operation/enable_kinesis_streaming_destination/_enable_kinesis_streaming_destination_input.rs
+++ generated/src/operation/enable_kinesis_streaming_destination/_enable_kinesis_streaming_destination_input.rs
@@ -26,7 +26,8 @@
 }
 impl EnableKinesisStreamingDestinationInput {
     /// Creates a new builder-style object to manufacture [`EnableKinesisStreamingDestinationInput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationInput).
-    pub fn builder() -> super::super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder {
+    pub fn builder() -> super::super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder
+    {
         super::super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder::default()
     }
 }
@@ -84,7 +85,9 @@
         self
     }
     /// <p>The source for the Kinesis streaming information that is being enabled.</p>
-    pub fn get_enable_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
+    pub fn get_enable_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
         &self.enable_kinesis_streaming_configuration
     }
     /// Consumes the builder and constructs a [`EnableKinesisStreamingDestinationInput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationInput).
```

### `src/operation/enable_kinesis_streaming_destination/_enable_kinesis_streaming_destination_output.rs`

```diff
--- reference/src/operation/enable_kinesis_streaming_destination/_enable_kinesis_streaming_destination_output.rs
+++ generated/src/operation/enable_kinesis_streaming_destination/_enable_kinesis_streaming_destination_output.rs
@@ -38,7 +38,8 @@
 }
 impl EnableKinesisStreamingDestinationOutput {
     /// Creates a new builder-style object to manufacture [`EnableKinesisStreamingDestinationOutput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput).
-    pub fn builder() -> super::super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder {
+    pub fn builder() -> super::super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder
+    {
         super::super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder::default()
     }
 }
@@ -110,7 +111,9 @@
         self
     }
     /// <p>The destination for the Kinesis streaming information that is being enabled.</p>
-    pub fn get_enable_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
+    pub fn get_enable_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
         &self.enable_kinesis_streaming_configuration
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/enable_kinesis_streaming_destination/builders.rs`

```diff
--- reference/src/operation/enable_kinesis_streaming_destination/builders.rs
+++ generated/src/operation/enable_kinesis_streaming_destination/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the EnableKinesisStreamingDestination as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestination::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestination::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestination::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestination::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -150,7 +154,9 @@
         self
     }
     /// <p>The source for the Kinesis streaming information that is being enabled.</p>
-    pub fn get_enable_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
+    pub fn get_enable_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::EnableKinesisStreamingConfiguration> {
         self.inner.get_enable_kinesis_streaming_configuration()
     }
 }
```

### `src/operation/execute_statement/_execute_statement_input.rs`

```diff
--- reference/src/operation/execute_statement/_execute_statement_input.rs
+++ generated/src/operation/execute_statement/_execute_statement_input.rs
@@ -66,7 +66,9 @@
     }
     /// <p>An optional parameter that returns the item attributes for an <code>ExecuteStatement</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn return_values_on_condition_check_failure(&self) -> ::std::option::Option<&super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn return_values_on_condition_check_failure(
+        &self,
+    ) -> ::std::option::Option<&super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         self.return_values_on_condition_check_failure.as_ref()
     }
 }
@@ -225,13 +227,18 @@
     }
     /// <p>An optional parameter that returns the item attributes for an <code>ExecuteStatement</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn get_return_values_on_condition_check_failure(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         &self.return_values_on_condition_check_failure
     }
     /// Consumes the builder and constructs a [`ExecuteStatementInput`](crate::operation::execute_statement::ExecuteStatementInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::execute_statement::ExecuteStatementInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::execute_statement::ExecuteStatementInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::execute_statement::ExecuteStatementInput {
             statement: self.statement,
             parameters: self.parameters,
```

### `src/operation/execute_statement/_execute_statement_output.rs`

```diff
--- reference/src/operation/execute_statement/_execute_statement_output.rs
+++ generated/src/operation/execute_statement/_execute_statement_output.rs
@@ -29,7 +29,9 @@
         self.consumed_capacity.as_ref()
     }
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request. If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved. If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
-    pub fn last_evaluated_key(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn last_evaluated_key(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         self.last_evaluated_key.as_ref()
     }
 }
@@ -49,10 +51,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct ExecuteStatementOutputBuilder {
-    pub(crate) items: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>>,
+    pub(crate) items:
+        ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>>,
     pub(crate) next_token: ::std::option::Option<::std::string::String>,
     pub(crate) consumed_capacity: ::std::option::Option<super::super::super::types::ConsumedCapacity>,
-    pub(crate) last_evaluated_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) last_evaluated_key:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     _request_id: Option<String>,
 }
 impl ExecuteStatementOutputBuilder {
@@ -129,7 +133,9 @@
         self
     }
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request. If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved. If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
-    pub fn get_last_evaluated_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn get_last_evaluated_key(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         &self.last_evaluated_key
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/execute_statement/builders.rs`

```diff
--- reference/src/operation/execute_statement/builders.rs
+++ generated/src/operation/execute_statement/builders.rs
@@ -243,7 +243,9 @@
     }
     /// <p>An optional parameter that returns the item attributes for an <code>ExecuteStatement</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn get_return_values_on_condition_check_failure(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         self.inner.get_return_values_on_condition_check_failure()
     }
 }
```

### `src/operation/execute_statement.rs`

```diff
--- reference/src/operation/execute_statement.rs
+++ generated/src/operation/execute_statement.rs
@@ -263,7 +263,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_execute_statement::ser_execute_statement_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_execute_statement::ser_execute_statement_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/execute_transaction/_execute_transaction_input.rs`

```diff
--- reference/src/operation/execute_transaction/_execute_transaction_input.rs
+++ generated/src/operation/execute_transaction/_execute_transaction_input.rs
@@ -54,7 +54,10 @@
         self
     }
     /// <p>The list of PartiQL statements representing the transaction to run.</p>
-    pub fn set_transact_statements(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ParameterizedStatement>>) -> Self {
+    pub fn set_transact_statements(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ParameterizedStatement>>,
+    ) -> Self {
         self.transact_statements = input;
         self
     }
@@ -93,7 +96,10 @@
     /// Consumes the builder and constructs a [`ExecuteTransactionInput`](crate::operation::execute_transaction::ExecuteTransactionInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::execute_transaction::ExecuteTransactionInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::execute_transaction::ExecuteTransactionInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::execute_transaction::ExecuteTransactionInput {
             transact_statements: self.transact_statements,
             client_request_token: self.client_request_token,
```

### `src/operation/execute_transaction/builders.rs`

```diff
--- reference/src/operation/execute_transaction/builders.rs
+++ generated/src/operation/execute_transaction/builders.rs
@@ -121,7 +121,10 @@
         self
     }
     /// <p>The list of PartiQL statements representing the transaction to run.</p>
-    pub fn set_transact_statements(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ParameterizedStatement>>) -> Self {
+    pub fn set_transact_statements(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ParameterizedStatement>>,
+    ) -> Self {
         self.inner = self.inner.set_transact_statements(input);
         self
     }
```

### `src/operation/execute_transaction.rs`

```diff
--- reference/src/operation/execute_transaction.rs
+++ generated/src/operation/execute_transaction.rs
@@ -265,7 +265,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_execute_transaction::ser_execute_transaction_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_execute_transaction::ser_execute_transaction_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/export_table_to_point_in_time/_export_table_to_point_in_time_input.rs`

```diff
--- reference/src/operation/export_table_to_point_in_time/_export_table_to_point_in_time_input.rs
+++ generated/src/operation/export_table_to_point_in_time/_export_table_to_point_in_time_input.rs
@@ -294,7 +294,10 @@
         self
     }
     /// <p>Optional object containing the parameters specific to an incremental export.</p>
-    pub fn set_incremental_export_specification(mut self, input: ::std::option::Option<super::super::super::types::IncrementalExportSpecification>) -> Self {
+    pub fn set_incremental_export_specification(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::IncrementalExportSpecification>,
+    ) -> Self {
         self.incremental_export_specification = input;
         self
     }
@@ -309,18 +312,20 @@
         super::super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput {
-            table_arn: self.table_arn,
-            export_time: self.export_time,
-            client_token: self.client_token,
-            s3_bucket: self.s3_bucket,
-            s3_bucket_owner: self.s3_bucket_owner,
-            s3_prefix: self.s3_prefix,
-            s3_sse_algorithm: self.s3_sse_algorithm,
-            s3_sse_kms_key_id: self.s3_sse_kms_key_id,
-            export_format: self.export_format,
-            export_type: self.export_type,
-            incremental_export_specification: self.incremental_export_specification,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput {
+                table_arn: self.table_arn,
+                export_time: self.export_time,
+                client_token: self.client_token,
+                s3_bucket: self.s3_bucket,
+                s3_bucket_owner: self.s3_bucket_owner,
+                s3_prefix: self.s3_prefix,
+                s3_sse_algorithm: self.s3_sse_algorithm,
+                s3_sse_kms_key_id: self.s3_sse_kms_key_id,
+                export_format: self.export_format,
+                export_type: self.export_type,
+                incremental_export_specification: self.incremental_export_specification,
+            },
+        )
     }
 }
```

### `src/operation/export_table_to_point_in_time/builders.rs`

```diff
--- reference/src/operation/export_table_to_point_in_time/builders.rs
+++ generated/src/operation/export_table_to_point_in_time/builders.rs
@@ -284,7 +284,10 @@
         self
     }
     /// <p>Optional object containing the parameters specific to an incremental export.</p>
-    pub fn set_incremental_export_specification(mut self, input: ::std::option::Option<super::super::super::types::IncrementalExportSpecification>) -> Self {
+    pub fn set_incremental_export_specification(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::IncrementalExportSpecification>,
+    ) -> Self {
         self.inner = self.inner.set_incremental_export_specification(input);
         self
     }
```

### `src/operation/get_item/_get_item_input.rs`

```diff
--- reference/src/operation/get_item/_get_item_input.rs
+++ generated/src/operation/get_item/_get_item_input.rs
@@ -176,7 +176,10 @@
     }
     /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to retrieve.</p>
     /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    ) -> Self {
         self.key = input;
         self
     }
@@ -388,7 +391,9 @@
         &self.expression_attribute_names
     }
     /// Consumes the builder and constructs a [`GetItemInput`](crate::operation::get_item::GetItemInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::get_item::GetItemInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::get_item::GetItemInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::get_item::GetItemInput {
             table_name: self.table_name,
             key: self.key,
```

### `src/operation/get_item/builders.rs`

```diff
--- reference/src/operation/get_item/builders.rs
+++ generated/src/operation/get_item/builders.rs
@@ -30,14 +30,20 @@
     inner: super::super::super::operation::get_item::builders::GetItemInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::get_item::GetItemOutput, super::super::super::operation::get_item::GetItemError>
-    for GetItemFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::get_item::GetItemOutput,
+        super::super::super::operation::get_item::GetItemError,
+    > for GetItemFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::get_item::GetItemOutput, super::super::super::operation::get_item::GetItemError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::get_item::GetItemOutput,
+            super::super::super::operation::get_item::GetItemError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -87,8 +93,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::get_item::GetItemOutput, super::super::super::operation::get_item::GetItemError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::get_item::GetItemOutput,
+        super::super::super::operation::get_item::GetItemError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
@@ -127,7 +136,10 @@
     }
     /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to retrieve.</p>
     /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    ) -> Self {
         self.inner = self.inner.set_key(input);
         self
     }
```

### `src/operation/get_item.rs`

```diff
--- reference/src/operation/get_item.rs
+++ generated/src/operation/get_item.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::get_item::GetItemError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::get_item::GetItemError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
```

### `src/operation/get_resource_policy/_get_resource_policy_input.rs`

```diff
--- reference/src/operation/get_resource_policy/_get_resource_policy_input.rs
+++ generated/src/operation/get_resource_policy/_get_resource_policy_input.rs
@@ -44,7 +44,10 @@
     /// Consumes the builder and constructs a [`GetResourcePolicyInput`](crate::operation::get_resource_policy::GetResourcePolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::get_resource_policy::GetResourcePolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::get_resource_policy::GetResourcePolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::get_resource_policy::GetResourcePolicyInput {
             resource_arn: self.resource_arn,
         })
```

### `src/operation/get_resource_policy.rs`

```diff
--- reference/src/operation/get_resource_policy.rs
+++ generated/src/operation/get_resource_policy.rs
@@ -258,7 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_resource_policy::ser_get_resource_policy_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_get_resource_policy::ser_get_resource_policy_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/import_table/_import_table_input.rs`

```diff
--- reference/src/operation/import_table/_import_table_input.rs
+++ generated/src/operation/import_table/_import_table_input.rs
@@ -159,7 +159,9 @@
         &self.table_creation_parameters
     }
     /// Consumes the builder and constructs a [`ImportTableInput`](crate::operation::import_table::ImportTableInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::import_table::ImportTableInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::import_table::ImportTableInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::import_table::ImportTableInput {
             client_token: self.client_token,
             s3_bucket_source: self.s3_bucket_source,
```

### `src/operation/list_backups/_list_backups_input.rs`

```diff
--- reference/src/operation/list_backups/_list_backups_input.rs
+++ generated/src/operation/list_backups/_list_backups_input.rs
@@ -191,7 +191,9 @@
         &self.backup_type
     }
     /// Consumes the builder and constructs a [`ListBackupsInput`](crate::operation::list_backups::ListBackupsInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_backups::ListBackupsInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_backups::ListBackupsInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_backups::ListBackupsInput {
             table_name: self.table_name,
             limit: self.limit,
```

### `src/operation/list_contributor_insights/_list_contributor_insights_input.rs`

```diff
--- reference/src/operation/list_contributor_insights/_list_contributor_insights_input.rs
+++ generated/src/operation/list_contributor_insights/_list_contributor_insights_input.rs
@@ -92,7 +92,7 @@
         ::std::result::Result::Ok(super::super::super::operation::list_contributor_insights::ListContributorInsightsInput {
             table_name: self.table_name,
             next_token: self.next_token,
-            max_results: self.max_results,
+            max_results: self.max_results.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/list_contributor_insights/_list_contributor_insights_output.rs`

```diff
--- reference/src/operation/list_contributor_insights/_list_contributor_insights_output.rs
+++ generated/src/operation/list_contributor_insights/_list_contributor_insights_output.rs
@@ -62,7 +62,9 @@
         self
     }
     /// <p>A list of ContributorInsightsSummary.</p>
-    pub fn get_contributor_insights_summaries(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::ContributorInsightsSummary>> {
+    pub fn get_contributor_insights_summaries(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::ContributorInsightsSummary>> {
         &self.contributor_insights_summaries
     }
     /// <p>A token to go to the next page if there is one.</p>
```

### `src/operation/list_contributor_insights/paginator.rs`

```diff
--- reference/src/operation/list_contributor_insights/paginator.rs
+++ generated/src/operation/list_contributor_insights/paginator.rs
@@ -78,8 +78,11 @@
                         }
                     };
                     loop {
-                        let resp =
-                            super::super::super::operation::list_contributor_insights::ListContributorInsights::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = super::super::super::operation::list_contributor_insights::ListContributorInsights::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
```

### `src/operation/list_exports/_list_exports_input.rs`

```diff
--- reference/src/operation/list_exports/_list_exports_input.rs
+++ generated/src/operation/list_exports/_list_exports_input.rs
@@ -83,7 +83,9 @@
         &self.next_token
     }
     /// Consumes the builder and constructs a [`ListExportsInput`](crate::operation::list_exports::ListExportsInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_exports::ListExportsInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_exports::ListExportsInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_exports::ListExportsInput {
             table_arn: self.table_arn,
             max_results: self.max_results,
```

### `src/operation/list_global_tables/_list_global_tables_input.rs`

```diff
--- reference/src/operation/list_global_tables/_list_global_tables_input.rs
+++ generated/src/operation/list_global_tables/_list_global_tables_input.rs
@@ -90,7 +90,10 @@
     /// Consumes the builder and constructs a [`ListGlobalTablesInput`](crate::operation::list_global_tables::ListGlobalTablesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_global_tables::ListGlobalTablesInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_global_tables::ListGlobalTablesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_global_tables::ListGlobalTablesInput {
             exclusive_start_global_table_name: self.exclusive_start_global_table_name,
             limit: self.limit,
```

### `src/operation/list_global_tables.rs`

```diff
--- reference/src/operation/list_global_tables.rs
+++ generated/src/operation/list_global_tables.rs
@@ -263,7 +263,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_global_tables::ser_list_global_tables_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_global_tables::ser_list_global_tables_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_imports/_list_imports_input.rs`

```diff
--- reference/src/operation/list_imports/_list_imports_input.rs
+++ generated/src/operation/list_imports/_list_imports_input.rs
@@ -83,7 +83,9 @@
         &self.next_token
     }
     /// Consumes the builder and constructs a [`ListImportsInput`](crate::operation::list_imports::ListImportsInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_imports::ListImportsInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_imports::ListImportsInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_imports::ListImportsInput {
             table_arn: self.table_arn,
             page_size: self.page_size,
```

### `src/operation/list_tables/_list_tables_input.rs`

```diff
--- reference/src/operation/list_tables/_list_tables_input.rs
+++ generated/src/operation/list_tables/_list_tables_input.rs
@@ -63,7 +63,9 @@
         &self.limit
     }
     /// Consumes the builder and constructs a [`ListTablesInput`](crate::operation::list_tables::ListTablesInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_tables::ListTablesInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_tables::ListTablesInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::list_tables::ListTablesInput {
             exclusive_start_table_name: self.exclusive_start_table_name,
             limit: self.limit,
```

### `src/operation/list_tags_of_resource/_list_tags_of_resource_input.rs`

```diff
--- reference/src/operation/list_tags_of_resource/_list_tags_of_resource_input.rs
+++ generated/src/operation/list_tags_of_resource/_list_tags_of_resource_input.rs
@@ -65,8 +65,10 @@
     /// Consumes the builder and constructs a [`ListTagsOfResourceInput`](crate::operation::list_tags_of_resource::ListTagsOfResourceInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::list_tags_of_resource::ListTagsOfResourceInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_tags_of_resource::ListTagsOfResourceInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::list_tags_of_resource::ListTagsOfResourceInput {
             resource_arn: self.resource_arn,
             next_token: self.next_token,
```

### `src/operation/list_tags_of_resource.rs`

```diff
--- reference/src/operation/list_tags_of_resource.rs
+++ generated/src/operation/list_tags_of_resource.rs
@@ -263,9 +263,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_tags_of_resource::ser_list_tags_of_resource_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::super::protocol_serde::shape_list_tags_of_resource::ser_list_tags_of_resource_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/put_item/_put_item_input.rs`

```diff
--- reference/src/operation/put_item/_put_item_input.rs
+++ generated/src/operation/put_item/_put_item_input.rs
@@ -100,7 +100,8 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     /// <p>An optional parameter that returns the item attributes for a <code>PutItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
     pub return_values_on_condition_check_failure: ::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure>,
@@ -129,7 +130,9 @@
         self.item.as_ref()
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expected(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn expected(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         self.expected.as_ref()
     }
     /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were updated with the <code>PutItem</code> request. For <code>PutItem</code>, the valid values are:</p>
@@ -227,7 +230,9 @@
     }
     /// <p>An optional parameter that returns the item attributes for a <code>PutItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn return_values_on_condition_check_failure(&self) -> ::std::option::Option<&super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn return_values_on_condition_check_failure(
+        &self,
+    ) -> ::std::option::Option<&super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         self.return_values_on_condition_check_failure.as_ref()
     }
 }
@@ -244,7 +249,8 @@
 pub struct PutItemInputBuilder {
     pub(crate) table_name: ::std::option::Option<::std::string::String>,
     pub(crate) item: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
-    pub(crate) expected: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>>,
+    pub(crate) expected:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>>,
     pub(crate) return_values: ::std::option::Option<super::super::super::types::ReturnValue>,
     pub(crate) return_consumed_capacity: ::std::option::Option<super::super::super::types::ReturnConsumedCapacity>,
     pub(crate) return_item_collection_metrics: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
@@ -251,7 +257,8 @@
     pub(crate) conditional_operator: ::std::option::Option<super::super::super::types::ConditionalOperator>,
     pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) return_values_on_condition_check_failure: ::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure>,
 }
 impl PutItemInputBuilder {
@@ -355,7 +362,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn get_expected(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         &self.expected
     }
     /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were updated with the <code>PutItem</code> request. For <code>PutItem</code>, the valid values are:</p>
@@ -449,7 +458,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.return_item_collection_metrics = input;
         self
     }
@@ -637,7 +649,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
@@ -688,11 +704,15 @@
     }
     /// <p>An optional parameter that returns the item attributes for a <code>PutItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn get_return_values_on_condition_check_failure(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         &self.return_values_on_condition_check_failure
     }
     /// Consumes the builder and constructs a [`PutItemInput`](crate::operation::put_item::PutItemInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::put_item::PutItemInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::put_item::PutItemInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::put_item::PutItemInput {
             table_name: self.table_name,
             item: self.item,
```

### `src/operation/put_item/_put_item_output.rs`

```diff
--- reference/src/operation/put_item/_put_item_output.rs
+++ generated/src/operation/put_item/_put_item_output.rs
@@ -23,7 +23,9 @@
 }
 impl PutItemOutput {
     /// <p>The attribute values as they appeared before the <code>PutItem</code> operation, but only if <code>ReturnValues</code> is specified as <code>ALL_OLD</code> in the request. Each element consists of an attribute name and an attribute value.</p>
-    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn attributes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         self.attributes.as_ref()
     }
     /// <p>The capacity units consumed by the <code>PutItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#write-operation-consumption">Capacity unity consumption for write operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -86,7 +88,9 @@
         self
     }
     /// <p>The attribute values as they appeared before the <code>PutItem</code> operation, but only if <code>ReturnValues</code> is specified as <code>ALL_OLD</code> in the request. Each element consists of an attribute name and an attribute value.</p>
-    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn get_attributes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         &self.attributes
     }
     /// <p>The capacity units consumed by the <code>PutItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#write-operation-consumption">Capacity unity consumption for write operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
```

### `src/operation/put_item/builders.rs`

```diff
--- reference/src/operation/put_item/builders.rs
+++ generated/src/operation/put_item/builders.rs
@@ -37,14 +37,20 @@
     inner: super::super::super::operation::put_item::builders::PutItemInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::put_item::PutItemOutput, super::super::super::operation::put_item::PutItemError>
-    for PutItemFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::put_item::PutItemOutput,
+        super::super::super::operation::put_item::PutItemError,
+    > for PutItemFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::put_item::PutItemOutput, super::super::super::operation::put_item::PutItemError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::put_item::PutItemOutput,
+            super::super::super::operation::put_item::PutItemError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -94,8 +100,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::put_item::PutItemOutput, super::super::super::operation::put_item::PutItemError, Self>
-    {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::put_item::PutItemOutput,
+        super::super::super::operation::put_item::PutItemError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
@@ -204,7 +213,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn get_expected(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         self.inner.get_expected()
     }
     /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were updated with the <code>PutItem</code> request. For <code>PutItem</code>, the valid values are:</p>
@@ -298,7 +309,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.inner = self.inner.set_return_item_collection_metrics(input);
         self
     }
@@ -486,7 +500,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         self.inner = self.inner.expression_attribute_values(k.into(), v);
         self
     }
@@ -535,7 +553,9 @@
     }
     /// <p>An optional parameter that returns the item attributes for a <code>PutItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn get_return_values_on_condition_check_failure(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         self.inner.get_return_values_on_condition_check_failure()
     }
 }
```

### `src/operation/put_item.rs`

```diff
--- reference/src/operation/put_item.rs
+++ generated/src/operation/put_item.rs
@@ -18,11 +18,15 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<super::super::operation::put_item::PutItemError>().expect("correct error type")) };
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<super::super::operation::put_item::PutItemError>()
+                    .expect("correct error type")
+            })
+        };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
```

### `src/operation/put_resource_policy/_put_resource_policy_input.rs`

```diff
--- reference/src/operation/put_resource_policy/_put_resource_policy_input.rs
+++ generated/src/operation/put_resource_policy/_put_resource_policy_input.rs
@@ -163,12 +163,15 @@
     /// Consumes the builder and constructs a [`PutResourcePolicyInput`](crate::operation::put_resource_policy::PutResourcePolicyInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::put_resource_policy::PutResourcePolicyInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::put_resource_policy::PutResourcePolicyInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::put_resource_policy::PutResourcePolicyInput {
             resource_arn: self.resource_arn,
             policy: self.policy,
             expected_revision_id: self.expected_revision_id,
-            confirm_remove_self_resource_access: self.confirm_remove_self_resource_access,
+            confirm_remove_self_resource_access: self.confirm_remove_self_resource_access.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/put_resource_policy.rs`

```diff
--- reference/src/operation/put_resource_policy.rs
+++ generated/src/operation/put_resource_policy.rs
@@ -268,7 +268,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_put_resource_policy::ser_put_resource_policy_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_put_resource_policy::ser_put_resource_policy_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/query/_query_input.rs`

```diff
--- reference/src/operation/query/_query_input.rs
+++ generated/src/operation/query/_query_input.rs
@@ -140,7 +140,8 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
 }
 impl QueryInput {
     /// <p>The name of the table containing the requested items. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
@@ -186,7 +187,9 @@
         self.consistent_read
     }
     /// <p>This is a legacy parameter. Use <code>KeyConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.KeyConditions.html">KeyConditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn key_conditions(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
+    pub fn key_conditions(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
         self.key_conditions.as_ref()
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.QueryFilter.html">QueryFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -205,7 +208,9 @@
     }
     /// <p>The primary key of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedKey</code> in the previous operation.</p>
     /// <p>The data type for <code>ExclusiveStartKey</code> must be String, Number, or Binary. No set data types are allowed.</p>
-    pub fn exclusive_start_key(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn exclusive_start_key(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         self.exclusive_start_key.as_ref()
     }
     /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
@@ -340,13 +345,15 @@
     pub(crate) query_filter: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>>,
     pub(crate) conditional_operator: ::std::option::Option<super::super::super::types::ConditionalOperator>,
     pub(crate) scan_index_forward: ::std::option::Option<bool>,
-    pub(crate) exclusive_start_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) exclusive_start_key:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) return_consumed_capacity: ::std::option::Option<super::super::super::types::ReturnConsumedCapacity>,
     pub(crate) projection_expression: ::std::option::Option<::std::string::String>,
     pub(crate) filter_expression: ::std::option::Option<::std::string::String>,
     pub(crate) key_condition_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
 }
 impl QueryInputBuilder {
     /// <p>The name of the table containing the requested items. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
@@ -508,7 +515,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>KeyConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.KeyConditions.html">KeyConditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_key_conditions(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
+    pub fn get_key_conditions(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
         &self.key_conditions
     }
     /// Adds a key-value pair to `query_filter`.
@@ -531,7 +540,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.QueryFilter.html">QueryFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_query_filter(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
+    pub fn get_query_filter(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
         &self.query_filter
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -924,7 +935,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
```

### `src/operation/query/_query_output.rs`

```diff
--- reference/src/operation/query/_query_output.rs
+++ generated/src/operation/query/_query_output.rs
@@ -42,7 +42,9 @@
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
     /// <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
     /// <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
-    pub fn last_evaluated_key(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn last_evaluated_key(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         self.last_evaluated_key.as_ref()
     }
     /// <p>The capacity units consumed by the <code>Query</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#read-operation-consumption">Capacity unit consumption for read operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -66,10 +68,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct QueryOutputBuilder {
-    pub(crate) items: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>>,
+    pub(crate) items:
+        ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>>,
     pub(crate) count: ::std::option::Option<i32>,
     pub(crate) scanned_count: ::std::option::Option<i32>,
-    pub(crate) last_evaluated_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) last_evaluated_key:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) consumed_capacity: ::std::option::Option<super::super::super::types::ConsumedCapacity>,
     _request_id: Option<String>,
 }
@@ -162,7 +166,9 @@
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
     /// <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
     /// <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
-    pub fn get_last_evaluated_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn get_last_evaluated_key(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         &self.last_evaluated_key
     }
     /// <p>The capacity units consumed by the <code>Query</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#read-operation-consumption">Capacity unit consumption for read operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
```

### `src/operation/query/builders.rs`

```diff
--- reference/src/operation/query/builders.rs
+++ generated/src/operation/query/builders.rs
@@ -39,14 +39,20 @@
     inner: super::super::super::operation::query::builders::QueryInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::query::QueryOutput, super::super::super::operation::query::QueryError>
-    for QueryFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::query::QueryOutput,
+        super::super::super::operation::query::QueryError,
+    > for QueryFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::query::QueryOutput, super::super::super::operation::query::QueryError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::query::QueryOutput,
+            super::super::super::operation::query::QueryError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -85,8 +91,11 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            super::super::super::operation::query::Query::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
+        let runtime_plugins = super::super::super::operation::query::Query::operation_runtime_plugins(
+            self.handle.runtime_plugins.clone(),
+            &self.handle.conf,
+            self.config_override,
+        );
         super::super::super::operation::query::Query::orchestrate(&runtime_plugins, input).await
     }

@@ -93,7 +102,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::query::QueryOutput, super::super::super::operation::query::QueryError, Self> {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::query::QueryOutput,
+        super::super::super::operation::query::QueryError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
@@ -267,7 +280,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>KeyConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.KeyConditions.html">KeyConditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_key_conditions(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
+    pub fn get_key_conditions(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
         self.inner.get_key_conditions()
     }
     ///
@@ -289,7 +304,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.QueryFilter.html">QueryFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_query_filter(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
+    pub fn get_query_filter(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
         self.inner.get_query_filter()
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -681,7 +698,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         self.inner = self.inner.expression_attribute_values(k.into(), v);
         self
     }
```

### `src/operation/query/paginator.rs`

```diff
--- reference/src/operation/query/paginator.rs
+++ generated/src/operation/query/paginator.rs
@@ -8,7 +8,10 @@

 impl QueryPaginator {
     /// Create a new paginator-wrapper
-    pub(crate) fn new(handle: std::sync::Arc<super::super::super::client::Handle>, builder: super::super::super::operation::query::builders::QueryInputBuilder) -> Self {
+    pub(crate) fn new(
+        handle: std::sync::Arc<super::super::super::client::Handle>,
+        builder: super::super::super::operation::query::builders::QueryInputBuilder,
+    ) -> Self {
         Self {
             handle,
             builder,
@@ -62,9 +65,12 @@
         // Move individual fields out of self for the borrow checker
         let builder = self.builder;
         let handle = self.handle;
-        let runtime_plugins =
-            super::super::super::operation::query::Query::operation_runtime_plugins(handle.runtime_plugins.clone(), &handle.conf, ::std::option::Option::None)
-                .with_operation_plugin(super::super::super::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
+        let runtime_plugins = super::super::super::operation::query::Query::operation_runtime_plugins(
+            handle.runtime_plugins.clone(),
+            &handle.conf,
+            ::std::option::Option::None,
+        )
+        .with_operation_plugin(super::super::super::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
         ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
             move |tx| {
                 ::std::boxed::Box::pin(async move {
@@ -133,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_query_output_output_items(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_query_output_output_items(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/query.rs`

```diff
--- reference/src/operation/query.rs
+++ generated/src/operation/query.rs
@@ -18,15 +18,20 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
-            ::aws_smithy_runtime_api::client::interceptors::context::Error,
-            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-        >| { err.map_service_error(|err| err.downcast::<super::super::operation::query::QueryError>().expect("correct error type")) };
+        let map_err =
+            |err: ::aws_smithy_runtime_api::client::result::SdkError<
+                ::aws_smithy_runtime_api::client::interceptors::context::Error,
+                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+            >| { err.map_service_error(|err| err.downcast::<super::super::operation::query::QueryError>().expect("correct error type")) };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(output.downcast::<super::super::operation::query::QueryOutput>().expect("correct output type"))
+        ::std::result::Result::Ok(
+            output
+                .downcast::<super::super::operation::query::QueryOutput>()
+                .expect("correct output type"),
+        )
     }

     pub(crate) async fn orchestrate_with_stop_point(
```

### `src/operation/restore_table_from_backup/_restore_table_from_backup_input.rs`

```diff
--- reference/src/operation/restore_table_from_backup/_restore_table_from_backup_input.rs
+++ generated/src/operation/restore_table_from_backup/_restore_table_from_backup_input.rs
@@ -144,7 +144,10 @@
         self
     }
     /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
-    pub fn set_global_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>) -> Self {
+    pub fn set_global_secondary_index_override(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>,
+    ) -> Self {
         self.global_secondary_index_override = input;
         self
     }
@@ -164,7 +167,10 @@
         self
     }
     /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
-    pub fn set_local_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>) -> Self {
+    pub fn set_local_secondary_index_override(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>,
+    ) -> Self {
         self.local_secondary_index_override = input;
         self
     }
```

### `src/operation/restore_table_from_backup/builders.rs`

```diff
--- reference/src/operation/restore_table_from_backup/builders.rs
+++ generated/src/operation/restore_table_from_backup/builders.rs
@@ -177,7 +177,10 @@
         self
     }
     /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
-    pub fn set_global_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>) -> Self {
+    pub fn set_global_secondary_index_override(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>,
+    ) -> Self {
         self.inner = self.inner.set_global_secondary_index_override(input);
         self
     }
@@ -196,7 +199,10 @@
         self
     }
     /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
-    pub fn set_local_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>) -> Self {
+    pub fn set_local_secondary_index_override(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>,
+    ) -> Self {
         self.inner = self.inner.set_local_secondary_index_override(input);
         self
     }
```

### `src/operation/restore_table_to_point_in_time/_restore_table_to_point_in_time_input.rs`

```diff
--- reference/src/operation/restore_table_to_point_in_time/_restore_table_to_point_in_time_input.rs
+++ generated/src/operation/restore_table_to_point_in_time/_restore_table_to_point_in_time_input.rs
@@ -210,7 +210,10 @@
     }
     /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
     /// <p>The <code>WarmThroughput</code> setting is not supported on global secondary indexes when you use <code>RestoreTableToPointInTime</code>. Although <code>WarmThroughput</code> appears in the shared index definition, including it in a <code>GlobalSecondaryIndexOverride</code> entry causes the request to fail with a validation error.</p>
-    pub fn set_global_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>) -> Self {
+    pub fn set_global_secondary_index_override(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>,
+    ) -> Self {
         self.global_secondary_index_override = input;
         self
     }
@@ -231,7 +234,10 @@
         self
     }
     /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
-    pub fn set_local_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>) -> Self {
+    pub fn set_local_secondary_index_override(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>,
+    ) -> Self {
         self.local_secondary_index_override = input;
         self
     }
@@ -308,19 +314,21 @@
         super::super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput {
-            source_table_arn: self.source_table_arn,
-            source_table_name: self.source_table_name,
-            target_table_name: self.target_table_name,
-            use_latest_restorable_time: self.use_latest_restorable_time,
-            restore_date_time: self.restore_date_time,
-            billing_mode_override: self.billing_mode_override,
-            global_secondary_index_override: self.global_secondary_index_override,
-            local_secondary_index_override: self.local_secondary_index_override,
-            provisioned_throughput_override: self.provisioned_throughput_override,
-            on_demand_throughput_override: self.on_demand_throughput_override,
-            sse_specification_override: self.sse_specification_override,
-            vector_index_override: self.vector_index_override,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput {
+                source_table_arn: self.source_table_arn,
+                source_table_name: self.source_table_name,
+                target_table_name: self.target_table_name,
+                use_latest_restorable_time: self.use_latest_restorable_time,
+                restore_date_time: self.restore_date_time,
+                billing_mode_override: self.billing_mode_override,
+                global_secondary_index_override: self.global_secondary_index_override,
+                local_secondary_index_override: self.local_secondary_index_override,
+                provisioned_throughput_override: self.provisioned_throughput_override,
+                on_demand_throughput_override: self.on_demand_throughput_override,
+                sse_specification_override: self.sse_specification_override,
+                vector_index_override: self.vector_index_override,
+            },
+        )
     }
 }
```

### `src/operation/restore_table_to_point_in_time/builders.rs`

```diff
--- reference/src/operation/restore_table_to_point_in_time/builders.rs
+++ generated/src/operation/restore_table_to_point_in_time/builders.rs
@@ -236,7 +236,10 @@
     }
     /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
     /// <p>The <code>WarmThroughput</code> setting is not supported on global secondary indexes when you use <code>RestoreTableToPointInTime</code>. Although <code>WarmThroughput</code> appears in the shared index definition, including it in a <code>GlobalSecondaryIndexOverride</code> entry causes the request to fail with a validation error.</p>
-    pub fn set_global_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>) -> Self {
+    pub fn set_global_secondary_index_override(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndex>>,
+    ) -> Self {
         self.inner = self.inner.set_global_secondary_index_override(input);
         self
     }
@@ -256,7 +259,10 @@
         self
     }
     /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
-    pub fn set_local_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>) -> Self {
+    pub fn set_local_secondary_index_override(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::LocalSecondaryIndex>>,
+    ) -> Self {
         self.inner = self.inner.set_local_secondary_index_override(input);
         self
     }
```

### `src/operation/scan/_scan_input.rs`

```diff
--- reference/src/operation/scan/_scan_input.rs
+++ generated/src/operation/scan/_scan_input.rs
@@ -104,7 +104,8 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     /// <p>A Boolean value that determines the read consistency model during the scan:</p>
     /// <ul>
     /// <li>
@@ -166,7 +167,9 @@
     /// <p>The primary key of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedKey</code> in the previous operation.</p>
     /// <p>The data type for <code>ExclusiveStartKey</code> must be String, Number or Binary. No set data types are allowed.</p>
     /// <p>In a parallel scan, a <code>Scan</code> request that includes <code>ExclusiveStartKey</code> must specify the same segment whose previous <code>Scan</code> returned the corresponding value of <code>LastEvaluatedKey</code>.</p>
-    pub fn exclusive_start_key(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn exclusive_start_key(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         self.exclusive_start_key.as_ref()
     }
     /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
@@ -283,7 +286,8 @@
     pub(crate) select: ::std::option::Option<super::super::super::types::Select>,
     pub(crate) scan_filter: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>>,
     pub(crate) conditional_operator: ::std::option::Option<super::super::super::types::ConditionalOperator>,
-    pub(crate) exclusive_start_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) exclusive_start_key:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) return_consumed_capacity: ::std::option::Option<super::super::super::types::ReturnConsumedCapacity>,
     pub(crate) total_segments: ::std::option::Option<i32>,
     pub(crate) segment: ::std::option::Option<i32>,
@@ -290,7 +294,8 @@
     pub(crate) projection_expression: ::std::option::Option<::std::string::String>,
     pub(crate) filter_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) consistent_read: ::std::option::Option<bool>,
 }
 impl ScanInputBuilder {
@@ -439,7 +444,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ScanFilter.html">ScanFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_scan_filter(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
+    pub fn get_scan_filter(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
         &self.scan_filter
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -736,7 +743,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
```

### `src/operation/scan/_scan_output.rs`

```diff
--- reference/src/operation/scan/_scan_output.rs
+++ generated/src/operation/scan/_scan_output.rs
@@ -42,7 +42,9 @@
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
     /// <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
     /// <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
-    pub fn last_evaluated_key(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn last_evaluated_key(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         self.last_evaluated_key.as_ref()
     }
     /// <p>The capacity units consumed by the <code>Scan</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#read-operation-consumption">Capacity unit consumption for read operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -66,10 +68,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct ScanOutputBuilder {
-    pub(crate) items: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>>,
+    pub(crate) items:
+        ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>>,
     pub(crate) count: ::std::option::Option<i32>,
     pub(crate) scanned_count: ::std::option::Option<i32>,
-    pub(crate) last_evaluated_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) last_evaluated_key:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) consumed_capacity: ::std::option::Option<super::super::super::types::ConsumedCapacity>,
     _request_id: Option<String>,
 }
@@ -162,7 +166,9 @@
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
     /// <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
     /// <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
-    pub fn get_last_evaluated_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn get_last_evaluated_key(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         &self.last_evaluated_key
     }
     /// <p>The capacity units consumed by the <code>Scan</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#read-operation-consumption">Capacity unit consumption for read operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
```

### `src/operation/scan/builders.rs`

```diff
--- reference/src/operation/scan/builders.rs
+++ generated/src/operation/scan/builders.rs
@@ -37,14 +37,20 @@
     inner: super::super::super::operation::scan::builders::ScanInputBuilder,
     config_override: ::std::option::Option<super::super::super::config::Builder>,
 }
-impl super::super::super::client::customize::internal::CustomizableSend<super::super::super::operation::scan::ScanOutput, super::super::super::operation::scan::ScanError>
-    for ScanFluentBuilder
+impl
+    super::super::super::client::customize::internal::CustomizableSend<
+        super::super::super::operation::scan::ScanOutput,
+        super::super::super::operation::scan::ScanError,
+    > for ScanFluentBuilder
 {
     fn send(
         self,
         config_override: super::super::super::config::Builder,
     ) -> super::super::super::client::customize::internal::BoxFuture<
-        super::super::super::client::customize::internal::SendResult<super::super::super::operation::scan::ScanOutput, super::super::super::operation::scan::ScanError>,
+        super::super::super::client::customize::internal::SendResult<
+            super::super::super::operation::scan::ScanOutput,
+            super::super::super::operation::scan::ScanError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -83,8 +89,11 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            super::super::super::operation::scan::Scan::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
+        let runtime_plugins = super::super::super::operation::scan::Scan::operation_runtime_plugins(
+            self.handle.runtime_plugins.clone(),
+            &self.handle.conf,
+            self.config_override,
+        );
         super::super::super::operation::scan::Scan::orchestrate(&runtime_plugins, input).await
     }

@@ -91,7 +100,11 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> super::super::super::client::customize::CustomizableOperation<super::super::super::operation::scan::ScanOutput, super::super::super::operation::scan::ScanError, Self> {
+    ) -> super::super::super::client::customize::CustomizableOperation<
+        super::super::super::operation::scan::ScanOutput,
+        super::super::super::operation::scan::ScanError,
+        Self,
+    > {
         super::super::super::client::customize::CustomizableOperation::new(self)
     }
     pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<super::super::super::config::Builder>) -> Self {
@@ -251,7 +264,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ScanFilter.html">ScanFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_scan_filter(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
+    pub fn get_scan_filter(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::Condition>> {
         self.inner.get_scan_filter()
     }
     /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -547,7 +562,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         self.inner = self.inner.expression_attribute_values(k.into(), v);
         self
     }
```

### `src/operation/scan/paginator.rs`

```diff
--- reference/src/operation/scan/paginator.rs
+++ generated/src/operation/scan/paginator.rs
@@ -8,7 +8,10 @@

 impl ScanPaginator {
     /// Create a new paginator-wrapper
-    pub(crate) fn new(handle: std::sync::Arc<super::super::super::client::Handle>, builder: super::super::super::operation::scan::builders::ScanInputBuilder) -> Self {
+    pub(crate) fn new(
+        handle: std::sync::Arc<super::super::super::client::Handle>,
+        builder: super::super::super::operation::scan::builders::ScanInputBuilder,
+    ) -> Self {
         Self {
             handle,
             builder,
@@ -62,9 +65,12 @@
         // Move individual fields out of self for the borrow checker
         let builder = self.builder;
         let handle = self.handle;
-        let runtime_plugins =
-            super::super::super::operation::scan::Scan::operation_runtime_plugins(handle.runtime_plugins.clone(), &handle.conf, ::std::option::Option::None)
-                .with_operation_plugin(super::super::super::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
+        let runtime_plugins = super::super::super::operation::scan::Scan::operation_runtime_plugins(
+            handle.runtime_plugins.clone(),
+            &handle.conf,
+            ::std::option::Option::None,
+        )
+        .with_operation_plugin(super::super::super::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
         ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
             move |tx| {
                 ::std::boxed::Box::pin(async move {
@@ -133,7 +139,10 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
-            .flat_map(|page| super::super::super::lens::lens_scan_output_output_items(page).unwrap_or_default().into_iter())
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
+            super::super::super::lens::lens_scan_output_output_items(page)
+                .unwrap_or_default()
+                .into_iter()
+        })
     }
 }
```

### `src/operation/scan.rs`

```diff
--- reference/src/operation/scan.rs
+++ generated/src/operation/scan.rs
@@ -18,15 +18,20 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
-            ::aws_smithy_runtime_api::client::interceptors::context::Error,
-            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-        >| { err.map_service_error(|err| err.downcast::<super::super::operation::scan::ScanError>().expect("correct error type")) };
+        let map_err =
+            |err: ::aws_smithy_runtime_api::client::result::SdkError<
+                ::aws_smithy_runtime_api::client::interceptors::context::Error,
+                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+            >| { err.map_service_error(|err| err.downcast::<super::super::operation::scan::ScanError>().expect("correct error type")) };
         let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
             .await
             .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(output.downcast::<super::super::operation::scan::ScanOutput>().expect("correct output type"))
+        ::std::result::Result::Ok(
+            output
+                .downcast::<super::super::operation::scan::ScanOutput>()
+                .expect("correct output type"),
+        )
     }

     pub(crate) async fn orchestrate_with_stop_point(
```

### `src/operation/search_vectors/_search_vectors_input.rs`

```diff
--- reference/src/operation/search_vectors/_search_vectors_input.rs
+++ generated/src/operation/search_vectors/_search_vectors_input.rs
@@ -21,7 +21,8 @@
     /// <p>One or more substitution tokens for attribute names in an expression. Use the <code>#</code> character in an expression to dereference an attribute name.</p>
     pub expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
     /// <p>One or more values that can be substituted in an expression. Use the <code>:</code> character in an expression to dereference an attribute value.</p>
-    pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     /// <p>A string that identifies one or more attributes to retrieve from the index. Separate attribute names with commas. If not specified, the operation returns all attributes projected into the vector index.</p>
     /// <p>Only attributes projected into the vector index can be retrieved.</p>
     pub projection_expression: ::std::option::Option<::std::string::String>,
@@ -103,7 +104,8 @@
     pub(crate) index_name: ::std::option::Option<::std::string::String>,
     pub(crate) return_consumed_capacity: ::std::option::Option<super::super::super::types::ReturnConsumedCapacity>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) projection_expression: ::std::option::Option<::std::string::String>,
     pub(crate) search_vector: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeValue>>,
     pub(crate) search_condition_expression: ::std::option::Option<::std::string::String>,
@@ -215,7 +217,11 @@
     /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
     ///
     /// <p>One or more values that can be substituted in an expression. Use the <code>:</code> character in an expression to dereference an attribute value.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
@@ -310,7 +316,8 @@
     /// Consumes the builder and constructs a [`SearchVectorsInput`](crate::operation::search_vectors::SearchVectorsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::search_vectors::SearchVectorsInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::super::operation::search_vectors::SearchVectorsInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::super::operation::search_vectors::SearchVectorsInput {
             table_name: self.table_name,
             index_name: self.index_name,
```

### `src/operation/search_vectors/builders.rs`

```diff
--- reference/src/operation/search_vectors/builders.rs
+++ generated/src/operation/search_vectors/builders.rs
@@ -220,7 +220,11 @@
     /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
     ///
     /// <p>One or more values that can be substituted in an expression. Use the <code>:</code> character in an expression to dereference an attribute value.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         self.inner = self.inner.expression_attribute_values(k.into(), v);
         self
     }
```

### `src/operation/tag_resource/_tag_resource_input.rs`

```diff
--- reference/src/operation/tag_resource/_tag_resource_input.rs
+++ generated/src/operation/tag_resource/_tag_resource_input.rs
@@ -71,7 +71,9 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`TagResourceInput`](crate::operation::tag_resource::TagResourceInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::tag_resource::TagResourceInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::tag_resource::TagResourceInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::tag_resource::TagResourceInput {
             resource_arn: self.resource_arn,
             tags: self.tags,
```

### `src/operation/transact_get_items/_transact_get_items_input.rs`

```diff
--- reference/src/operation/transact_get_items/_transact_get_items_input.rs
+++ generated/src/operation/transact_get_items/_transact_get_items_input.rs
@@ -72,7 +72,10 @@
     /// Consumes the builder and constructs a [`TransactGetItemsInput`](crate::operation::transact_get_items::TransactGetItemsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::transact_get_items::TransactGetItemsInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::transact_get_items::TransactGetItemsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::transact_get_items::TransactGetItemsInput {
             transact_items: self.transact_items,
             return_consumed_capacity: self.return_consumed_capacity,
```

### `src/operation/transact_get_items.rs`

```diff
--- reference/src/operation/transact_get_items.rs
+++ generated/src/operation/transact_get_items.rs
@@ -212,7 +212,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_transact_get_items::ser_transact_get_items_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_transact_get_items::ser_transact_get_items_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -268,9 +270,10 @@

 // The get_* functions below are generated from JMESPath expressions in the
 // operationContextParams trait. They target the operation's input shape.
-
 // Generated from JMESPath Expression: TransactItems[*].Get.TableName
-fn get_resource_arn_list(input: &super::super::operation::transact_get_items::TransactGetItemsInput) -> Option<::std::vec::Vec<&::std::string::String>> {
+fn get_resource_arn_list(
+    input: &super::super::operation::transact_get_items::TransactGetItemsInput,
+) -> Option<::std::vec::Vec<&::std::string::String>> {
     let _fld_1 = input.transact_items.as_ref()?;
     let _prj_4 = _fld_1
         .iter()
```

### `src/operation/transact_write_items/_transact_write_items_input.rs`

```diff
--- reference/src/operation/transact_write_items/_transact_write_items_input.rs
+++ generated/src/operation/transact_write_items/_transact_write_items_input.rs
@@ -140,7 +140,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections (if any), that were modified during the operation and are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.return_item_collection_metrics = input;
         self
     }
@@ -174,8 +177,10 @@
     /// Consumes the builder and constructs a [`TransactWriteItemsInput`](crate::operation::transact_write_items::TransactWriteItemsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::transact_write_items::TransactWriteItemsInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        super::super::super::operation::transact_write_items::TransactWriteItemsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::transact_write_items::TransactWriteItemsInput {
             transact_items: self.transact_items,
             return_consumed_capacity: self.return_consumed_capacity,
```

### `src/operation/transact_write_items/_transact_write_items_output.rs`

```diff
--- reference/src/operation/transact_write_items/_transact_write_items_output.rs
+++ generated/src/operation/transact_write_items/_transact_write_items_output.rs
@@ -22,7 +22,8 @@
     /// <p>A list of tables that were processed by <code>TransactWriteItems</code> and, for each table, information about any item collections that were affected by individual <code>UpdateItem</code>, <code>PutItem</code>, or <code>DeleteItem</code> operations.</p>
     pub fn item_collection_metrics(
         &self,
-    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>> {
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>>
+    {
         self.item_collection_metrics.as_ref()
     }
 }
@@ -89,7 +90,9 @@
     /// <p>A list of tables that were processed by <code>TransactWriteItems</code> and, for each table, information about any item collections that were affected by individual <code>UpdateItem</code>, <code>PutItem</code>, or <code>DeleteItem</code> operations.</p>
     pub fn set_item_collection_metrics(
         mut self,
-        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>>,
+        input: ::std::option::Option<
+            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>,
+        >,
     ) -> Self {
         self.item_collection_metrics = input;
         self
@@ -97,7 +100,8 @@
     /// <p>A list of tables that were processed by <code>TransactWriteItems</code> and, for each table, information about any item collections that were affected by individual <code>UpdateItem</code>, <code>PutItem</code>, or <code>DeleteItem</code> operations.</p>
     pub fn get_item_collection_metrics(
         &self,
-    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>> {
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::super::super::types::ItemCollectionMetrics>>>
+    {
         &self.item_collection_metrics
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/transact_write_items/builders.rs`

```diff
--- reference/src/operation/transact_write_items/builders.rs
+++ generated/src/operation/transact_write_items/builders.rs
@@ -200,7 +200,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections (if any), that were modified during the operation and are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.inner = self.inner.set_return_item_collection_metrics(input);
         self
     }
```

### `src/operation/transact_write_items.rs`

```diff
--- reference/src/operation/transact_write_items.rs
+++ generated/src/operation/transact_write_items.rs
@@ -265,8 +265,9 @@
             );
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_transact_write_items::ser_transact_write_items_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_transact_write_items::ser_transact_write_items_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -322,15 +323,18 @@

 // The get_* functions below are generated from JMESPath expressions in the
 // operationContextParams trait. They target the operation's input shape.
-
 // Generated from JMESPath Expression: TransactItems[*].[ConditionCheck.TableName, Put.TableName, Delete.TableName, Update.TableName][]
-fn get_resource_arn_list(input: &super::super::operation::transact_write_items::TransactWriteItemsInput) -> Option<::std::vec::Vec<&::std::string::String>> {
+fn get_resource_arn_list(
+    input: &super::super::operation::transact_write_items::TransactWriteItemsInput,
+) -> Option<::std::vec::Vec<&::std::string::String>> {
     let _fld_1 = input.transact_items.as_ref()?;
     let _prj_11 = _fld_1
         .iter()
         .flat_map(|v| {
             #[allow(clippy::let_and_return)]
-            fn map(_v: &super::super::types::TransactWriteItem) -> ::std::option::Option<::std::vec::Vec<::std::option::Option<&::std::string::String>>> {
+            fn map(
+                _v: &super::super::types::TransactWriteItem,
+            ) -> ::std::option::Option<::std::vec::Vec<::std::option::Option<&::std::string::String>>> {
                 let _fld_2 = _v.condition_check.as_ref();
                 let _fld_3 = _fld_2.map(|v| &v.table_name);
                 let _fld_4 = _v.put.as_ref();
```

### `src/operation/update_continuous_backups/_update_continuous_backups_input.rs`

```diff
--- reference/src/operation/update_continuous_backups/_update_continuous_backups_input.rs
+++ generated/src/operation/update_continuous_backups/_update_continuous_backups_input.rs
@@ -55,7 +55,10 @@
         self
     }
     /// <p>Represents the settings used to enable point in time recovery.</p>
-    pub fn set_point_in_time_recovery_specification(mut self, input: ::std::option::Option<super::super::super::types::PointInTimeRecoverySpecification>) -> Self {
+    pub fn set_point_in_time_recovery_specification(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::PointInTimeRecoverySpecification>,
+    ) -> Self {
         self.point_in_time_recovery_specification = input;
         self
     }
```

### `src/operation/update_continuous_backups/_update_continuous_backups_output.rs`

```diff
--- reference/src/operation/update_continuous_backups/_update_continuous_backups_output.rs
+++ generated/src/operation/update_continuous_backups/_update_continuous_backups_output.rs
@@ -39,7 +39,10 @@
         self
     }
     /// <p>Represents the continuous backups and point in time recovery settings on the table.</p>
-    pub fn set_continuous_backups_description(mut self, input: ::std::option::Option<super::super::super::types::ContinuousBackupsDescription>) -> Self {
+    pub fn set_continuous_backups_description(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ContinuousBackupsDescription>,
+    ) -> Self {
         self.continuous_backups_description = input;
         self
     }
```

### `src/operation/update_continuous_backups/builders.rs`

```diff
--- reference/src/operation/update_continuous_backups/builders.rs
+++ generated/src/operation/update_continuous_backups/builders.rs
@@ -130,7 +130,10 @@
         self
     }
     /// <p>Represents the settings used to enable point in time recovery.</p>
-    pub fn set_point_in_time_recovery_specification(mut self, input: ::std::option::Option<super::super::super::types::PointInTimeRecoverySpecification>) -> Self {
+    pub fn set_point_in_time_recovery_specification(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::PointInTimeRecoverySpecification>,
+    ) -> Self {
         self.inner = self.inner.set_point_in_time_recovery_specification(input);
         self
     }
```

### `src/operation/update_contributor_insights/_update_contributor_insights_input.rs`

```diff
--- reference/src/operation/update_contributor_insights/_update_contributor_insights_input.rs
+++ generated/src/operation/update_contributor_insights/_update_contributor_insights_input.rs
@@ -112,11 +112,13 @@
         super::super::super::operation::update_contributor_insights::UpdateContributorInsightsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::update_contributor_insights::UpdateContributorInsightsInput {
-            table_name: self.table_name,
-            index_name: self.index_name,
-            contributor_insights_action: self.contributor_insights_action,
-            contributor_insights_mode: self.contributor_insights_mode,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::update_contributor_insights::UpdateContributorInsightsInput {
+                table_name: self.table_name,
+                index_name: self.index_name,
+                contributor_insights_action: self.contributor_insights_action,
+                contributor_insights_mode: self.contributor_insights_mode,
+            },
+        )
     }
 }
```

### `src/operation/update_global_table/_update_global_table_input.rs`

```diff
--- reference/src/operation/update_global_table/_update_global_table_input.rs
+++ generated/src/operation/update_global_table/_update_global_table_input.rs
@@ -73,7 +73,10 @@
     /// Consumes the builder and constructs a [`UpdateGlobalTableInput`](crate::operation::update_global_table::UpdateGlobalTableInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_global_table::UpdateGlobalTableInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_global_table::UpdateGlobalTableInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_global_table::UpdateGlobalTableInput {
             global_table_name: self.global_table_name,
             replica_updates: self.replica_updates,
```

### `src/operation/update_global_table.rs`

```diff
--- reference/src/operation/update_global_table.rs
+++ generated/src/operation/update_global_table.rs
@@ -258,7 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_global_table::ser_update_global_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_global_table::ser_update_global_table_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/update_global_table_settings/_update_global_table_settings_input.rs`

```diff
--- reference/src/operation/update_global_table_settings/_update_global_table_settings_input.rs
+++ generated/src/operation/update_global_table_settings/_update_global_table_settings_input.rs
@@ -16,7 +16,8 @@
     /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code></p>
     pub global_table_provisioned_write_capacity_units: ::std::option::Option<i64>,
     /// <p>Auto scaling settings for managing provisioned write capacity for the global table.</p>
-    pub global_table_provisioned_write_capacity_auto_scaling_settings_update: ::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate>,
+    pub global_table_provisioned_write_capacity_auto_scaling_settings_update:
+        ::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate>,
     /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
     pub global_table_global_secondary_index_settings_update:
         ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>>,
@@ -51,7 +52,9 @@
     /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
     ///
     /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.global_table_global_secondary_index_settings_update.is_none()`.
-    pub fn global_table_global_secondary_index_settings_update(&self) -> &[super::super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate] {
+    pub fn global_table_global_secondary_index_settings_update(
+        &self,
+    ) -> &[super::super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate] {
         self.global_table_global_secondary_index_settings_update.as_deref().unwrap_or_default()
     }
     /// <p>Represents the settings for a global table in a Region that will be modified.</p>
@@ -75,7 +78,8 @@
     pub(crate) global_table_name: ::std::option::Option<::std::string::String>,
     pub(crate) global_table_billing_mode: ::std::option::Option<super::super::super::types::BillingMode>,
     pub(crate) global_table_provisioned_write_capacity_units: ::std::option::Option<i64>,
-    pub(crate) global_table_provisioned_write_capacity_auto_scaling_settings_update: ::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate>,
+    pub(crate) global_table_provisioned_write_capacity_auto_scaling_settings_update:
+        ::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate>,
     pub(crate) global_table_global_secondary_index_settings_update:
         ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>>,
     pub(crate) replica_settings_update: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsUpdate>>,
@@ -143,7 +147,10 @@
         &self.global_table_provisioned_write_capacity_units
     }
     /// <p>Auto scaling settings for managing provisioned write capacity for the global table.</p>
-    pub fn global_table_provisioned_write_capacity_auto_scaling_settings_update(mut self, input: super::super::super::types::AutoScalingSettingsUpdate) -> Self {
+    pub fn global_table_provisioned_write_capacity_auto_scaling_settings_update(
+        mut self,
+        input: super::super::super::types::AutoScalingSettingsUpdate,
+    ) -> Self {
         self.global_table_provisioned_write_capacity_auto_scaling_settings_update = ::std::option::Option::Some(input);
         self
     }
@@ -166,7 +173,10 @@
     /// To override the contents of this collection use [`set_global_table_global_secondary_index_settings_update`](Self::set_global_table_global_secondary_index_settings_update).
     ///
     /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
-    pub fn global_table_global_secondary_index_settings_update(mut self, input: super::super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate) -> Self {
+    pub fn global_table_global_secondary_index_settings_update(
+        mut self,
+        input: super::super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate,
+    ) -> Self {
         let mut v = self.global_table_global_secondary_index_settings_update.unwrap_or_default();
         v.push(input);
         self.global_table_global_secondary_index_settings_update = ::std::option::Option::Some(v);
@@ -198,7 +208,10 @@
         self
     }
     /// <p>Represents the settings for a global table in a Region that will be modified.</p>
-    pub fn set_replica_settings_update(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsUpdate>>) -> Self {
+    pub fn set_replica_settings_update(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsUpdate>>,
+    ) -> Self {
         self.replica_settings_update = input;
         self
     }
@@ -213,14 +226,16 @@
         super::super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsInput {
-            global_table_name: self.global_table_name,
-            global_table_billing_mode: self.global_table_billing_mode,
-            global_table_provisioned_write_capacity_units: self.global_table_provisioned_write_capacity_units,
-            global_table_provisioned_write_capacity_auto_scaling_settings_update: self
-                .global_table_provisioned_write_capacity_auto_scaling_settings_update,
-            global_table_global_secondary_index_settings_update: self.global_table_global_secondary_index_settings_update,
-            replica_settings_update: self.replica_settings_update,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsInput {
+                global_table_name: self.global_table_name,
+                global_table_billing_mode: self.global_table_billing_mode,
+                global_table_provisioned_write_capacity_units: self.global_table_provisioned_write_capacity_units,
+                global_table_provisioned_write_capacity_auto_scaling_settings_update: self
+                    .global_table_provisioned_write_capacity_auto_scaling_settings_update,
+                global_table_global_secondary_index_settings_update: self.global_table_global_secondary_index_settings_update,
+                replica_settings_update: self.replica_settings_update,
+            },
+        )
     }
 }
```

### `src/operation/update_global_table_settings/_update_global_table_settings_output.rs`

```diff
--- reference/src/operation/update_global_table_settings/_update_global_table_settings_output.rs
+++ generated/src/operation/update_global_table_settings/_update_global_table_settings_output.rs
@@ -68,7 +68,10 @@
         self
     }
     /// <p>The Region-specific settings for the global table.</p>
-    pub fn set_replica_settings(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsDescription>>) -> Self {
+    pub fn set_replica_settings(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsDescription>>,
+    ) -> Self {
         self.replica_settings = input;
         self
     }
```

### `src/operation/update_global_table_settings/builders.rs`

```diff
--- reference/src/operation/update_global_table_settings/builders.rs
+++ generated/src/operation/update_global_table_settings/builders.rs
@@ -172,7 +172,10 @@
         self.inner.get_global_table_provisioned_write_capacity_units()
     }
     /// <p>Auto scaling settings for managing provisioned write capacity for the global table.</p>
-    pub fn global_table_provisioned_write_capacity_auto_scaling_settings_update(mut self, input: super::super::super::types::AutoScalingSettingsUpdate) -> Self {
+    pub fn global_table_provisioned_write_capacity_auto_scaling_settings_update(
+        mut self,
+        input: super::super::super::types::AutoScalingSettingsUpdate,
+    ) -> Self {
         self.inner = self.inner.global_table_provisioned_write_capacity_auto_scaling_settings_update(input);
         self
     }
@@ -196,7 +199,10 @@
     /// To override the contents of this collection use [`set_global_table_global_secondary_index_settings_update`](Self::set_global_table_global_secondary_index_settings_update).
     ///
     /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
-    pub fn global_table_global_secondary_index_settings_update(mut self, input: super::super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate) -> Self {
+    pub fn global_table_global_secondary_index_settings_update(
+        mut self,
+        input: super::super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate,
+    ) -> Self {
         self.inner = self.inner.global_table_global_secondary_index_settings_update(input);
         self
     }
@@ -225,7 +231,10 @@
         self
     }
     /// <p>Represents the settings for a global table in a Region that will be modified.</p>
-    pub fn set_replica_settings_update(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsUpdate>>) -> Self {
+    pub fn set_replica_settings_update(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaSettingsUpdate>>,
+    ) -> Self {
         self.inner = self.inner.set_replica_settings_update(input);
         self
     }
```

### `src/operation/update_item/_update_item_input.rs`

```diff
--- reference/src/operation/update_item/_update_item_input.rs
+++ generated/src/operation/update_item/_update_item_input.rs
@@ -10,7 +10,8 @@
     /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
     pub key: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     /// <p>This is a legacy parameter. Use <code>UpdateExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributeUpdates.html">AttributeUpdates</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub attribute_updates: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValueUpdate>>,
+    pub attribute_updates:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValueUpdate>>,
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     pub expected: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>>,
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -131,7 +132,8 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     /// <p>An optional parameter that returns the item attributes for an <code>UpdateItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
     pub return_values_on_condition_check_failure: ::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure>,
@@ -153,7 +155,9 @@
         self.attribute_updates.as_ref()
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expected(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn expected(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         self.expected.as_ref()
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -295,7 +299,9 @@
     }
     /// <p>An optional parameter that returns the item attributes for an <code>UpdateItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn return_values_on_condition_check_failure(&self) -> ::std::option::Option<&super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn return_values_on_condition_check_failure(
+        &self,
+    ) -> ::std::option::Option<&super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         self.return_values_on_condition_check_failure.as_ref()
     }
 }
@@ -312,8 +318,10 @@
 pub struct UpdateItemInputBuilder {
     pub(crate) table_name: ::std::option::Option<::std::string::String>,
     pub(crate) key: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
-    pub(crate) attribute_updates: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValueUpdate>>,
-    pub(crate) expected: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>>,
+    pub(crate) attribute_updates:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValueUpdate>>,
+    pub(crate) expected:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>>,
     pub(crate) conditional_operator: ::std::option::Option<super::super::super::types::ConditionalOperator>,
     pub(crate) return_values: ::std::option::Option<super::super::super::types::ReturnValue>,
     pub(crate) return_consumed_capacity: ::std::option::Option<super::super::super::types::ReturnConsumedCapacity>,
@@ -321,7 +329,8 @@
     pub(crate) update_expression: ::std::option::Option<::std::string::String>,
     pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
     pub(crate) return_values_on_condition_check_failure: ::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure>,
 }
 impl UpdateItemInputBuilder {
@@ -354,7 +363,10 @@
     }
     /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
     /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    ) -> Self {
         self.key = input;
         self
     }
@@ -368,7 +380,11 @@
     /// To override the contents of this collection use [`set_attribute_updates`](Self::set_attribute_updates).
     ///
     /// <p>This is a legacy parameter. Use <code>UpdateExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributeUpdates.html">AttributeUpdates</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn attribute_updates(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValueUpdate) -> Self {
+    pub fn attribute_updates(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValueUpdate,
+    ) -> Self {
         let mut hash_map = self.attribute_updates.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.attribute_updates = ::std::option::Option::Some(hash_map);
@@ -408,7 +424,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn get_expected(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         &self.expected
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -528,7 +546,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.return_item_collection_metrics = input;
         self
     }
@@ -824,7 +845,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
@@ -875,11 +900,15 @@
     }
     /// <p>An optional parameter that returns the item attributes for an <code>UpdateItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn get_return_values_on_condition_check_failure(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         &self.return_values_on_condition_check_failure
     }
     /// Consumes the builder and constructs a [`UpdateItemInput`](crate::operation::update_item::UpdateItemInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::update_item::UpdateItemInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::update_item::UpdateItemInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::update_item::UpdateItemInput {
             table_name: self.table_name,
             key: self.key,
```

### `src/operation/update_item/_update_item_output.rs`

```diff
--- reference/src/operation/update_item/_update_item_output.rs
+++ generated/src/operation/update_item/_update_item_output.rs
@@ -25,7 +25,9 @@
 impl UpdateItemOutput {
     /// <p>A map of attribute values as they appear before or after the <code>UpdateItem</code> operation, as determined by the <code>ReturnValues</code> parameter.</p>
     /// <p>The <code>Attributes</code> map is only present if the update was successful and <code>ReturnValues</code> was specified as something other than <code>NONE</code> in the request. Each element represents one attribute.</p>
-    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn attributes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         self.attributes.as_ref()
     }
     /// <p>The capacity units consumed by the <code>UpdateItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#write-operation-consumption">Capacity unity consumption for write operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -91,7 +93,9 @@
     }
     /// <p>A map of attribute values as they appear before or after the <code>UpdateItem</code> operation, as determined by the <code>ReturnValues</code> parameter.</p>
     /// <p>The <code>Attributes</code> map is only present if the update was successful and <code>ReturnValues</code> was specified as something other than <code>NONE</code> in the request. Each element represents one attribute.</p>
-    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
+    pub fn get_attributes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>> {
         &self.attributes
     }
     /// <p>The capacity units consumed by the <code>UpdateItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#write-operation-consumption">Capacity unity consumption for write operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
```

### `src/operation/update_item/builders.rs`

```diff
--- reference/src/operation/update_item/builders.rs
+++ generated/src/operation/update_item/builders.rs
@@ -136,7 +136,10 @@
     }
     /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
     /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::AttributeValue>>,
+    ) -> Self {
         self.inner = self.inner.set_key(input);
         self
     }
@@ -151,7 +154,11 @@
     /// To override the contents of this collection use [`set_attribute_updates`](Self::set_attribute_updates).
     ///
     /// <p>This is a legacy parameter. Use <code>UpdateExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributeUpdates.html">AttributeUpdates</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn attribute_updates(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValueUpdate) -> Self {
+    pub fn attribute_updates(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValueUpdate,
+    ) -> Self {
         self.inner = self.inner.attribute_updates(k.into(), v);
         self
     }
@@ -188,7 +195,9 @@
         self
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
+    pub fn get_expected(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::super::types::ExpectedAttributeValue>> {
         self.inner.get_expected()
     }
     /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -308,7 +317,10 @@
         self
     }
     /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
-    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>) -> Self {
+    pub fn set_return_item_collection_metrics(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::ReturnItemCollectionMetrics>,
+    ) -> Self {
         self.inner = self.inner.set_return_item_collection_metrics(input);
         self
     }
@@ -604,7 +616,11 @@
     /// <p>You could then use these values in an expression, such as this:</p>
     /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
     /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::super::types::AttributeValue,
+    ) -> Self {
         self.inner = self.inner.expression_attribute_values(k.into(), v);
         self
     }
@@ -653,7 +669,9 @@
     }
     /// <p>An optional parameter that returns the item attributes for an <code>UpdateItem</code> operation that failed a condition check.</p>
     /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
-    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
+    pub fn get_return_values_on_condition_check_failure(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::ReturnValuesOnConditionCheckFailure> {
         self.inner.get_return_values_on_condition_check_failure()
     }
 }
```

### `src/operation/update_kinesis_streaming_destination/_update_kinesis_streaming_destination_input.rs`

```diff
--- reference/src/operation/update_kinesis_streaming_destination/_update_kinesis_streaming_destination_input.rs
+++ generated/src/operation/update_kinesis_streaming_destination/_update_kinesis_streaming_destination_input.rs
@@ -26,7 +26,8 @@
 }
 impl UpdateKinesisStreamingDestinationInput {
     /// Creates a new builder-style object to manufacture [`UpdateKinesisStreamingDestinationInput`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationInput).
-    pub fn builder() -> super::super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationInputBuilder {
+    pub fn builder() -> super::super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationInputBuilder
+    {
         super::super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationInputBuilder::default()
     }
 }
@@ -84,7 +85,9 @@
         self
     }
     /// <p>The command to update the Kinesis stream configuration.</p>
-    pub fn get_update_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::UpdateKinesisStreamingConfiguration> {
+    pub fn get_update_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::UpdateKinesisStreamingConfiguration> {
         &self.update_kinesis_streaming_configuration
     }
     /// Consumes the builder and constructs a [`UpdateKinesisStreamingDestinationInput`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationInput).
```

### `src/operation/update_kinesis_streaming_destination/_update_kinesis_streaming_destination_output.rs`

```diff
--- reference/src/operation/update_kinesis_streaming_destination/_update_kinesis_streaming_destination_output.rs
+++ generated/src/operation/update_kinesis_streaming_destination/_update_kinesis_streaming_destination_output.rs
@@ -38,7 +38,8 @@
 }
 impl UpdateKinesisStreamingDestinationOutput {
     /// Creates a new builder-style object to manufacture [`UpdateKinesisStreamingDestinationOutput`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput).
-    pub fn builder() -> super::super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationOutputBuilder {
+    pub fn builder() -> super::super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationOutputBuilder
+    {
         super::super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationOutputBuilder::default()
     }
 }
@@ -110,7 +111,9 @@
         self
     }
     /// <p>The command to update the Kinesis streaming destination configuration.</p>
-    pub fn get_update_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::UpdateKinesisStreamingConfiguration> {
+    pub fn get_update_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::UpdateKinesisStreamingConfiguration> {
         &self.update_kinesis_streaming_configuration
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/update_kinesis_streaming_destination/builders.rs`

```diff
--- reference/src/operation/update_kinesis_streaming_destination/builders.rs
+++ generated/src/operation/update_kinesis_streaming_destination/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the UpdateKinesisStreamingDestination as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestination::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        super::super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestination::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            super::super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestination::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        super::super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestination::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -150,7 +154,9 @@
         self
     }
     /// <p>The command to update the Kinesis stream configuration.</p>
-    pub fn get_update_kinesis_streaming_configuration(&self) -> &::std::option::Option<super::super::super::types::UpdateKinesisStreamingConfiguration> {
+    pub fn get_update_kinesis_streaming_configuration(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::UpdateKinesisStreamingConfiguration> {
         self.inner.get_update_kinesis_streaming_configuration()
     }
 }
```

### `src/operation/update_table/_update_table_input.rs`

```diff
--- reference/src/operation/update_table/_update_table_input.rs
+++ generated/src/operation/update_table/_update_table_input.rs
@@ -236,7 +236,10 @@
         self
     }
     /// <p>An array of attributes that describe the key schema for the table and indexes. If you are adding a new global secondary index to the table, <code>AttributeDefinitions</code> must include the key element(s) of the new index.</p>
-    pub fn set_attribute_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeDefinition>>) -> Self {
+    pub fn set_attribute_definitions(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeDefinition>>,
+    ) -> Self {
         self.attribute_definitions = input;
         self
     }
@@ -355,7 +358,9 @@
     /// </ul>
     /// <p>You can create or delete only one global secondary index per <code>UpdateTable</code> operation.</p>
     /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.OnlineOps.html">Managing Global Secondary Indexes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexUpdate>> {
+    pub fn get_global_secondary_index_updates(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexUpdate>> {
         &self.global_secondary_index_updates
     }
     /// <p>Represents the DynamoDB Streams configuration for the table.</p><note>
@@ -522,7 +527,9 @@
     /// </ul>
     /// <p>You can create or delete only one witness per <code>UpdateTable</code> operation.</p>
     /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/V2globaltables_HowItWorks.html#V2globaltables_HowItWorks.consistency-modes">Multi-Region strong consistency (MRSC)</a> in the Amazon DynamoDB Developer Guide</p>
-    pub fn get_global_table_witness_updates(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalTableWitnessGroupUpdate>> {
+    pub fn get_global_table_witness_updates(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalTableWitnessGroupUpdate>> {
         &self.global_table_witness_updates
     }
     /// <p>Updates the maximum number of read and write units for the specified table in on-demand capacity mode. If you use this parameter, you must specify <code>MaxReadRequestUnits</code>, <code>MaxWriteRequestUnits</code>, or both.</p>
@@ -585,7 +592,9 @@
     /// <li>
     /// <p><code>DISABLED</code>: Remove settings replication on a regional table. Settings replication needs to be defined to ENABLED again in order to create a Multi-Account Global Table using this table.</p></li>
     /// </ul>
-    pub fn get_global_table_settings_replication_mode(&self) -> &::std::option::Option<super::super::super::types::GlobalTableSettingsReplicationMode> {
+    pub fn get_global_table_settings_replication_mode(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::GlobalTableSettingsReplicationMode> {
         &self.global_table_settings_replication_mode
     }
     /// Appends an item to `vector_index_updates`.
@@ -612,7 +621,9 @@
         &self.vector_index_updates
     }
     /// Consumes the builder and constructs a [`UpdateTableInput`](crate::operation::update_table::UpdateTableInput).
-    pub fn build(self) -> ::std::result::Result<super::super::super::operation::update_table::UpdateTableInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::update_table::UpdateTableInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::super::operation::update_table::UpdateTableInput {
             attribute_definitions: self.attribute_definitions,
             table_name: self.table_name,
```

### `src/operation/update_table/builders.rs`

```diff
--- reference/src/operation/update_table/builders.rs
+++ generated/src/operation/update_table/builders.rs
@@ -129,7 +129,10 @@
         self
     }
     /// <p>An array of attributes that describe the key schema for the table and indexes. If you are adding a new global secondary index to the table, <code>AttributeDefinitions</code> must include the key element(s) of the new index.</p>
-    pub fn set_attribute_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeDefinition>>) -> Self {
+    pub fn set_attribute_definitions(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::AttributeDefinition>>,
+    ) -> Self {
         self.inner = self.inner.set_attribute_definitions(input);
         self
     }
@@ -246,7 +249,9 @@
     /// </ul>
     /// <p>You can create or delete only one global secondary index per <code>UpdateTable</code> operation.</p>
     /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.OnlineOps.html">Managing Global Secondary Indexes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn get_global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexUpdate>> {
+    pub fn get_global_secondary_index_updates(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexUpdate>> {
         self.inner.get_global_secondary_index_updates()
     }
     /// <p>Represents the DynamoDB Streams configuration for the table.</p><note>
@@ -411,7 +416,9 @@
     /// </ul>
     /// <p>You can create or delete only one witness per <code>UpdateTable</code> operation.</p>
     /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/V2globaltables_HowItWorks.html#V2globaltables_HowItWorks.consistency-modes">Multi-Region strong consistency (MRSC)</a> in the Amazon DynamoDB Developer Guide</p>
-    pub fn get_global_table_witness_updates(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalTableWitnessGroupUpdate>> {
+    pub fn get_global_table_witness_updates(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalTableWitnessGroupUpdate>> {
         self.inner.get_global_table_witness_updates()
     }
     /// <p>Updates the maximum number of read and write units for the specified table in on-demand capacity mode. If you use this parameter, you must specify <code>MaxReadRequestUnits</code>, <code>MaxWriteRequestUnits</code>, or both.</p>
@@ -474,7 +481,9 @@
     /// <li>
     /// <p><code>DISABLED</code>: Remove settings replication on a regional table. Settings replication needs to be defined to ENABLED again in order to create a Multi-Account Global Table using this table.</p></li>
     /// </ul>
-    pub fn get_global_table_settings_replication_mode(&self) -> &::std::option::Option<super::super::super::types::GlobalTableSettingsReplicationMode> {
+    pub fn get_global_table_settings_replication_mode(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::GlobalTableSettingsReplicationMode> {
         self.inner.get_global_table_settings_replication_mode()
     }
     ///
```

### `src/operation/update_table_replica_auto_scaling/_update_table_replica_auto_scaling_input.rs`

```diff
--- reference/src/operation/update_table_replica_auto_scaling/_update_table_replica_auto_scaling_input.rs
+++ generated/src/operation/update_table_replica_auto_scaling/_update_table_replica_auto_scaling_input.rs
@@ -45,7 +45,8 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct UpdateTableReplicaAutoScalingInputBuilder {
-    pub(crate) global_secondary_index_updates: ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexAutoScalingUpdate>>,
+    pub(crate) global_secondary_index_updates:
+        ::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexAutoScalingUpdate>>,
     pub(crate) table_name: ::std::option::Option<::std::string::String>,
     pub(crate) provisioned_write_capacity_auto_scaling_update: ::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate>,
     pub(crate) replica_updates: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaAutoScalingUpdate>>,
@@ -71,7 +72,9 @@
         self
     }
     /// <p>Represents the auto scaling settings of the global secondary indexes of the replica to be updated.</p>
-    pub fn get_global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexAutoScalingUpdate>> {
+    pub fn get_global_secondary_index_updates(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexAutoScalingUpdate>> {
         &self.global_secondary_index_updates
     }
     /// <p>The name of the global table to be updated. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
@@ -103,7 +106,9 @@
         self
     }
     /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
-    pub fn get_provisioned_write_capacity_auto_scaling_update(&self) -> &::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate> {
+    pub fn get_provisioned_write_capacity_auto_scaling_update(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate> {
         &self.provisioned_write_capacity_auto_scaling_update
     }
     /// Appends an item to `replica_updates`.
@@ -118,7 +123,10 @@
         self
     }
     /// <p>Represents the auto scaling settings of replicas of the table that will be modified.</p>
-    pub fn set_replica_updates(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaAutoScalingUpdate>>) -> Self {
+    pub fn set_replica_updates(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaAutoScalingUpdate>>,
+    ) -> Self {
         self.replica_updates = input;
         self
     }
@@ -133,11 +141,13 @@
         super::super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(super::super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput {
-            global_secondary_index_updates: self.global_secondary_index_updates,
-            table_name: self.table_name,
-            provisioned_write_capacity_auto_scaling_update: self.provisioned_write_capacity_auto_scaling_update,
-            replica_updates: self.replica_updates,
-        })
+        ::std::result::Result::Ok(
+            super::super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput {
+                global_secondary_index_updates: self.global_secondary_index_updates,
+                table_name: self.table_name,
+                provisioned_write_capacity_auto_scaling_update: self.provisioned_write_capacity_auto_scaling_update,
+                replica_updates: self.replica_updates,
+            },
+        )
     }
 }
```

### `src/operation/update_table_replica_auto_scaling/_update_table_replica_auto_scaling_output.rs`

```diff
--- reference/src/operation/update_table_replica_auto_scaling/_update_table_replica_auto_scaling_output.rs
+++ generated/src/operation/update_table_replica_auto_scaling/_update_table_replica_auto_scaling_output.rs
@@ -39,7 +39,10 @@
         self
     }
     /// <p>Returns information about the auto scaling settings of a table with replicas.</p>
-    pub fn set_table_auto_scaling_description(mut self, input: ::std::option::Option<super::super::super::types::TableAutoScalingDescription>) -> Self {
+    pub fn set_table_auto_scaling_description(
+        mut self,
+        input: ::std::option::Option<super::super::super::types::TableAutoScalingDescription>,
+    ) -> Self {
         self.table_auto_scaling_description = input;
         self
     }
```

### `src/operation/update_table_replica_auto_scaling/builders.rs`

```diff
--- reference/src/operation/update_table_replica_auto_scaling/builders.rs
+++ generated/src/operation/update_table_replica_auto_scaling/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the UpdateTableReplicaAutoScaling as a reference.
-    pub fn as_input(&self) -> &super::super::super::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::super::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,11 +83,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = super::super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScaling::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            super::super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScaling::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         super::super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScaling::orchestrate(&runtime_plugins, input).await
     }

@@ -127,7 +130,9 @@
         self
     }
     /// <p>Represents the auto scaling settings of the global secondary indexes of the replica to be updated.</p>
-    pub fn get_global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexAutoScalingUpdate>> {
+    pub fn get_global_secondary_index_updates(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::super::types::GlobalSecondaryIndexAutoScalingUpdate>> {
         self.inner.get_global_secondary_index_updates()
     }
     /// <p>The name of the global table to be updated. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
@@ -158,7 +163,9 @@
         self
     }
     /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
-    pub fn get_provisioned_write_capacity_auto_scaling_update(&self) -> &::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate> {
+    pub fn get_provisioned_write_capacity_auto_scaling_update(
+        &self,
+    ) -> &::std::option::Option<super::super::super::types::AutoScalingSettingsUpdate> {
         self.inner.get_provisioned_write_capacity_auto_scaling_update()
     }
     ///
@@ -172,7 +179,10 @@
         self
     }
     /// <p>Represents the auto scaling settings of replicas of the table that will be modified.</p>
-    pub fn set_replica_updates(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaAutoScalingUpdate>>) -> Self {
+    pub fn set_replica_updates(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::super::types::ReplicaAutoScalingUpdate>>,
+    ) -> Self {
         self.inner = self.inner.set_replica_updates(input);
         self
     }
```

### `src/operation/update_table_replica_auto_scaling.rs`

```diff
--- reference/src/operation/update_table_replica_auto_scaling.rs
+++ generated/src/operation/update_table_replica_auto_scaling.rs
@@ -213,9 +213,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            super::super::protocol_serde::shape_update_table_replica_auto_scaling::de_update_table_replica_auto_scaling_http_error(status, headers, body)
+            super::super::protocol_serde::shape_update_table_replica_auto_scaling::de_update_table_replica_auto_scaling_http_error(
+                status, headers, body,
+            )
         } else {
-            super::super::protocol_serde::shape_update_table_replica_auto_scaling::de_update_table_replica_auto_scaling_http_response(status, headers, body)
+            super::super::protocol_serde::shape_update_table_replica_auto_scaling::de_update_table_replica_auto_scaling_http_response(
+                status, headers, body,
+            )
         };
         super::super::protocol_serde::type_erase_result(parse_result)
     }
```

### `src/operation/update_time_to_live/_update_time_to_live_input.rs`

```diff
--- reference/src/operation/update_time_to_live/_update_time_to_live_input.rs
+++ generated/src/operation/update_time_to_live/_update_time_to_live_input.rs
@@ -67,7 +67,10 @@
     /// Consumes the builder and constructs a [`UpdateTimeToLiveInput`](crate::operation::update_time_to_live::UpdateTimeToLiveInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::super::operation::update_time_to_live::UpdateTimeToLiveInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_time_to_live::UpdateTimeToLiveInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(super::super::super::operation::update_time_to_live::UpdateTimeToLiveInput {
             table_name: self.table_name,
             time_to_live_specification: self.time_to_live_specification,
```

### `src/operation/update_time_to_live.rs`

```diff
--- reference/src/operation/update_time_to_live.rs
+++ generated/src/operation/update_time_to_live.rs
@@ -258,7 +258,9 @@
             );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_time_to_live::ser_update_time_to_live_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_update_time_to_live::ser_update_time_to_live_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/protocol_serde/shape_attribute_definition.rs`

```diff
--- reference/src/protocol_serde/shape_attribute_definition.rs
+++ generated/src/protocol_serde/shape_attribute_definition.rs
@@ -57,9 +57,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::attribute_definition_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::attribute_definition_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_attribute_value.rs`

```diff
--- reference/src/protocol_serde/shape_attribute_value.rs
+++ generated/src/protocol_serde/shape_attribute_value.rs
@@ -1,20 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_attribute_value(
-    object_6: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::AttributeValue,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::AttributeValue::S(inner) => {
-            object_6.key("S").string(inner.as_str());
+            object.key("S").string(inner.as_str());
         }
         super::super::types::AttributeValue::N(inner) => {
-            object_6.key("N").string(inner.as_str());
+            object.key("N").string(inner.as_str());
         }
         super::super::types::AttributeValue::B(inner) => {
-            object_6.key("B").string_unchecked(&::aws_smithy_types::base64::encode(inner));
+            object.key("B").string_unchecked(&::aws_smithy_types::base64::encode(inner));
         }
         super::super::types::AttributeValue::Ss(inner) => {
-            let mut array_1 = object_6.key("SS").start_array();
+            let mut array_1 = object.key("SS").start_array();
             for item_2 in inner {
                 {
                     array_1.value().string(item_2.as_str());
@@ -23,53 +23,53 @@
             array_1.finish();
         }
         super::super::types::AttributeValue::Ns(inner) => {
-            let mut array_3 = object_6.key("NS").start_array();
-            for item_4 in inner {
+            let mut array_1 = object.key("NS").start_array();
+            for item_2 in inner {
                 {
-                    array_3.value().string(item_4.as_str());
+                    array_1.value().string(item_2.as_str());
                 }
             }
-            array_3.finish();
+            array_1.finish();
         }
         super::super::types::AttributeValue::Bs(inner) => {
-            let mut array_5 = object_6.key("BS").start_array();
-            for item_6 in inner {
+            let mut array_1 = object.key("BS").start_array();
+            for item_2 in inner {
                 {
-                    array_5.value().string_unchecked(&::aws_smithy_types::base64::encode(item_6));
+                    array_1.value().string_unchecked(&::aws_smithy_types::base64::encode(item_2));
                 }
             }
-            array_5.finish();
+            array_1.finish();
         }
         super::super::types::AttributeValue::M(inner) => {
             #[allow(unused_mut)]
-            let mut object_7 = object_6.key("M").start_object();
-            for (key_8, value_9) in inner {
+            let mut object_1 = object.key("M").start_object();
+            for (key_2, value_3) in inner {
                 {
                     #[allow(unused_mut)]
-                    let mut object_10 = object_7.key(key_8.as_str()).start_object();
-                    super::super::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_10, value_9)?;
-                    object_10.finish();
+                    let mut object_4 = object_1.key(key_2.as_str()).start_object();
+                    super::super::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_4, value_3)?;
+                    object_4.finish();
                 }
             }
-            object_7.finish();
+            object_1.finish();
         }
         super::super::types::AttributeValue::L(inner) => {
-            let mut array_11 = object_6.key("L").start_array();
-            for item_12 in inner {
+            let mut array_1 = object.key("L").start_array();
+            for item_2 in inner {
                 {
                     #[allow(unused_mut)]
-                    let mut object_13 = array_11.value().start_object();
-                    super::super::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_13, item_12)?;
-                    object_13.finish();
+                    let mut object_3 = array_1.value().start_object();
+                    super::super::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_3, item_2)?;
+                    object_3.finish();
                 }
             }
-            array_11.finish();
+            array_1.finish();
         }
         super::super::types::AttributeValue::Null(inner) => {
-            object_6.key("NULL").boolean(*inner);
+            object.key("NULL").boolean(*inner);
         }
         super::super::types::AttributeValue::Bool(inner) => {
-            object_6.key("BOOL").boolean(*inner);
+            object.key("BOOL").boolean(*inner);
         }
         super::super::types::AttributeValue::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -100,9 +100,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_auto_scaling_policy_description_list.rs`

```diff
--- reference/src/protocol_serde/shape_auto_scaling_policy_description_list.rs
+++ generated/src/protocol_serde/shape_auto_scaling_policy_description_list.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::AutoScalingPolicyDescription>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::AutoScalingPolicyDescription>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
```

### `src/protocol_serde/shape_auto_scaling_settings_description.rs`

```diff
--- reference/src/protocol_serde/shape_auto_scaling_settings_description.rs
+++ generated/src/protocol_serde/shape_auto_scaling_settings_description.rs
@@ -20,42 +20,39 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "MinimumUnits" => {
-                            builder = builder.set_minimum_units(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i64::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "MaximumUnits" => {
-                            builder = builder.set_maximum_units(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i64::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "AutoScalingDisabled" => {
-                            builder = builder.set_auto_scaling_disabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                        }
-                        "AutoScalingRoleArn" => {
-                            builder = builder.set_auto_scaling_role_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "ScalingPolicies" => {
-                            builder = builder.set_scaling_policies(
-                                super::super::protocol_serde::shape_auto_scaling_policy_description_list::de_auto_scaling_policy_description_list(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "MinimumUnits" => {
+                                builder = builder.set_minimum_units(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i64::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "MaximumUnits" => {
+                                builder = builder.set_maximum_units(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i64::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "AutoScalingDisabled" => {
+                                builder =
+                                    builder.set_auto_scaling_disabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                            }
+                            "AutoScalingRoleArn" => {
+                                builder = builder.set_auto_scaling_role_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "ScalingPolicies" => {
+                                builder = builder.set_scaling_policies(super::super::protocol_serde::shape_auto_scaling_policy_description_list::de_auto_scaling_policy_description_list(tokens, _value, depth + 1)?);
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_backup_description.rs`

```diff
--- reference/src/protocol_serde/shape_backup_description.rs
+++ generated/src/protocol_serde/shape_backup_description.rs
@@ -20,32 +20,30 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        match key.to_unescaped()?.as_ref() {
-                            "BackupDetails" => {
-                                builder = builder.set_backup_details(super::super::protocol_serde::shape_backup_details::de_backup_details(
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "BackupDetails" => {
+                            builder = builder.set_backup_details(super::super::protocol_serde::shape_backup_details::de_backup_details(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
+                        }
+                        "SourceTableDetails" => {
+                            builder = builder.set_source_table_details(
+                                super::super::protocol_serde::shape_source_table_details::de_source_table_details(tokens, _value, depth + 1)?,
+                            );
+                        }
+                        "SourceTableFeatureDetails" => {
+                            builder = builder.set_source_table_feature_details(
+                                super::super::protocol_serde::shape_source_table_feature_details::de_source_table_feature_details(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?);
-                            }
-                            "SourceTableDetails" => {
-                                builder = builder.set_source_table_details(
-                                    super::super::protocol_serde::shape_source_table_details::de_source_table_details(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "SourceTableFeatureDetails" => {
-                                builder = builder.set_source_table_feature_details(
-                                    super::super::protocol_serde::shape_source_table_feature_details::de_source_table_feature_details(
-                                        tokens,
-                                        _value,
-                                        depth + 1,
-                                    )?,
-                                );
-                            }
-                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                                )?,
+                            );
                         }
-                    }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_backup_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_backup_not_found_exception.rs
+++ generated/src/protocol_serde/shape_backup_not_found_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_backup_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::BackupNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::BackupNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::BackupNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_batch_execute_statement.rs`

```diff
--- reference/src/protocol_serde/shape_batch_execute_statement.rs
+++ generated/src/protocol_serde/shape_batch_execute_statement.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::batch_execute_statement::BatchExecuteStatementError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::batch_execute_statement::BatchExecuteStatementError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -113,23 +117,21 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "Responses" => {
-                        builder = builder.set_responses(super::super::protocol_serde::shape_parti_ql_batch_response::de_parti_ql_batch_response(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(
-                            super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Responses" => {
+                    builder = builder.set_responses(super::super::protocol_serde::shape_parti_ql_batch_response::de_parti_ql_batch_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(
+                        super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
+                    );
                 }
-            }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_batch_get_item.rs`

```diff
--- reference/src/protocol_serde/shape_batch_get_item.rs
+++ generated/src/protocol_serde/shape_batch_get_item.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::batch_get_item::BatchGetItemError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::batch_get_item::BatchGetItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +84,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::batch_get_item::BatchGetItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::batch_get_item::BatchGetItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -153,30 +157,28 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "Responses" => {
-                        builder = builder.set_responses(super::super::protocol_serde::shape_batch_get_response_map::de_batch_get_response_map(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
-                    "UnprocessedKeys" => {
-                        builder = builder.set_unprocessed_keys(super::super::protocol_serde::shape_batch_get_request_map::de_batch_get_request_map(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(
-                            super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Responses" => {
+                    builder = builder.set_responses(super::super::protocol_serde::shape_batch_get_response_map::de_batch_get_response_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "UnprocessedKeys" => {
+                    builder = builder.set_unprocessed_keys(super::super::protocol_serde::shape_batch_get_request_map::de_batch_get_request_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(
+                        super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
+                    );
                 }
-            }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_batch_statement_error.rs`

```diff
--- reference/src/protocol_serde/shape_batch_statement_error.rs
+++ generated/src/protocol_serde/shape_batch_statement_error.rs
@@ -24,7 +24,10 @@
                         "Code" => {
                             builder = builder.set_code(
                                 ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| super::super::types::BatchStatementErrorCodeEnum::from(u.as_ref())))
+                                    .map(|s| {
+                                        s.to_unescaped()
+                                            .map(|u| super::super::types::BatchStatementErrorCodeEnum::from(u.as_ref()))
+                                    })
                                     .transpose()?,
                             );
                         }
@@ -36,7 +39,11 @@
                             );
                         }
                         "Item" => {
-                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
+                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
```

### `src/protocol_serde/shape_batch_statement_response.rs`

```diff
--- reference/src/protocol_serde/shape_batch_statement_response.rs
+++ generated/src/protocol_serde/shape_batch_statement_response.rs
@@ -36,7 +36,11 @@
                             );
                         }
                         "Item" => {
-                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
+                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
```

### `src/protocol_serde/shape_batch_write_item.rs`

```diff
--- reference/src/protocol_serde/shape_batch_write_item.rs
+++ generated/src/protocol_serde/shape_batch_write_item.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::batch_write_item::BatchWriteItemOutput, super::super::operation::batch_write_item::BatchWriteItemError> {
+) -> std::result::Result<
+    super::super::operation::batch_write_item::BatchWriteItemOutput,
+    super::super::operation::batch_write_item::BatchWriteItemError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::batch_write_item::BatchWriteItemError::unhandled)?;
@@ -37,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::batch_write_item::BatchWriteItemError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::batch_write_item::BatchWriteItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -117,8 +121,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::batch_write_item::BatchWriteItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::batch_write_item::BatchWriteItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -151,7 +158,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::batch_write_item::BatchWriteItemOutput, super::super::operation::batch_write_item::BatchWriteItemError> {
+) -> std::result::Result<
+    super::super::operation::batch_write_item::BatchWriteItemOutput,
+    super::super::operation::batch_write_item::BatchWriteItemError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::batch_write_item::builders::BatchWriteItemOutputBuilder::default();
@@ -187,30 +197,28 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "UnprocessedItems" => {
-                        builder = builder.set_unprocessed_items(
-                            super::super::protocol_serde::shape_batch_write_item_request_map::de_batch_write_item_request_map(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    "ItemCollectionMetrics" => {
-                        builder = builder.set_item_collection_metrics(
-                            super::super::protocol_serde::shape_item_collection_metrics_per_table::de_item_collection_metrics_per_table(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?,
-                        );
-                    }
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(
-                            super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "UnprocessedItems" => {
+                    builder = builder.set_unprocessed_items(
+                        super::super::protocol_serde::shape_batch_write_item_request_map::de_batch_write_item_request_map(tokens, _value, depth + 1)?,
+                    );
+                }
+                "ItemCollectionMetrics" => {
+                    builder = builder.set_item_collection_metrics(
+                        super::super::protocol_serde::shape_item_collection_metrics_per_table::de_item_collection_metrics_per_table(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
+                }
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(
+                        super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
+                    );
                 }
-            }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_cancellation_reason.rs`

```diff
--- reference/src/protocol_serde/shape_cancellation_reason.rs
+++ generated/src/protocol_serde/shape_cancellation_reason.rs
@@ -22,7 +22,11 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "Item" => {
-                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
+                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         "Code" => {
                             builder = builder.set_code(
```

### `src/protocol_serde/shape_conditional_check_failed_exception.rs`

```diff
--- reference/src/protocol_serde/shape_conditional_check_failed_exception.rs
+++ generated/src/protocol_serde/shape_conditional_check_failed_exception.rs
@@ -23,7 +23,11 @@
                     );
                 }
                 "Item" => {
-                    builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
+                    builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_consumed_capacity.rs`

```diff
--- reference/src/protocol_serde/shape_consumed_capacity.rs
+++ generated/src/protocol_serde/shape_consumed_capacity.rs
@@ -66,7 +66,11 @@
                         }
                         "VectorIndexes" => {
                             builder = builder.set_vector_indexes(
-                                super::super::protocol_serde::shape_vector_indexes_capacity_map::de_vector_indexes_capacity_map(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_vector_indexes_capacity_map::de_vector_indexes_capacity_map(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_contributor_insights_summaries.rs`

```diff
--- reference/src/protocol_serde/shape_contributor_insights_summaries.rs
+++ generated/src/protocol_serde/shape_contributor_insights_summaries.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::ContributorInsightsSummary>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::ContributorInsightsSummary>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -23,8 +26,11 @@
                         break;
                     }
                     _ => {
-                        let value =
-                            super::super::protocol_serde::shape_contributor_insights_summary::de_contributor_insights_summary(tokens, _value, depth + 1)?;
+                        let value = super::super::protocol_serde::shape_contributor_insights_summary::de_contributor_insights_summary(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_create_backup.rs`

```diff
--- reference/src/protocol_serde/shape_create_backup.rs
+++ generated/src/protocol_serde/shape_create_backup.rs
@@ -32,24 +32,22 @@
             }
             tmp
         }),
-        "ContinuousBackupsUnavailableException" => super::super::operation::create_backup::CreateBackupError::ContinuousBackupsUnavailableException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "ContinuousBackupsUnavailableException" => {
+            super::super::operation::create_backup::CreateBackupError::ContinuousBackupsUnavailableException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::ContinuousBackupsUnavailableExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_continuous_backups_unavailable_exception::de_continuous_backups_unavailable_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::create_backup::CreateBackupError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::ContinuousBackupsUnavailableExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_continuous_backups_unavailable_exception::de_continuous_backups_unavailable_exception_json_err(_response_body, output).map_err(super::super::operation::create_backup::CreateBackupError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "InternalServerError" => super::super::operation::create_backup::CreateBackupError::InternalServerError({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -70,8 +68,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_backup::CreateBackupError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::create_backup::CreateBackupError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -172,7 +171,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "BackupDetails" => {
-                    builder = builder.set_backup_details(super::super::protocol_serde::shape_backup_details::de_backup_details(tokens, _value, depth + 1)?);
+                    builder = builder.set_backup_details(super::super::protocol_serde::shape_backup_details::de_backup_details(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_create_global_table.rs`

```diff
--- reference/src/protocol_serde/shape_create_global_table.rs
+++ generated/src/protocol_serde/shape_create_global_table.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::create_global_table::CreateGlobalTableOutput, super::super::operation::create_global_table::CreateGlobalTableError>
-{
+) -> std::result::Result<
+    super::super::operation::create_global_table::CreateGlobalTableOutput,
+    super::super::operation::create_global_table::CreateGlobalTableError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::create_global_table::CreateGlobalTableError::unhandled)?;
@@ -18,24 +20,27 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "GlobalTableAlreadyExistsException" => super::super::operation::create_global_table::CreateGlobalTableError::GlobalTableAlreadyExistsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "GlobalTableAlreadyExistsException" => {
+            super::super::operation::create_global_table::CreateGlobalTableError::GlobalTableAlreadyExistsException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::GlobalTableAlreadyExistsExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_global_table_already_exists_exception::de_global_table_already_exists_exception_json_err(
-                    _response_body,
-                    output,
-                )
-                .map_err(super::super::operation::create_global_table::CreateGlobalTableError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::GlobalTableAlreadyExistsExceptionBuilder::default();
+                    output =
+                        super::super::protocol_serde::shape_global_table_already_exists_exception::de_global_table_already_exists_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(super::super::operation::create_global_table::CreateGlobalTableError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "InternalServerError" => super::super::operation::create_global_table::CreateGlobalTableError::InternalServerError({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -56,8 +61,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_global_table::CreateGlobalTableError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::create_global_table::CreateGlobalTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -105,8 +111,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::create_global_table::CreateGlobalTableOutput, super::super::operation::create_global_table::CreateGlobalTableError>
-{
+) -> std::result::Result<
+    super::super::operation::create_global_table::CreateGlobalTableOutput,
+    super::super::operation::create_global_table::CreateGlobalTableError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::create_global_table::builders::CreateGlobalTableOutputBuilder::default();
```

### `src/protocol_serde/shape_create_table.rs`

```diff
--- reference/src/protocol_serde/shape_create_table.rs
+++ generated/src/protocol_serde/shape_create_table.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::create_table::CreateTableError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::create_table::CreateTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -110,8 +111,10 @@
 pub(crate) fn de_create_table(
     _value: &[u8],
     mut builder: super::super::operation::create_table::builders::CreateTableOutputBuilder,
-) -> ::std::result::Result<super::super::operation::create_table::builders::CreateTableOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::create_table::builders::CreateTableOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_delete_item.rs`

```diff
--- reference/src/protocol_serde/shape_delete_item.rs
+++ generated/src/protocol_serde/shape_delete_item.rs
@@ -55,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,27 +66,13 @@
             }
             tmp
         }),
-        "ItemCollectionSizeLimitExceededException" => super::super::operation::delete_item::DeleteItemError::ItemCollectionSizeLimitExceededException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::ItemCollectionSizeLimitExceededExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_item_collection_size_limit_exceeded_exception::de_item_collection_size_limit_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ProvisionedThroughputExceededException" => {
-            super::super::operation::delete_item::DeleteItemError::ProvisionedThroughputExceededException({
+        "ItemCollectionSizeLimitExceededException" => {
+            super::super::operation::delete_item::DeleteItemError::ItemCollectionSizeLimitExceededException({
                 #[allow(unused_mut)]
                 let mut tmp = {
                     #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
+                    let mut output = super::super::types::error::builders::ItemCollectionSizeLimitExceededExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_item_collection_size_limit_exceeded_exception::de_item_collection_size_limit_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -95,6 +82,20 @@
                 tmp
             })
         }
+        "ProvisionedThroughputExceededException" => super::super::operation::delete_item::DeleteItemError::ProvisionedThroughputExceededException({
+            #[allow(unused_mut)]
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "ReplicatedWriteConflictException" => super::super::operation::delete_item::DeleteItemError::ReplicatedWriteConflictException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -133,8 +134,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -163,9 +167,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TransactionConflictExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::delete_item::DeleteItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -207,8 +213,10 @@
 pub(crate) fn de_delete_item(
     _value: &[u8],
     mut builder: super::super::operation::delete_item::builders::DeleteItemOutputBuilder,
-) -> ::std::result::Result<super::super::operation::delete_item::builders::DeleteItemOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::delete_item::builders::DeleteItemOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -217,26 +225,28 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "Attributes" => {
-                        builder = builder.set_attributes(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
-                    }
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(super::super::protocol_serde::shape_consumed_capacity::de_consumed_capacity(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
-                    "ItemCollectionMetrics" => {
-                        builder = builder.set_item_collection_metrics(
-                            super::super::protocol_serde::shape_item_collection_metrics::de_item_collection_metrics(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Attributes" => {
+                    builder = builder.set_attributes(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-            }
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(super::super::protocol_serde::shape_consumed_capacity::de_consumed_capacity(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "ItemCollectionMetrics" => {
+                    builder = builder.set_item_collection_metrics(
+                        super::super::protocol_serde::shape_item_collection_metrics::de_item_collection_metrics(tokens, _value, depth + 1)?,
+                    );
+                }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_delete_resource_policy.rs`

```diff
--- reference/src/protocol_serde/shape_delete_resource_policy.rs
+++ generated/src/protocol_serde/shape_delete_resource_policy.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -40,8 +44,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -70,8 +75,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::PolicyNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_policy_not_found_exception::de_policy_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_policy_not_found_exception::de_policy_not_found_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -100,8 +106,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_delete_table.rs`

```diff
--- reference/src/protocol_serde/shape_delete_table.rs
+++ generated/src/protocol_serde/shape_delete_table.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_table::DeleteTableError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::delete_table::DeleteTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,8 +83,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::delete_table::DeleteTableError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::delete_table::DeleteTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -125,8 +129,10 @@
 pub(crate) fn de_delete_table(
     _value: &[u8],
     mut builder: super::super::operation::delete_table::builders::DeleteTableOutputBuilder,
-) -> ::std::result::Result<super::super::operation::delete_table::builders::DeleteTableOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::delete_table::builders::DeleteTableOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_describe_continuous_backups.rs`

```diff
--- reference/src/protocol_serde/shape_describe_continuous_backups.rs
+++ generated/src/protocol_serde/shape_describe_continuous_backups.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -39,21 +35,26 @@
             }
             tmp
         }),
-        "InvalidEndpointException" => super::super::operation::describe_continuous_backups::DescribeContinuousBackupsError::InvalidEndpointException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidEndpointException" => {
+            super::super::operation::describe_continuous_backups::DescribeContinuousBackupsError::InvalidEndpointException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::describe_continuous_backups::DescribeContinuousBackupsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "TableNotFoundException" => super::super::operation::describe_continuous_backups::DescribeContinuousBackupsError::TableNotFoundException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -120,7 +121,11 @@
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "ContinuousBackupsDescription" => {
                     builder = builder.set_continuous_backups_description(
-                        super::super::protocol_serde::shape_continuous_backups_description::de_continuous_backups_description(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_continuous_backups_description::de_continuous_backups_description(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_describe_contributor_insights.rs`

```diff
--- reference/src/protocol_serde/shape_describe_contributor_insights.rs
+++ generated/src/protocol_serde/shape_describe_contributor_insights.rs
@@ -41,9 +41,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::describe_contributor_insights::DescribeContributorInsightsError::unhandled)?;
+                    output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::describe_contributor_insights::DescribeContributorInsightsError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -118,7 +120,11 @@
                 }
                 "ContributorInsightsRuleList" => {
                     builder = builder.set_contributor_insights_rule_list(
-                        super::super::protocol_serde::shape_contributor_insights_rule_list::de_contributor_insights_rule_list(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_contributor_insights_rule_list::de_contributor_insights_rule_list(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 "ContributorInsightsStatus" => {
```

### `src/protocol_serde/shape_describe_endpoints.rs`

```diff
--- reference/src/protocol_serde/shape_describe_endpoints.rs
+++ generated/src/protocol_serde/shape_describe_endpoints.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::describe_endpoints::DescribeEndpointsOutput, super::super::operation::describe_endpoints::DescribeEndpointsError>
-{
+) -> std::result::Result<
+    super::super::operation::describe_endpoints::DescribeEndpointsOutput,
+    super::super::operation::describe_endpoints::DescribeEndpointsError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::describe_endpoints::DescribeEndpointsError::unhandled)?;
@@ -19,8 +21,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::describe_endpoints::DescribeEndpointsOutput, super::super::operation::describe_endpoints::DescribeEndpointsError>
-{
+) -> std::result::Result<
+    super::super::operation::describe_endpoints::DescribeEndpointsOutput,
+    super::super::operation::describe_endpoints::DescribeEndpointsError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::describe_endpoints::builders::DescribeEndpointsOutputBuilder::default();
```

### `src/protocol_serde/shape_describe_global_table.rs`

```diff
--- reference/src/protocol_serde/shape_describe_global_table.rs
+++ generated/src/protocol_serde/shape_describe_global_table.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::describe_global_table::DescribeGlobalTableError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::describe_global_table::DescribeGlobalTableError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -58,8 +62,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::describe_global_table::DescribeGlobalTableError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::describe_global_table::DescribeGlobalTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_describe_global_table_settings.rs`

```diff
--- reference/src/protocol_serde/shape_describe_global_table_settings.rs
+++ generated/src/protocol_serde/shape_describe_global_table_settings.rs
@@ -55,21 +55,26 @@
             }
             tmp
         }),
-        "InvalidEndpointException" => super::super::operation::describe_global_table_settings::DescribeGlobalTableSettingsError::InvalidEndpointException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidEndpointException" => {
+            super::super::operation::describe_global_table_settings::DescribeGlobalTableSettingsError::InvalidEndpointException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::describe_global_table_settings::DescribeGlobalTableSettingsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => super::super::operation::describe_global_table_settings::DescribeGlobalTableSettingsError::generic(generic),
     })
 }
```

### `src/protocol_serde/shape_describe_kinesis_streaming_destination.rs`

```diff
--- reference/src/protocol_serde/shape_describe_kinesis_streaming_destination.rs
+++ generated/src/protocol_serde/shape_describe_kinesis_streaming_destination.rs
@@ -15,7 +15,9 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled(generic))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -27,7 +29,9 @@
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InternalServerErrorBuilder::default();
                     output = super::super::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
-                        .map_err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled)?;
+                        .map_err(
+                            super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled,
+                        )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -43,8 +47,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled)?;
+                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -60,9 +67,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled)?;
+                    output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -89,9 +98,11 @@
         #[allow(unused_mut)]
         let mut output =
             super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationOutputBuilder::default();
-        output =
-            super::super::protocol_serde::shape_describe_kinesis_streaming_destination::de_describe_kinesis_streaming_destination(_response_body, output)
-                .map_err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled)?;
+        output = super::super::protocol_serde::shape_describe_kinesis_streaming_destination::de_describe_kinesis_streaming_destination(
+            _response_body,
+            output,
+        )
+        .map_err(super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_describe_table.rs`

```diff
--- reference/src/protocol_serde/shape_describe_table.rs
+++ generated/src/protocol_serde/shape_describe_table.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::describe_table::DescribeTableError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::describe_table::DescribeTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -52,8 +53,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::describe_table::DescribeTableError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::describe_table::DescribeTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_describe_table_replica_auto_scaling.rs`

```diff
--- reference/src/protocol_serde/shape_describe_table_replica_auto_scaling.rs
+++ generated/src/protocol_serde/shape_describe_table_replica_auto_scaling.rs
@@ -20,21 +20,23 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InternalServerError" => super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::InternalServerError({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InternalServerError" => {
+            super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::InternalServerError({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::InternalServerErrorBuilder::default();
-                output = super::super::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
-                    .map_err(super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::InternalServerErrorBuilder::default();
+                    output = super::super::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
+                        .map_err(super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "ResourceNotFoundException" => {
             super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::ResourceNotFoundException({
                 #[allow(unused_mut)]
@@ -41,9 +43,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::unhandled)?;
+                    output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -68,9 +72,11 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = super::super::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingOutputBuilder::default();
-        output = super::super::protocol_serde::shape_describe_table_replica_auto_scaling::de_describe_table_replica_auto_scaling(_response_body, output)
-            .map_err(super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::unhandled)?;
+        let mut output =
+            super::super::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingOutputBuilder::default();
+        output =
+            super::super::protocol_serde::shape_describe_table_replica_auto_scaling::de_describe_table_replica_auto_scaling(_response_body, output)
+                .map_err(super::super::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
@@ -81,7 +87,10 @@
 ) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    super::super::protocol_serde::shape_describe_table_replica_auto_scaling_input::ser_describe_table_replica_auto_scaling_input_input(&mut object, input)?;
+    super::super::protocol_serde::shape_describe_table_replica_auto_scaling_input::ser_describe_table_replica_auto_scaling_input_input(
+        &mut object,
+        input,
+    )?;
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
@@ -104,7 +113,11 @@
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "TableAutoScalingDescription" => {
                     builder = builder.set_table_auto_scaling_description(
-                        super::super::protocol_serde::shape_table_auto_scaling_description::de_table_auto_scaling_description(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_table_auto_scaling_description::de_table_auto_scaling_description(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_describe_time_to_live.rs`

```diff
--- reference/src/protocol_serde/shape_describe_time_to_live.rs
+++ generated/src/protocol_serde/shape_describe_time_to_live.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::describe_time_to_live::DescribeTimeToLiveError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::describe_time_to_live::DescribeTimeToLiveError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -40,8 +44,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::describe_time_to_live::DescribeTimeToLiveError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::describe_time_to_live::DescribeTimeToLiveError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -55,8 +60,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::describe_time_to_live::DescribeTimeToLiveError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::describe_time_to_live::DescribeTimeToLiveError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_disable_kinesis_streaming_destination.rs`

```diff
--- reference/src/protocol_serde/shape_disable_kinesis_streaming_destination.rs
+++ generated/src/protocol_serde/shape_disable_kinesis_streaming_destination.rs
@@ -15,7 +15,9 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled(generic))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -27,7 +29,9 @@
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InternalServerErrorBuilder::default();
                     output = super::super::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
-                        .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
+                        .map_err(
+                            super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled,
+                        )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -43,8 +47,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
+                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -60,8 +67,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::LimitExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
+                            .map_err(
+                                super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled,
+                            )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -77,8 +87,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceInUseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
+                            .map_err(
+                                super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled,
+                            )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -94,9 +107,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
+                    output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -123,8 +138,11 @@
         #[allow(unused_mut)]
         let mut output =
             super::super::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationOutputBuilder::default();
-        output = super::super::protocol_serde::shape_disable_kinesis_streaming_destination::de_disable_kinesis_streaming_destination(_response_body, output)
-            .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
+        output = super::super::protocol_serde::shape_disable_kinesis_streaming_destination::de_disable_kinesis_streaming_destination(
+            _response_body,
+            output,
+        )
+        .map_err(super::super::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_enable_kinesis_streaming_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_enable_kinesis_streaming_configuration.rs
+++ generated/src/protocol_serde/shape_enable_kinesis_streaming_configuration.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_enable_kinesis_streaming_configuration(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::EnableKinesisStreamingConfiguration,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.approximate_creation_date_time_precision {
+        object.key("ApproximateCreationDateTimePrecision").string(var_1.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_enable_kinesis_streaming_configuration<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -47,13 +57,3 @@
         )),
     }
 }
-
-pub fn ser_enable_kinesis_streaming_configuration(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::EnableKinesisStreamingConfiguration,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.approximate_creation_date_time_precision {
-        object.key("ApproximateCreationDateTimePrecision").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_enable_kinesis_streaming_destination.rs`

```diff
--- reference/src/protocol_serde/shape_enable_kinesis_streaming_destination.rs
+++ generated/src/protocol_serde/shape_enable_kinesis_streaming_destination.rs
@@ -15,7 +15,9 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled(generic))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -43,8 +45,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled)?;
+                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -60,8 +65,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::LimitExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
+                            .map_err(
+                                super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled,
+                            )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -77,8 +85,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceInUseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
+                            .map_err(
+                                super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled,
+                            )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -94,9 +105,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled)?;
+                    output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -121,9 +134,11 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder::default();
-        output = super::super::protocol_serde::shape_enable_kinesis_streaming_destination::de_enable_kinesis_streaming_destination(_response_body, output)
-            .map_err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled)?;
+        let mut output =
+            super::super::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder::default();
+        output =
+            super::super::protocol_serde::shape_enable_kinesis_streaming_destination::de_enable_kinesis_streaming_destination(_response_body, output)
+                .map_err(super::super::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_endpoint.rs`

```diff
--- reference/src/protocol_serde/shape_endpoint.rs
+++ generated/src/protocol_serde/shape_endpoint.rs
@@ -44,9 +44,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::endpoint_correct_errors(builder).build().map_err(|err| {
-                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
-            })?))
+            Ok(Some(super::super::serde_util::endpoint_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_execute_statement.rs`

```diff
--- reference/src/protocol_serde/shape_execute_statement.rs
+++ generated/src/protocol_serde/shape_execute_statement.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::execute_statement::ExecuteStatementOutput, super::super::operation::execute_statement::ExecuteStatementError> {
+) -> std::result::Result<
+    super::super::operation::execute_statement::ExecuteStatementOutput,
+    super::super::operation::execute_statement::ExecuteStatementError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::execute_statement::ExecuteStatementError::unhandled)?;
@@ -117,8 +120,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::execute_statement::ExecuteStatementError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::execute_statement::ExecuteStatementError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -147,9 +153,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TransactionConflictExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::execute_statement::ExecuteStatementError::unhandled)?;
+                output = super::super::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::execute_statement::ExecuteStatementError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -167,7 +175,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::execute_statement::ExecuteStatementOutput, super::super::operation::execute_statement::ExecuteStatementError> {
+) -> std::result::Result<
+    super::super::operation::execute_statement::ExecuteStatementOutput,
+    super::super::operation::execute_statement::ExecuteStatementError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::execute_statement::builders::ExecuteStatementOutputBuilder::default();
```

### `src/protocol_serde/shape_execute_transaction.rs`

```diff
--- reference/src/protocol_serde/shape_execute_transaction.rs
+++ generated/src/protocol_serde/shape_execute_transaction.rs
@@ -26,12 +26,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::IdempotentParameterMismatchExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_idempotent_parameter_mismatch_exception::de_idempotent_parameter_mismatch_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::execute_transaction::ExecuteTransactionError::unhandled)?;
+                    output = super::super::protocol_serde::shape_idempotent_parameter_mismatch_exception::de_idempotent_parameter_mismatch_exception_json_err(_response_body, output).map_err(super::super::operation::execute_transaction::ExecuteTransactionError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -92,8 +87,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::execute_transaction::ExecuteTransactionError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::execute_transaction::ExecuteTransactionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -122,9 +120,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TransactionCanceledExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_transaction_canceled_exception::de_transaction_canceled_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::execute_transaction::ExecuteTransactionError::unhandled)?;
+                output = super::super::protocol_serde::shape_transaction_canceled_exception::de_transaction_canceled_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::execute_transaction::ExecuteTransactionError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -199,23 +199,21 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "Responses" => {
-                        builder = builder.set_responses(super::super::protocol_serde::shape_item_response_list::de_item_response_list(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(
-                            super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Responses" => {
+                    builder = builder.set_responses(super::super::protocol_serde::shape_item_response_list::de_item_response_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(
+                        super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
+                    );
                 }
-            }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_export_conflict_exception.rs`

```diff
--- reference/src/protocol_serde/shape_export_conflict_exception.rs
+++ generated/src/protocol_serde/shape_export_conflict_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_export_conflict_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ExportConflictExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ExportConflictExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::ExportConflictExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_export_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_export_not_found_exception.rs
+++ generated/src/protocol_serde/shape_export_not_found_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_export_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ExportNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ExportNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::ExportNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_export_table_to_point_in_time.rs`

```diff
--- reference/src/protocol_serde/shape_export_table_to_point_in_time.rs
+++ generated/src/protocol_serde/shape_export_table_to_point_in_time.rs
@@ -15,30 +15,29 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "ExportConflictException" => super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::ExportConflictException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "ExportConflictException" => {
+            super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::ExportConflictException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::ExportConflictExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_export_conflict_exception::de_export_conflict_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::ExportConflictExceptionBuilder::default();
+                    output =
+                        super::super::protocol_serde::shape_export_conflict_exception::de_export_conflict_exception_json_err(_response_body, output)
+                            .map_err(super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "InternalServerError" => super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::InternalServerError({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -54,22 +53,26 @@
             }
             tmp
         }),
-        "InvalidExportTimeException" => super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::InvalidExportTimeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidExportTimeException" => {
+            super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::InvalidExportTimeException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::InvalidExportTimeExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_invalid_export_time_exception::de_invalid_export_time_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::InvalidExportTimeExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_invalid_export_time_exception::de_invalid_export_time_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "LimitExceededException" => super::super::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::LimitExceededException({
             #[allow(unused_mut)]
             let mut tmp = {
```

### `src/protocol_serde/shape_get_item.rs`

```diff
--- reference/src/protocol_serde/shape_get_item.rs
+++ generated/src/protocol_serde/shape_get_item.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_item::GetItemError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::get_item::GetItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -47,22 +48,20 @@
             }
             tmp
         }),
-        "ProvisionedThroughputExceededException" => {
-            super::super::operation::get_item::GetItemError::ProvisionedThroughputExceededException({
+        "ProvisionedThroughputExceededException" => super::super::operation::get_item::GetItemError::ProvisionedThroughputExceededException({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::get_item::GetItemError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::get_item::GetItemError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "RequestLimitExceeded" => super::super::operation::get_item::GetItemError::RequestLimitExceeded({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -83,8 +82,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_item::GetItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::get_item::GetItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -141,7 +143,8 @@
 pub(crate) fn de_get_item(
     _value: &[u8],
     mut builder: super::super::operation::get_item::builders::GetItemOutputBuilder,
-) -> ::std::result::Result<super::super::operation::get_item::builders::GetItemOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<super::super::operation::get_item::builders::GetItemOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -152,7 +155,11 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "Item" => {
-                    builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
+                    builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 "ConsumedCapacity" => {
                     builder = builder.set_consumed_capacity(super::super::protocol_serde::shape_consumed_capacity::de_consumed_capacity(
```

### `src/protocol_serde/shape_get_resource_policy.rs`

```diff
--- reference/src/protocol_serde/shape_get_resource_policy.rs
+++ generated/src/protocol_serde/shape_get_resource_policy.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::get_resource_policy::GetResourcePolicyOutput, super::super::operation::get_resource_policy::GetResourcePolicyError>
-{
+) -> std::result::Result<
+    super::super::operation::get_resource_policy::GetResourcePolicyOutput,
+    super::super::operation::get_resource_policy::GetResourcePolicyError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::get_resource_policy::GetResourcePolicyError::unhandled)?;
@@ -38,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_resource_policy::GetResourcePolicyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::get_resource_policy::GetResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -53,8 +56,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::PolicyNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_policy_not_found_exception::de_policy_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_resource_policy::GetResourcePolicyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_policy_not_found_exception::de_policy_not_found_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::get_resource_policy::GetResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -68,8 +72,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::get_resource_policy::GetResourcePolicyError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::get_resource_policy::GetResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -87,8 +94,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::get_resource_policy::GetResourcePolicyOutput, super::super::operation::get_resource_policy::GetResourcePolicyError>
-{
+) -> std::result::Result<
+    super::super::operation::get_resource_policy::GetResourcePolicyOutput,
+    super::super::operation::get_resource_policy::GetResourcePolicyError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::get_resource_policy::builders::GetResourcePolicyOutputBuilder::default();
```

### `src/protocol_serde/shape_global_secondary_index.rs`

```diff
--- reference/src/protocol_serde/shape_global_secondary_index.rs
+++ generated/src/protocol_serde/shape_global_secondary_index.rs
@@ -66,41 +66,41 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        match key.to_unescaped()?.as_ref() {
-                            "IndexName" => {
-                                builder = builder.set_index_name(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "KeySchema" => {
-                                builder = builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
-                            }
-                            "Projection" => {
-                                builder = builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
-                            }
-                            "ProvisionedThroughput" => {
-                                builder = builder.set_provisioned_throughput(
-                                    super::super::protocol_serde::shape_provisioned_throughput::de_provisioned_throughput(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "OnDemandThroughput" => {
-                                builder = builder.set_on_demand_throughput(
-                                    super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "WarmThroughput" => {
-                                builder = builder.set_warm_throughput(super::super::protocol_serde::shape_warm_throughput::de_warm_throughput(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?);
-                            }
-                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "IndexName" => {
+                            builder = builder.set_index_name(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "KeySchema" => {
+                            builder =
+                                builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
+                        }
+                        "Projection" => {
+                            builder =
+                                builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
+                        }
+                        "ProvisionedThroughput" => {
+                            builder = builder.set_provisioned_throughput(
+                                super::super::protocol_serde::shape_provisioned_throughput::de_provisioned_throughput(tokens, _value, depth + 1)?,
+                            );
+                        }
+                        "OnDemandThroughput" => {
+                            builder = builder.set_on_demand_throughput(
+                                super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
+                            );
+                        }
+                        "WarmThroughput" => {
+                            builder = builder.set_warm_throughput(super::super::protocol_serde::shape_warm_throughput::de_warm_throughput(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
-                    }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
@@ -108,9 +108,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::global_secondary_index_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::global_secondary_index_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_global_secondary_index_description.rs`

```diff
--- reference/src/protocol_serde/shape_global_secondary_index_description.rs
+++ generated/src/protocol_serde/shape_global_secondary_index_description.rs
@@ -20,74 +20,72 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        match key.to_unescaped()?.as_ref() {
-                            "IndexName" => {
-                                builder = builder.set_index_name(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "KeySchema" => {
-                                builder = builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
-                            }
-                            "Projection" => {
-                                builder = builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
-                            }
-                            "IndexStatus" => {
-                                builder = builder.set_index_status(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| super::super::types::IndexStatus::from(u.as_ref())))
-                                        .transpose()?,
-                                );
-                            }
-                            "Backfilling" => {
-                                builder = builder.set_backfilling(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                            }
-                            "ProvisionedThroughput" => {
-                                builder = builder.set_provisioned_throughput(
-                                    super::super::protocol_serde::shape_provisioned_throughput_description::de_provisioned_throughput_description(
-                                        tokens,
-                                        _value,
-                                        depth + 1,
-                                    )?,
-                                );
-                            }
-                            "IndexSizeBytes" => {
-                                builder = builder.set_index_size_bytes(
-                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                        .map(i64::try_from)
-                                        .transpose()?,
-                                );
-                            }
-                            "ItemCount" => {
-                                builder = builder.set_item_count(
-                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                        .map(i64::try_from)
-                                        .transpose()?,
-                                );
-                            }
-                            "IndexArn" => {
-                                builder = builder.set_index_arn(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "OnDemandThroughput" => {
-                                builder = builder.set_on_demand_throughput(
-                                    super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "WarmThroughput" => {
-                                builder = builder.set_warm_throughput(
-                                    super::super::protocol_serde::shape_global_secondary_index_warm_throughput_description::de_global_secondary_index_warm_throughput_description(tokens, _value, depth + 1)?
-                                );
-                            }
-                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "IndexName" => {
+                            builder = builder.set_index_name(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "KeySchema" => {
+                            builder =
+                                builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
+                        }
+                        "Projection" => {
+                            builder =
+                                builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
+                        }
+                        "IndexStatus" => {
+                            builder = builder.set_index_status(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| super::super::types::IndexStatus::from(u.as_ref())))
+                                    .transpose()?,
+                            );
+                        }
+                        "Backfilling" => {
+                            builder = builder.set_backfilling(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                        }
+                        "ProvisionedThroughput" => {
+                            builder = builder.set_provisioned_throughput(
+                                super::super::protocol_serde::shape_provisioned_throughput_description::de_provisioned_throughput_description(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
+                            );
+                        }
+                        "IndexSizeBytes" => {
+                            builder = builder.set_index_size_bytes(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i64::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "ItemCount" => {
+                            builder = builder.set_item_count(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i64::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "IndexArn" => {
+                            builder = builder.set_index_arn(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "OnDemandThroughput" => {
+                            builder = builder.set_on_demand_throughput(
+                                super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
+                            );
+                        }
+                        "WarmThroughput" => {
+                            builder = builder.set_warm_throughput(super::super::protocol_serde::shape_global_secondary_index_warm_throughput_description::de_global_secondary_index_warm_throughput_description(tokens, _value, depth + 1)?);
                         }
-                    }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_global_secondary_index_info.rs`

```diff
--- reference/src/protocol_serde/shape_global_secondary_index_info.rs
+++ generated/src/protocol_serde/shape_global_secondary_index_info.rs
@@ -20,34 +20,34 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        match key.to_unescaped()?.as_ref() {
-                            "IndexName" => {
-                                builder = builder.set_index_name(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "KeySchema" => {
-                                builder = builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
-                            }
-                            "Projection" => {
-                                builder = builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
-                            }
-                            "ProvisionedThroughput" => {
-                                builder = builder.set_provisioned_throughput(
-                                    super::super::protocol_serde::shape_provisioned_throughput::de_provisioned_throughput(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "OnDemandThroughput" => {
-                                builder = builder.set_on_demand_throughput(
-                                    super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "IndexName" => {
+                            builder = builder.set_index_name(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "KeySchema" => {
+                            builder =
+                                builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
+                        }
+                        "Projection" => {
+                            builder =
+                                builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
+                        }
+                        "ProvisionedThroughput" => {
+                            builder = builder.set_provisioned_throughput(
+                                super::super::protocol_serde::shape_provisioned_throughput::de_provisioned_throughput(tokens, _value, depth + 1)?,
+                            );
+                        }
+                        "OnDemandThroughput" => {
+                            builder = builder.set_on_demand_throughput(
+                                super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
+                            );
                         }
-                    }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_global_secondary_index_warm_throughput_description.rs`

```diff
--- reference/src/protocol_serde/shape_global_secondary_index_warm_throughput_description.rs
+++ generated/src/protocol_serde/shape_global_secondary_index_warm_throughput_description.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<super::super::types::GlobalSecondaryIndexWarmThroughputDescription>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<super::super::types::GlobalSecondaryIndexWarmThroughputDescription>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
```

### `src/protocol_serde/shape_global_secondary_indexes.rs`

```diff
--- reference/src/protocol_serde/shape_global_secondary_indexes.rs
+++ generated/src/protocol_serde/shape_global_secondary_indexes.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::GlobalSecondaryIndexInfo>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::GlobalSecondaryIndexInfo>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -23,8 +26,11 @@
                         break;
                     }
                     _ => {
-                        let value =
-                            super::super::protocol_serde::shape_global_secondary_index_info::de_global_secondary_index_info(tokens, _value, depth + 1)?;
+                        let value = super::super::protocol_serde::shape_global_secondary_index_info::de_global_secondary_index_info(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_global_table.rs`

```diff
--- reference/src/protocol_serde/shape_global_table.rs
+++ generated/src/protocol_serde/shape_global_table.rs
@@ -29,8 +29,11 @@
                             );
                         }
                         "ReplicationGroup" => {
-                            builder =
-                                builder.set_replication_group(super::super::protocol_serde::shape_replica_list::de_replica_list(tokens, _value, depth + 1)?);
+                            builder = builder.set_replication_group(super::super::protocol_serde::shape_replica_list::de_replica_list(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
```

### `src/protocol_serde/shape_global_table_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_global_table_not_found_exception.rs
+++ generated/src/protocol_serde/shape_global_table_not_found_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_global_table_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::GlobalTableNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::GlobalTableNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::GlobalTableNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_import_conflict_exception.rs`

```diff
--- reference/src/protocol_serde/shape_import_conflict_exception.rs
+++ generated/src/protocol_serde/shape_import_conflict_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_import_conflict_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ImportConflictExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ImportConflictExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::ImportConflictExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_import_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_import_not_found_exception.rs
+++ generated/src/protocol_serde/shape_import_not_found_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_import_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ImportNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ImportNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::ImportNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_import_table.rs`

```diff
--- reference/src/protocol_serde/shape_import_table.rs
+++ generated/src/protocol_serde/shape_import_table.rs
@@ -95,8 +95,10 @@
 pub(crate) fn de_import_table(
     _value: &[u8],
     mut builder: super::super::operation::import_table::builders::ImportTableOutputBuilder,
-) -> ::std::result::Result<super::super::operation::import_table::builders::ImportTableOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::import_table::builders::ImportTableOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_import_table_description.rs`

```diff
--- reference/src/protocol_serde/shape_import_table_description.rs
+++ generated/src/protocol_serde/shape_import_table_description.rs
@@ -20,138 +20,140 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        match key.to_unescaped()?.as_ref() {
-                            "ImportArn" => {
-                                builder = builder.set_import_arn(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "ImportStatus" => {
-                                builder = builder.set_import_status(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| super::super::types::ImportStatus::from(u.as_ref())))
-                                        .transpose()?,
-                                );
-                            }
-                            "TableArn" => {
-                                builder = builder.set_table_arn(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "TableId" => {
-                                builder = builder.set_table_id(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "ClientToken" => {
-                                builder = builder.set_client_token(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "S3BucketSource" => {
-                                builder = builder.set_s3_bucket_source(super::super::protocol_serde::shape_s3_bucket_source::de_s3_bucket_source(
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "ImportArn" => {
+                            builder = builder.set_import_arn(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "ImportStatus" => {
+                            builder = builder.set_import_status(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| super::super::types::ImportStatus::from(u.as_ref())))
+                                    .transpose()?,
+                            );
+                        }
+                        "TableArn" => {
+                            builder = builder.set_table_arn(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "TableId" => {
+                            builder = builder.set_table_id(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "ClientToken" => {
+                            builder = builder.set_client_token(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "S3BucketSource" => {
+                            builder = builder.set_s3_bucket_source(super::super::protocol_serde::shape_s3_bucket_source::de_s3_bucket_source(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
+                        }
+                        "ErrorCount" => {
+                            builder = builder.set_error_count(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i64::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "CloudWatchLogGroupArn" => {
+                            builder = builder.set_cloud_watch_log_group_arn(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "InputFormat" => {
+                            builder = builder.set_input_format(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| super::super::types::InputFormat::from(u.as_ref())))
+                                    .transpose()?,
+                            );
+                        }
+                        "InputFormatOptions" => {
+                            builder = builder.set_input_format_options(
+                                super::super::protocol_serde::shape_input_format_options::de_input_format_options(tokens, _value, depth + 1)?,
+                            );
+                        }
+                        "InputCompressionType" => {
+                            builder = builder.set_input_compression_type(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| super::super::types::InputCompressionType::from(u.as_ref())))
+                                    .transpose()?,
+                            );
+                        }
+                        "TableCreationParameters" => {
+                            builder = builder.set_table_creation_parameters(
+                                super::super::protocol_serde::shape_table_creation_parameters::de_table_creation_parameters(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?);
-                            }
-                            "ErrorCount" => {
-                                builder = builder.set_error_count(
-                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                        .map(i64::try_from)
-                                        .transpose()?,
-                                );
-                            }
-                            "CloudWatchLogGroupArn" => {
-                                builder = builder.set_cloud_watch_log_group_arn(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "InputFormat" => {
-                                builder = builder.set_input_format(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| super::super::types::InputFormat::from(u.as_ref())))
-                                        .transpose()?,
-                                );
-                            }
-                            "InputFormatOptions" => {
-                                builder = builder.set_input_format_options(
-                                    super::super::protocol_serde::shape_input_format_options::de_input_format_options(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "InputCompressionType" => {
-                                builder = builder.set_input_compression_type(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| super::super::types::InputCompressionType::from(u.as_ref())))
-                                        .transpose()?,
-                                );
-                            }
-                            "TableCreationParameters" => {
-                                builder = builder.set_table_creation_parameters(
-                                    super::super::protocol_serde::shape_table_creation_parameters::de_table_creation_parameters(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "StartTime" => {
-                                builder = builder.set_start_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                                    tokens.next(),
-                                    ::aws_smithy_types::date_time::Format::EpochSeconds,
-                                )?);
-                            }
-                            "EndTime" => {
-                                builder = builder.set_end_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                                    tokens.next(),
-                                    ::aws_smithy_types::date_time::Format::EpochSeconds,
-                                )?);
-                            }
-                            "ProcessedSizeBytes" => {
-                                builder = builder.set_processed_size_bytes(
-                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                        .map(i64::try_from)
-                                        .transpose()?,
-                                );
-                            }
-                            "ProcessedItemCount" => {
-                                builder = builder.set_processed_item_count(
-                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                        .map(i64::try_from)
-                                        .transpose()?,
-                                );
-                            }
-                            "ImportedItemCount" => {
-                                builder = builder.set_imported_item_count(
-                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                        .map(i64::try_from)
-                                        .transpose()?,
-                                );
-                            }
-                            "FailureCode" => {
-                                builder = builder.set_failure_code(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "FailureMessage" => {
-                                builder = builder.set_failure_message(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                                )?,
+                            );
+                        }
+                        "StartTime" => {
+                            builder = builder.set_start_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                                tokens.next(),
+                                ::aws_smithy_types::date_time::Format::EpochSeconds,
+                            )?);
+                        }
+                        "EndTime" => {
+                            builder = builder.set_end_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                                tokens.next(),
+                                ::aws_smithy_types::date_time::Format::EpochSeconds,
+                            )?);
+                        }
+                        "ProcessedSizeBytes" => {
+                            builder = builder.set_processed_size_bytes(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i64::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "ProcessedItemCount" => {
+                            builder = builder.set_processed_item_count(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i64::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "ImportedItemCount" => {
+                            builder = builder.set_imported_item_count(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i64::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "FailureCode" => {
+                            builder = builder.set_failure_code(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "FailureMessage" => {
+                            builder = builder.set_failure_message(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
                         }
-                    }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_input_format_options.rs`

```diff
--- reference/src/protocol_serde/shape_input_format_options.rs
+++ generated/src/protocol_serde/shape_input_format_options.rs
@@ -35,7 +35,11 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "Csv" => {
-                            builder = builder.set_csv(super::super::protocol_serde::shape_csv_options::de_csv_options(tokens, _value, depth + 1)?);
+                            builder = builder.set_csv(super::super::protocol_serde::shape_csv_options::de_csv_options(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
```

### `src/protocol_serde/shape_invalid_endpoint_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_endpoint_exception.rs
+++ generated/src/protocol_serde/shape_invalid_endpoint_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_invalid_endpoint_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidEndpointExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidEndpointExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidEndpointExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_export_time_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_export_time_exception.rs
+++ generated/src/protocol_serde/shape_invalid_export_time_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_invalid_export_time_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidExportTimeExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidExportTimeExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidExportTimeExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_invalid_restore_time_exception.rs`

```diff
--- reference/src/protocol_serde/shape_invalid_restore_time_exception.rs
+++ generated/src/protocol_serde/shape_invalid_restore_time_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_invalid_restore_time_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::InvalidRestoreTimeExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InvalidRestoreTimeExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::InvalidRestoreTimeExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_item_response.rs`

```diff
--- reference/src/protocol_serde/shape_item_response.rs
+++ generated/src/protocol_serde/shape_item_response.rs
@@ -22,7 +22,11 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "Item" => {
-                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
+                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
```

### `src/protocol_serde/shape_key_schema_element.rs`

```diff
--- reference/src/protocol_serde/shape_key_schema_element.rs
+++ generated/src/protocol_serde/shape_key_schema_element.rs
@@ -57,9 +57,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::key_schema_element_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::key_schema_element_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_keys_and_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_keys_and_attributes.rs
+++ generated/src/protocol_serde/shape_keys_and_attributes.rs
@@ -71,38 +71,38 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "Keys" => {
-                            builder = builder.set_keys(super::super::protocol_serde::shape_key_list::de_key_list(tokens, _value, depth + 1)?);
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "Keys" => {
+                                builder = builder.set_keys(super::super::protocol_serde::shape_key_list::de_key_list(tokens, _value, depth + 1)?);
+                            }
+                            "AttributesToGet" => {
+                                builder = builder.set_attributes_to_get(
+                                    super::super::protocol_serde::shape_attribute_name_list::de_attribute_name_list(tokens, _value, depth + 1)?,
+                                );
+                            }
+                            "ConsistentRead" => {
+                                builder = builder.set_consistent_read(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                            }
+                            "ProjectionExpression" => {
+                                builder = builder.set_projection_expression(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "ExpressionAttributeNames" => {
+                                builder = builder.set_expression_attribute_names(
+                                    super::super::protocol_serde::shape_expression_attribute_name_map::de_expression_attribute_name_map(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        "AttributesToGet" => {
-                            builder = builder.set_attributes_to_get(super::super::protocol_serde::shape_attribute_name_list::de_attribute_name_list(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "ConsistentRead" => {
-                            builder = builder.set_consistent_read(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                        }
-                        "ProjectionExpression" => {
-                            builder = builder.set_projection_expression(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "ExpressionAttributeNames" => {
-                            builder = builder.set_expression_attribute_names(
-                                super::super::protocol_serde::shape_expression_attribute_name_map::de_expression_attribute_name_map(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
@@ -110,9 +110,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::keys_and_attributes_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::keys_and_attributes_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_kinesis_data_stream_destinations.rs`

```diff
--- reference/src/protocol_serde/shape_kinesis_data_stream_destinations.rs
+++ generated/src/protocol_serde/shape_kinesis_data_stream_destinations.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::KinesisDataStreamDestination>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::KinesisDataStreamDestination>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
```

### `src/protocol_serde/shape_list_backups.rs`

```diff
--- reference/src/protocol_serde/shape_list_backups.rs
+++ generated/src/protocol_serde/shape_list_backups.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_backups::ListBackupsError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::list_backups::ListBackupsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,8 +81,10 @@
 pub(crate) fn de_list_backups(
     _value: &[u8],
     mut builder: super::super::operation::list_backups::builders::ListBackupsOutputBuilder,
-) -> ::std::result::Result<super::super::operation::list_backups::builders::ListBackupsOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::list_backups::builders::ListBackupsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_list_contributor_insights.rs`

```diff
--- reference/src/protocol_serde/shape_list_contributor_insights.rs
+++ generated/src/protocol_serde/shape_list_contributor_insights.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::list_contributor_insights::ListContributorInsightsError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::list_contributor_insights::ListContributorInsightsError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -44,8 +40,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_contributor_insights::ListContributorInsightsError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_contributor_insights::ListContributorInsightsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -105,7 +104,11 @@
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "ContributorInsightsSummaries" => {
                     builder = builder.set_contributor_insights_summaries(
-                        super::super::protocol_serde::shape_contributor_insights_summaries::de_contributor_insights_summaries(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_contributor_insights_summaries::de_contributor_insights_summaries(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 "NextToken" => {
```

### `src/protocol_serde/shape_list_exports.rs`

```diff
--- reference/src/protocol_serde/shape_list_exports.rs
+++ generated/src/protocol_serde/shape_list_exports.rs
@@ -80,8 +80,10 @@
 pub(crate) fn de_list_exports(
     _value: &[u8],
     mut builder: super::super::operation::list_exports::builders::ListExportsOutputBuilder,
-) -> ::std::result::Result<super::super::operation::list_exports::builders::ListExportsOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::list_exports::builders::ListExportsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_list_global_tables.rs`

```diff
--- reference/src/protocol_serde/shape_list_global_tables.rs
+++ generated/src/protocol_serde/shape_list_global_tables.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_global_tables::ListGlobalTablesOutput, super::super::operation::list_global_tables::ListGlobalTablesError> {
+) -> std::result::Result<
+    super::super::operation::list_global_tables::ListGlobalTablesOutput,
+    super::super::operation::list_global_tables::ListGlobalTablesError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::list_global_tables::ListGlobalTablesError::unhandled)?;
@@ -37,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_global_tables::ListGlobalTablesError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::list_global_tables::ListGlobalTablesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -56,7 +60,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::list_global_tables::ListGlobalTablesOutput, super::super::operation::list_global_tables::ListGlobalTablesError> {
+) -> std::result::Result<
+    super::super::operation::list_global_tables::ListGlobalTablesOutput,
+    super::super::operation::list_global_tables::ListGlobalTablesError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::list_global_tables::builders::ListGlobalTablesOutputBuilder::default();
```

### `src/protocol_serde/shape_list_imports.rs`

```diff
--- reference/src/protocol_serde/shape_list_imports.rs
+++ generated/src/protocol_serde/shape_list_imports.rs
@@ -65,8 +65,10 @@
 pub(crate) fn de_list_imports(
     _value: &[u8],
     mut builder: super::super::operation::list_imports::builders::ListImportsOutputBuilder,
-) -> ::std::result::Result<super::super::operation::list_imports::builders::ListImportsOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::list_imports::builders::ListImportsOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_list_tables.rs`

```diff
--- reference/src/protocol_serde/shape_list_tables.rs
+++ generated/src/protocol_serde/shape_list_tables.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_tables::ListTablesError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::list_tables::ListTablesError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,8 +81,10 @@
 pub(crate) fn de_list_tables(
     _value: &[u8],
     mut builder: super::super::operation::list_tables::builders::ListTablesOutputBuilder,
-) -> ::std::result::Result<super::super::operation::list_tables::builders::ListTablesOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::list_tables::builders::ListTablesOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_list_tags_of_resource.rs`

```diff
--- reference/src/protocol_serde/shape_list_tags_of_resource.rs
+++ generated/src/protocol_serde/shape_list_tags_of_resource.rs
@@ -15,7 +15,11 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::list_tags_of_resource::ListTagsOfResourceError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::list_tags_of_resource::ListTagsOfResourceError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -40,8 +44,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_tags_of_resource::ListTagsOfResourceError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::list_tags_of_resource::ListTagsOfResourceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -55,8 +60,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::list_tags_of_resource::ListTagsOfResourceError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::list_tags_of_resource::ListTagsOfResourceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_local_secondary_indexes.rs`

```diff
--- reference/src/protocol_serde/shape_local_secondary_indexes.rs
+++ generated/src/protocol_serde/shape_local_secondary_indexes.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::LocalSecondaryIndexInfo>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::LocalSecondaryIndexInfo>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
```

### `src/protocol_serde/shape_parti_ql_batch_response.rs`

```diff
--- reference/src/protocol_serde/shape_parti_ql_batch_response.rs
+++ generated/src/protocol_serde/shape_parti_ql_batch_response.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::BatchStatementResponse>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::BatchStatementResponse>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -23,7 +26,8 @@
                         break;
                     }
                     _ => {
-                        let value = super::super::protocol_serde::shape_batch_statement_response::de_batch_statement_response(tokens, _value, depth + 1)?;
+                        let value =
+                            super::super::protocol_serde::shape_batch_statement_response::de_batch_statement_response(tokens, _value, depth + 1)?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_policy_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_policy_not_found_exception.rs
+++ generated/src/protocol_serde/shape_policy_not_found_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_policy_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::PolicyNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::PolicyNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::PolicyNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_projection.rs`

```diff
--- reference/src/protocol_serde/shape_projection.rs
+++ generated/src/protocol_serde/shape_projection.rs
@@ -49,7 +49,11 @@
                         }
                         "NonKeyAttributes" => {
                             builder = builder.set_non_key_attributes(
-                                super::super::protocol_serde::shape_non_key_attribute_name_list::de_non_key_attribute_name_list(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_non_key_attribute_name_list::de_non_key_attribute_name_list(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_provisioned_throughput.rs`

```diff
--- reference/src/protocol_serde/shape_provisioned_throughput.rs
+++ generated/src/protocol_serde/shape_provisioned_throughput.rs
@@ -63,9 +63,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::provisioned_throughput_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::provisioned_throughput_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_provisioned_throughput_exceeded_exception.rs`

```diff
--- reference/src/protocol_serde/shape_provisioned_throughput_exceeded_exception.rs
+++ generated/src/protocol_serde/shape_provisioned_throughput_exceeded_exception.rs
@@ -14,23 +14,23 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                .transpose()?,
+                        );
+                    }
+                    "ThrottlingReasons" => {
+                        builder = builder.set_throttling_reasons(
+                            super::super::protocol_serde::shape_throttling_reason_list::de_throttling_reason_list(tokens, _value, depth + 1)?,
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                "ThrottlingReasons" => {
-                    builder = builder.set_throttling_reasons(super::super::protocol_serde::shape_throttling_reason_list::de_throttling_reason_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_put_item.rs`

```diff
--- reference/src/protocol_serde/shape_put_item.rs
+++ generated/src/protocol_serde/shape_put_item.rs
@@ -55,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::put_item::PutItemError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::put_item::PutItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -79,22 +80,20 @@
             }
             tmp
         }),
-        "ProvisionedThroughputExceededException" => {
-            super::super::operation::put_item::PutItemError::ProvisionedThroughputExceededException({
+        "ProvisionedThroughputExceededException" => super::super::operation::put_item::PutItemError::ProvisionedThroughputExceededException({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::put_item::PutItemError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::put_item::PutItemError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "ReplicatedWriteConflictException" => super::super::operation::put_item::PutItemError::ReplicatedWriteConflictException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -133,8 +132,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::put_item::PutItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::put_item::PutItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -163,9 +165,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TransactionConflictExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::put_item::PutItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::put_item::PutItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -207,7 +211,8 @@
 pub(crate) fn de_put_item(
     _value: &[u8],
     mut builder: super::super::operation::put_item::builders::PutItemOutputBuilder,
-) -> ::std::result::Result<super::super::operation::put_item::builders::PutItemOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<super::super::operation::put_item::builders::PutItemOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -216,26 +221,28 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "Attributes" => {
-                        builder = builder.set_attributes(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
-                    }
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(super::super::protocol_serde::shape_consumed_capacity::de_consumed_capacity(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
-                    "ItemCollectionMetrics" => {
-                        builder = builder.set_item_collection_metrics(
-                            super::super::protocol_serde::shape_item_collection_metrics::de_item_collection_metrics(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Attributes" => {
+                    builder = builder.set_attributes(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(super::super::protocol_serde::shape_consumed_capacity::de_consumed_capacity(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-            }
+                "ItemCollectionMetrics" => {
+                    builder = builder.set_item_collection_metrics(
+                        super::super::protocol_serde::shape_item_collection_metrics::de_item_collection_metrics(tokens, _value, depth + 1)?,
+                    );
+                }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_put_request.rs`

```diff
--- reference/src/protocol_serde/shape_put_request.rs
+++ generated/src/protocol_serde/shape_put_request.rs
@@ -59,9 +59,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::put_request_correct_errors(builder).build().map_err(|err| {
-                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
-            })?))
+            Ok(Some(super::super::serde_util::put_request_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_put_resource_policy.rs`

```diff
--- reference/src/protocol_serde/shape_put_resource_policy.rs
+++ generated/src/protocol_serde/shape_put_resource_policy.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::put_resource_policy::PutResourcePolicyOutput, super::super::operation::put_resource_policy::PutResourcePolicyError>
-{
+) -> std::result::Result<
+    super::super::operation::put_resource_policy::PutResourcePolicyOutput,
+    super::super::operation::put_resource_policy::PutResourcePolicyError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::put_resource_policy::PutResourcePolicyError::unhandled)?;
@@ -38,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::put_resource_policy::PutResourcePolicyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::put_resource_policy::PutResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -68,8 +71,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::PolicyNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_policy_not_found_exception::de_policy_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::put_resource_policy::PutResourcePolicyError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_policy_not_found_exception::de_policy_not_found_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::put_resource_policy::PutResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -98,8 +102,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::put_resource_policy::PutResourcePolicyError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::put_resource_policy::PutResourcePolicyError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -117,8 +124,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::put_resource_policy::PutResourcePolicyOutput, super::super::operation::put_resource_policy::PutResourcePolicyError>
-{
+) -> std::result::Result<
+    super::super::operation::put_resource_policy::PutResourcePolicyOutput,
+    super::super::operation::put_resource_policy::PutResourcePolicyError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::put_resource_policy::builders::PutResourcePolicyOutputBuilder::default();
```

### `src/protocol_serde/shape_query.rs`

```diff
--- reference/src/protocol_serde/shape_query.rs
+++ generated/src/protocol_serde/shape_query.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::query::QueryError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::query::QueryError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -47,22 +48,20 @@
             }
             tmp
         }),
-        "ProvisionedThroughputExceededException" => {
-            super::super::operation::query::QueryError::ProvisionedThroughputExceededException({
+        "ProvisionedThroughputExceededException" => super::super::operation::query::QueryError::ProvisionedThroughputExceededException({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::query::QueryError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::query::QueryError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "RequestLimitExceeded" => super::super::operation::query::QueryError::RequestLimitExceeded({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -83,8 +82,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::query::QueryError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::query::QueryError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -121,7 +123,8 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::query::builders::QueryOutputBuilder::default();
-        output = super::super::protocol_serde::shape_query::de_query(_response_body, output).map_err(super::super::operation::query::QueryError::unhandled)?;
+        output = super::super::protocol_serde::shape_query::de_query(_response_body, output)
+            .map_err(super::super::operation::query::QueryError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_replica_already_exists_exception.rs`

```diff
--- reference/src/protocol_serde/shape_replica_already_exists_exception.rs
+++ generated/src/protocol_serde/shape_replica_already_exists_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_replica_already_exists_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ReplicaAlreadyExistsExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ReplicaAlreadyExistsExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::ReplicaAlreadyExistsExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_replica_global_secondary_index_description_list.rs`

```diff
--- reference/src/protocol_serde/shape_replica_global_secondary_index_description_list.rs
+++ generated/src/protocol_serde/shape_replica_global_secondary_index_description_list.rs
@@ -26,12 +26,7 @@
                         break;
                     }
                     _ => {
-                        let value =
-                            super::super::protocol_serde::shape_replica_global_secondary_index_description::de_replica_global_secondary_index_description(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?;
+                        let value = super::super::protocol_serde::shape_replica_global_secondary_index_description::de_replica_global_secondary_index_description(tokens, _value, depth + 1)?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_replica_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_replica_not_found_exception.rs
+++ generated/src/protocol_serde/shape_replica_not_found_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_replica_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ReplicaNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ReplicaNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::ReplicaNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_replica_settings_description_list.rs`

```diff
--- reference/src/protocol_serde/shape_replica_settings_description_list.rs
+++ generated/src/protocol_serde/shape_replica_settings_description_list.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::ReplicaSettingsDescription>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::ReplicaSettingsDescription>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -23,8 +26,11 @@
                         break;
                     }
                     _ => {
-                        let value =
-                            super::super::protocol_serde::shape_replica_settings_description::de_replica_settings_description(tokens, _value, depth + 1)?;
+                        let value = super::super::protocol_serde::shape_replica_settings_description::de_replica_settings_description(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_replica_settings_update.rs`

```diff
--- reference/src/protocol_serde/shape_replica_settings_update.rs
+++ generated/src/protocol_serde/shape_replica_settings_update.rs
@@ -24,10 +24,7 @@
             {
                 #[allow(unused_mut)]
                 let mut object_7 = array_5.value().start_object();
-                super::super::protocol_serde::shape_replica_global_secondary_index_settings_update::ser_replica_global_secondary_index_settings_update(
-                    &mut object_7,
-                    item_6,
-                )?;
+                super::super::protocol_serde::shape_replica_global_secondary_index_settings_update::ser_replica_global_secondary_index_settings_update(&mut object_7, item_6)?;
                 object_7.finish();
             }
         }
```

### `src/protocol_serde/shape_request_limit_exceeded.rs`

```diff
--- reference/src/protocol_serde/shape_request_limit_exceeded.rs
+++ generated/src/protocol_serde/shape_request_limit_exceeded.rs
@@ -2,7 +2,8 @@
 pub(crate) fn de_request_limit_exceeded_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::RequestLimitExceededBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::RequestLimitExceededBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<super::super::types::error::builders::RequestLimitExceededBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -11,23 +12,23 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "ThrottlingReasons" => {
-                    builder = builder.set_throttling_reasons(super::super::protocol_serde::shape_throttling_reason_list::de_throttling_reason_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                .transpose()?,
+                        );
+                    }
+                    "ThrottlingReasons" => {
+                        builder = builder.set_throttling_reasons(
+                            super::super::protocol_serde::shape_throttling_reason_list::de_throttling_reason_list(tokens, _value, depth + 1)?,
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_resource_not_found_exception.rs`

```diff
--- reference/src/protocol_serde/shape_resource_not_found_exception.rs
+++ generated/src/protocol_serde/shape_resource_not_found_exception.rs
@@ -2,7 +2,10 @@
 pub(crate) fn de_resource_not_found_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ResourceNotFoundExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ResourceNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    super::super::types::error::builders::ResourceNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_restore_table_from_backup.rs`

```diff
--- reference/src/protocol_serde/shape_restore_table_from_backup.rs
+++ generated/src/protocol_serde/shape_restore_table_from_backup.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -44,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::BackupNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_backup_not_found_exception::de_backup_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_backup_not_found_exception::de_backup_not_found_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -74,8 +71,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -99,22 +97,26 @@
             }
             tmp
         }),
-        "TableAlreadyExistsException" => super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::TableAlreadyExistsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "TableAlreadyExistsException" => {
+            super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::TableAlreadyExistsException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::TableAlreadyExistsExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_table_already_exists_exception::de_table_already_exists_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::TableAlreadyExistsExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_table_already_exists_exception::de_table_already_exists_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "TableInUseException" => super::super::operation::restore_table_from_backup::RestoreTableFromBackupError::TableInUseException({
             #[allow(unused_mut)]
             let mut tmp = {
```

### `src/protocol_serde/shape_restore_table_to_point_in_time.rs`

```diff
--- reference/src/protocol_serde/shape_restore_table_to_point_in_time.rs
+++ generated/src/protocol_serde/shape_restore_table_to_point_in_time.rs
@@ -35,21 +35,26 @@
             }
             tmp
         }),
-        "InvalidEndpointException" => super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::InvalidEndpointException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidEndpointException" => {
+            super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::InvalidEndpointException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "InvalidRestoreTimeException" => {
             super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::InvalidRestoreTimeException({
                 #[allow(unused_mut)]
@@ -70,21 +75,24 @@
                 tmp
             })
         }
-        "LimitExceededException" => super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::LimitExceededException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "LimitExceededException" => {
+            super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::LimitExceededException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::LimitExceededExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::LimitExceededExceptionBuilder::default();
+                    output =
+                        super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
+                            .map_err(super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "PointInTimeRecoveryUnavailableException" => {
             super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::PointInTimeRecoveryUnavailableException({
                 #[allow(unused_mut)]
@@ -136,21 +144,24 @@
             }
             tmp
         }),
-        "TableNotFoundException" => super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::TableNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "TableNotFoundException" => {
+            super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::TableNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::TableNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_table_not_found_exception::de_table_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::TableNotFoundExceptionBuilder::default();
+                    output =
+                        super::super::protocol_serde::shape_table_not_found_exception::de_table_not_found_exception_json_err(_response_body, output)
+                            .map_err(super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => super::super::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError::generic(generic),
     })
 }
```

### `src/protocol_serde/shape_scan.rs`

```diff
--- reference/src/protocol_serde/shape_scan.rs
+++ generated/src/protocol_serde/shape_scan.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::scan::ScanError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::scan::ScanError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -47,22 +48,20 @@
             }
             tmp
         }),
-        "ProvisionedThroughputExceededException" => {
-            super::super::operation::scan::ScanError::ProvisionedThroughputExceededException({
+        "ProvisionedThroughputExceededException" => super::super::operation::scan::ScanError::ProvisionedThroughputExceededException({
+            #[allow(unused_mut)]
+            let mut tmp = {
                 #[allow(unused_mut)]
-                let mut tmp = {
-                    #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::scan::ScanError::unhandled)?;
-                    let output = output.meta(generic);
-                    output.build()
-                };
-                if tmp.message.is_none() {
-                    tmp.message = _error_message;
-                }
-                tmp
-            })
-        }
+                let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::scan::ScanError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "RequestLimitExceeded" => super::super::operation::scan::ScanError::RequestLimitExceeded({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -83,8 +82,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::scan::ScanError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::scan::ScanError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -121,7 +123,8 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::scan::builders::ScanOutputBuilder::default();
-        output = super::super::protocol_serde::shape_scan::de_scan(_response_body, output).map_err(super::super::operation::scan::ScanError::unhandled)?;
+        output =
+            super::super::protocol_serde::shape_scan::de_scan(_response_body, output).map_err(super::super::operation::scan::ScanError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_search_result_item.rs`

```diff
--- reference/src/protocol_serde/shape_search_result_item.rs
+++ generated/src/protocol_serde/shape_search_result_item.rs
@@ -22,7 +22,11 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "Item" => {
-                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
+                            builder = builder.set_item(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         "Score" => {
                             builder = builder
```

### `src/protocol_serde/shape_search_schema_element.rs`

```diff
--- reference/src/protocol_serde/shape_search_schema_element.rs
+++ generated/src/protocol_serde/shape_search_schema_element.rs
@@ -57,9 +57,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::search_schema_element_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::search_schema_element_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_search_vectors.rs`

```diff
--- reference/src/protocol_serde/shape_search_vectors.rs
+++ generated/src/protocol_serde/shape_search_vectors.rs
@@ -52,8 +52,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::search_vectors::SearchVectorsError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::search_vectors::SearchVectorsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_source_table_details.rs`

```diff
--- reference/src/protocol_serde/shape_source_table_details.rs
+++ generated/src/protocol_serde/shape_source_table_details.rs
@@ -20,72 +20,71 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        match key.to_unescaped()?.as_ref() {
-                            "TableName" => {
-                                builder = builder.set_table_name(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "TableId" => {
-                                builder = builder.set_table_id(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "TableArn" => {
-                                builder = builder.set_table_arn(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                        .transpose()?,
-                                );
-                            }
-                            "TableSizeBytes" => {
-                                builder = builder.set_table_size_bytes(
-                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                        .map(i64::try_from)
-                                        .transpose()?,
-                                );
-                            }
-                            "KeySchema" => {
-                                builder = builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
-                            }
-                            "TableCreationDateTime" => {
-                                builder = builder.set_table_creation_date_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                                    tokens.next(),
-                                    ::aws_smithy_types::date_time::Format::EpochSeconds,
-                                )?);
-                            }
-                            "ProvisionedThroughput" => {
-                                builder = builder.set_provisioned_throughput(
-                                    super::super::protocol_serde::shape_provisioned_throughput::de_provisioned_throughput(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "OnDemandThroughput" => {
-                                builder = builder.set_on_demand_throughput(
-                                    super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
-                                );
-                            }
-                            "ItemCount" => {
-                                builder = builder.set_item_count(
-                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                        .map(i64::try_from)
-                                        .transpose()?,
-                                );
-                            }
-                            "BillingMode" => {
-                                builder = builder.set_billing_mode(
-                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                        .map(|s| s.to_unescaped().map(|u| super::super::types::BillingMode::from(u.as_ref())))
-                                        .transpose()?,
-                                );
-                            }
-                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "TableName" => {
+                            builder = builder.set_table_name(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "TableId" => {
+                            builder = builder.set_table_id(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "TableArn" => {
+                            builder = builder.set_table_arn(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "TableSizeBytes" => {
+                            builder = builder.set_table_size_bytes(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i64::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "KeySchema" => {
+                            builder =
+                                builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
+                        }
+                        "TableCreationDateTime" => {
+                            builder = builder.set_table_creation_date_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                                tokens.next(),
+                                ::aws_smithy_types::date_time::Format::EpochSeconds,
+                            )?);
+                        }
+                        "ProvisionedThroughput" => {
+                            builder = builder.set_provisioned_throughput(
+                                super::super::protocol_serde::shape_provisioned_throughput::de_provisioned_throughput(tokens, _value, depth + 1)?,
+                            );
+                        }
+                        "OnDemandThroughput" => {
+                            builder = builder.set_on_demand_throughput(
+                                super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
+                            );
+                        }
+                        "ItemCount" => {
+                            builder = builder.set_item_count(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i64::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "BillingMode" => {
+                            builder = builder.set_billing_mode(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| super::super::types::BillingMode::from(u.as_ref())))
+                                    .transpose()?,
+                            );
                         }
-                    }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
@@ -93,9 +92,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::source_table_details_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::source_table_details_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_source_table_feature_details.rs`

```diff
--- reference/src/protocol_serde/shape_source_table_feature_details.rs
+++ generated/src/protocol_serde/shape_source_table_feature_details.rs
@@ -32,11 +32,9 @@
                             );
                         }
                         "StreamDescription" => {
-                            builder = builder.set_stream_description(super::super::protocol_serde::shape_stream_specification::de_stream_specification(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
+                            builder = builder.set_stream_description(
+                                super::super::protocol_serde::shape_stream_specification::de_stream_specification(tokens, _value, depth + 1)?,
+                            );
                         }
                         "TimeToLiveDescription" => {
                             builder = builder.set_time_to_live_description(
```

### `src/protocol_serde/shape_stream_specification.rs`

```diff
--- reference/src/protocol_serde/shape_stream_specification.rs
+++ generated/src/protocol_serde/shape_stream_specification.rs
@@ -53,9 +53,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::stream_specification_correct_errors(builder).build().map_err(
-                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
-            )?))
+            Ok(Some(
+                super::super::serde_util::stream_specification_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_table_already_exists_exception.rs`

```diff
--- reference/src/protocol_serde/shape_table_already_exists_exception.rs
+++ generated/src/protocol_serde/shape_table_already_exists_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_table_already_exists_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::TableAlreadyExistsExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::TableAlreadyExistsExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::TableAlreadyExistsExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_table_auto_scaling_description.rs`

```diff
--- reference/src/protocol_serde/shape_table_auto_scaling_description.rs
+++ generated/src/protocol_serde/shape_table_auto_scaling_description.rs
@@ -20,32 +20,28 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "TableName" => {
-                            builder = builder.set_table_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "TableStatus" => {
-                            builder = builder.set_table_status(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| super::super::types::TableStatus::from(u.as_ref())))
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "TableName" => {
+                                builder = builder.set_table_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "TableStatus" => {
+                                builder = builder.set_table_status(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| super::super::types::TableStatus::from(u.as_ref())))
+                                        .transpose()?,
+                                );
+                            }
+                            "Replicas" => {
+                                builder = builder.set_replicas(super::super::protocol_serde::shape_replica_auto_scaling_description_list::de_replica_auto_scaling_description_list(tokens, _value, depth + 1)?);
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        "Replicas" => {
-                            builder = builder.set_replicas(
-                                super::super::protocol_serde::shape_replica_auto_scaling_description_list::de_replica_auto_scaling_description_list(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_table_creation_parameters.rs`

```diff
--- reference/src/protocol_serde/shape_table_creation_parameters.rs
+++ generated/src/protocol_serde/shape_table_creation_parameters.rs
@@ -113,7 +113,8 @@
                             );
                         }
                         "KeySchema" => {
-                            builder = builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
+                            builder =
+                                builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
                         }
                         "BillingMode" => {
                             builder = builder.set_billing_mode(
@@ -128,11 +129,9 @@
                             );
                         }
                         "OnDemandThroughput" => {
-                            builder = builder.set_on_demand_throughput(super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
+                            builder = builder.set_on_demand_throughput(
+                                super::super::protocol_serde::shape_on_demand_throughput::de_on_demand_throughput(tokens, _value, depth + 1)?,
+                            );
                         }
                         "SSESpecification" => {
                             builder = builder.set_sse_specification(super::super::protocol_serde::shape_sse_specification::de_sse_specification(
@@ -143,7 +142,11 @@
                         }
                         "GlobalSecondaryIndexes" => {
                             builder = builder.set_global_secondary_indexes(
-                                super::super::protocol_serde::shape_global_secondary_index_list::de_global_secondary_index_list(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_global_secondary_index_list::de_global_secondary_index_list(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         "VectorIndexes" => {
```

### `src/protocol_serde/shape_table_description.rs`

```diff
--- reference/src/protocol_serde/shape_table_description.rs
+++ generated/src/protocol_serde/shape_table_description.rs
@@ -35,7 +35,8 @@
                                 );
                             }
                             "KeySchema" => {
-                                builder = builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
+                                builder =
+                                    builder.set_key_schema(super::super::protocol_serde::shape_key_schema::de_key_schema(tokens, _value, depth + 1)?);
                             }
                             "TableStatus" => {
                                 builder = builder.set_table_status(
@@ -93,22 +94,10 @@
                                 );
                             }
                             "LocalSecondaryIndexes" => {
-                                builder = builder.set_local_secondary_indexes(
-                                    super::super::protocol_serde::shape_local_secondary_index_description_list::de_local_secondary_index_description_list(
-                                        tokens,
-                                        _value,
-                                        depth + 1,
-                                    )?,
-                                );
+                                builder = builder.set_local_secondary_indexes(super::super::protocol_serde::shape_local_secondary_index_description_list::de_local_secondary_index_description_list(tokens, _value, depth + 1)?);
                             }
                             "GlobalSecondaryIndexes" => {
-                                builder = builder.set_global_secondary_indexes(
-                                    super::super::protocol_serde::shape_global_secondary_index_description_list::de_global_secondary_index_description_list(
-                                        tokens,
-                                        _value,
-                                        depth + 1,
-                                    )?,
-                                );
+                                builder = builder.set_global_secondary_indexes(super::super::protocol_serde::shape_global_secondary_index_description_list::de_global_secondary_index_description_list(tokens, _value, depth + 1)?);
                             }
                             "StreamSpecification" => {
                                 builder = builder.set_stream_specification(
@@ -137,20 +126,15 @@
                                 );
                             }
                             "Replicas" => {
-                                builder = builder.set_replicas(super::super::protocol_serde::shape_replica_description_list::de_replica_description_list(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?);
-                            }
-                            "GlobalTableWitnesses" => {
-                                builder = builder.set_global_table_witnesses(
-                                    super::super::protocol_serde::shape_global_table_witness_description_list::de_global_table_witness_description_list(
+                                builder =
+                                    builder.set_replicas(super::super::protocol_serde::shape_replica_description_list::de_replica_description_list(
                                         tokens,
                                         _value,
                                         depth + 1,
-                                    )?,
-                                );
+                                    )?);
+                            }
+                            "GlobalTableWitnesses" => {
+                                builder = builder.set_global_table_witnesses(super::super::protocol_serde::shape_global_table_witness_description_list::de_global_table_witness_description_list(tokens, _value, depth + 1)?);
                             }
                             "GlobalTableSettingsReplicationMode" => {
                                 builder = builder.set_global_table_settings_replication_mode(
@@ -184,11 +168,9 @@
                                 )?);
                             }
                             "TableClassSummary" => {
-                                builder = builder.set_table_class_summary(super::super::protocol_serde::shape_table_class_summary::de_table_class_summary(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?);
+                                builder = builder.set_table_class_summary(
+                                    super::super::protocol_serde::shape_table_class_summary::de_table_class_summary(tokens, _value, depth + 1)?,
+                                );
                             }
                             "DeletionProtectionEnabled" => {
                                 builder = builder
```

### `src/protocol_serde/shape_tag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_tag_resource.rs
+++ generated/src/protocol_serde/shape_tag_resource.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::tag_resource::TagResourceError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::tag_resource::TagResourceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,8 +83,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::tag_resource::TagResourceError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::tag_resource::TagResourceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,3 +123,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_tag_resource(
+    _value: &[u8],
+    mut builder: super::super::operation::tag_resource::builders::TagResourceOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::tag_resource::builders::TagResourceOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_throttling_exception.rs`

```diff
--- reference/src/protocol_serde/shape_throttling_exception.rs
+++ generated/src/protocol_serde/shape_throttling_exception.rs
@@ -2,7 +2,8 @@
 pub(crate) fn de_throttling_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::ThrottlingExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ThrottlingExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<super::super::types::error::builders::ThrottlingExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -11,23 +12,23 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "throttlingReasons" => {
-                    builder = builder.set_throttling_reasons(super::super::protocol_serde::shape_throttling_reason_list::de_throttling_reason_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                .transpose()?,
+                        );
+                    }
+                    "throttlingReasons" => {
+                        builder = builder.set_throttling_reasons(
+                            super::super::protocol_serde::shape_throttling_reason_list::de_throttling_reason_list(tokens, _value, depth + 1)?,
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_time_to_live_specification.rs`

```diff
--- reference/src/protocol_serde/shape_time_to_live_specification.rs
+++ generated/src/protocol_serde/shape_time_to_live_specification.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_time_to_live_specification(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::TimeToLiveSpecification,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("Enabled").boolean(input.enabled);
+    }
+    {
+        object.key("AttributeName").string(input.attribute_name.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_time_to_live_specification<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -51,16 +64,3 @@
         )),
     }
 }
-
-pub fn ser_time_to_live_specification(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::TimeToLiveSpecification,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("Enabled").boolean(input.enabled);
-    }
-    {
-        object.key("AttributeName").string(input.attribute_name.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_transact_get_items.rs`

```diff
--- reference/src/protocol_serde/shape_transact_get_items.rs
+++ generated/src/protocol_serde/shape_transact_get_items.rs
@@ -4,7 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::transact_get_items::TransactGetItemsOutput, super::super::operation::transact_get_items::TransactGetItemsError> {
+) -> std::result::Result<
+    super::super::operation::transact_get_items::TransactGetItemsOutput,
+    super::super::operation::transact_get_items::TransactGetItemsError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::transact_get_items::TransactGetItemsError::unhandled)?;
@@ -37,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::transact_get_items::TransactGetItemsError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +87,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::transact_get_items::TransactGetItemsError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,9 +120,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TransactionCanceledExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_transaction_canceled_exception::de_transaction_canceled_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::transact_get_items::TransactGetItemsError::unhandled)?;
+                output = super::super::protocol_serde::shape_transaction_canceled_exception::de_transaction_canceled_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::transact_get_items::TransactGetItemsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -133,7 +142,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::transact_get_items::TransactGetItemsOutput, super::super::operation::transact_get_items::TransactGetItemsError> {
+) -> std::result::Result<
+    super::super::operation::transact_get_items::TransactGetItemsOutput,
+    super::super::operation::transact_get_items::TransactGetItemsError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::transact_get_items::builders::TransactGetItemsOutputBuilder::default();
@@ -169,23 +181,21 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(
-                            super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    "Responses" => {
-                        builder = builder.set_responses(super::super::protocol_serde::shape_item_response_list::de_item_response_list(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(
+                        super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
+                    );
+                }
+                "Responses" => {
+                    builder = builder.set_responses(super::super::protocol_serde::shape_item_response_list::de_item_response_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-            }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_transact_write_items.rs`

```diff
--- reference/src/protocol_serde/shape_transact_write_items.rs
+++ generated/src/protocol_serde/shape_transact_write_items.rs
@@ -26,12 +26,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::IdempotentParameterMismatchExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_idempotent_parameter_mismatch_exception::de_idempotent_parameter_mismatch_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::transact_write_items::TransactWriteItemsError::unhandled)?;
+                    output = super::super::protocol_serde::shape_idempotent_parameter_mismatch_exception::de_idempotent_parameter_mismatch_exception_json_err(_response_body, output).map_err(super::super::operation::transact_write_items::TransactWriteItemsError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -61,8 +56,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::transact_write_items::TransactWriteItemsError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::transact_write_items::TransactWriteItemsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -107,8 +103,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::transact_write_items::TransactWriteItemsError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::transact_write_items::TransactWriteItemsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -137,9 +136,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TransactionCanceledExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_transaction_canceled_exception::de_transaction_canceled_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::transact_write_items::TransactWriteItemsError::unhandled)?;
+                output = super::super::protocol_serde::shape_transaction_canceled_exception::de_transaction_canceled_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::transact_write_items::TransactWriteItemsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -214,25 +215,23 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(
-                            super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    "ItemCollectionMetrics" => {
-                        builder = builder.set_item_collection_metrics(
-                            super::super::protocol_serde::shape_item_collection_metrics_per_table::de_item_collection_metrics_per_table(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(
+                        super::super::protocol_serde::shape_consumed_capacity_multiple::de_consumed_capacity_multiple(tokens, _value, depth + 1)?,
+                    );
+                }
+                "ItemCollectionMetrics" => {
+                    builder = builder.set_item_collection_metrics(
+                        super::super::protocol_serde::shape_item_collection_metrics_per_table::de_item_collection_metrics_per_table(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
                 }
-            }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_transaction_canceled_exception.rs`

```diff
--- reference/src/protocol_serde/shape_transaction_canceled_exception.rs
+++ generated/src/protocol_serde/shape_transaction_canceled_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_transaction_canceled_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::TransactionCanceledExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::TransactionCanceledExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::TransactionCanceledExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -12,23 +14,21 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "Message" => {
-                        builder = builder.set_message(
-                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                .transpose()?,
-                        );
-                    }
-                    "CancellationReasons" => {
-                        builder = builder.set_cancellation_reasons(
-                            super::super::protocol_serde::shape_cancellation_reason_list::de_cancellation_reason_list(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Message" => {
+                    builder = builder.set_message(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "CancellationReasons" => {
+                    builder = builder.set_cancellation_reasons(
+                        super::super::protocol_serde::shape_cancellation_reason_list::de_cancellation_reason_list(tokens, _value, depth + 1)?,
+                    );
                 }
-            }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_transaction_conflict_exception.rs`

```diff
--- reference/src/protocol_serde/shape_transaction_conflict_exception.rs
+++ generated/src/protocol_serde/shape_transaction_conflict_exception.rs
@@ -2,8 +2,10 @@
 pub(crate) fn de_transaction_conflict_exception_json_err(
     _value: &[u8],
     mut builder: super::super::types::error::builders::TransactionConflictExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::TransactionConflictExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::types::error::builders::TransactionConflictExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_untag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_untag_resource.rs
+++ generated/src/protocol_serde/shape_untag_resource.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::untag_resource::UntagResourceError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::untag_resource::UntagResourceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,8 +83,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::untag_resource::UntagResourceError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::untag_resource::UntagResourceError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,3 +123,36 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_untag_resource(
+    _value: &[u8],
+    mut builder: super::super::operation::untag_resource::builders::UntagResourceOutputBuilder,
+) -> ::std::result::Result<
+    super::super::operation::untag_resource::builders::UntagResourceOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_continuous_backups.rs`

```diff
--- reference/src/protocol_serde/shape_update_continuous_backups.rs
+++ generated/src/protocol_serde/shape_update_continuous_backups.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -30,12 +26,7 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ContinuousBackupsUnavailableExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_continuous_backups_unavailable_exception::de_continuous_backups_unavailable_exception_json_err(
-                            _response_body,
-                            output,
-                        )
-                        .map_err(super::super::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
+                    output = super::super::protocol_serde::shape_continuous_backups_unavailable_exception::de_continuous_backups_unavailable_exception_json_err(_response_body, output).map_err(super::super::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -65,8 +56,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -141,7 +133,11 @@
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "ContinuousBackupsDescription" => {
                     builder = builder.set_continuous_backups_description(
-                        super::super::protocol_serde::shape_continuous_backups_description::de_continuous_backups_description(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_continuous_backups_description::de_continuous_backups_description(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_update_contributor_insights.rs`

```diff
--- reference/src/protocol_serde/shape_update_contributor_insights.rs
+++ generated/src/protocol_serde/shape_update_contributor_insights.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::update_contributor_insights::UpdateContributorInsightsError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::update_contributor_insights::UpdateContributorInsightsError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -39,21 +35,26 @@
             }
             tmp
         }),
-        "ResourceNotFoundException" => super::super::operation::update_contributor_insights::UpdateContributorInsightsError::ResourceNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "ResourceNotFoundException" => {
+            super::super::operation::update_contributor_insights::UpdateContributorInsightsError::ResourceNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::update_contributor_insights::UpdateContributorInsightsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => super::super::operation::update_contributor_insights::UpdateContributorInsightsError::generic(generic),
     })
 }
```

### `src/protocol_serde/shape_update_global_table.rs`

```diff
--- reference/src/protocol_serde/shape_update_global_table.rs
+++ generated/src/protocol_serde/shape_update_global_table.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::update_global_table::UpdateGlobalTableOutput, super::super::operation::update_global_table::UpdateGlobalTableError>
-{
+) -> std::result::Result<
+    super::super::operation::update_global_table::UpdateGlobalTableOutput,
+    super::super::operation::update_global_table::UpdateGlobalTableError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::update_global_table::UpdateGlobalTableError::unhandled)?;
@@ -56,8 +58,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_global_table::UpdateGlobalTableError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_global_table::UpdateGlobalTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -89,8 +92,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ReplicaNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_replica_not_found_exception::de_replica_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_global_table::UpdateGlobalTableError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_replica_not_found_exception::de_replica_not_found_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_global_table::UpdateGlobalTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -123,8 +127,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::update_global_table::UpdateGlobalTableOutput, super::super::operation::update_global_table::UpdateGlobalTableError>
-{
+) -> std::result::Result<
+    super::super::operation::update_global_table::UpdateGlobalTableOutput,
+    super::super::operation::update_global_table::UpdateGlobalTableError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::update_global_table::builders::UpdateGlobalTableOutputBuilder::default();
```

### `src/protocol_serde/shape_update_global_table_settings.rs`

```diff
--- reference/src/protocol_serde/shape_update_global_table_settings.rs
+++ generated/src/protocol_serde/shape_update_global_table_settings.rs
@@ -15,11 +15,7 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => {
-            return Err(super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::unhandled(
-                generic,
-            ))
-        }
+        None => return Err(super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::unhandled(generic)),
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -74,21 +70,26 @@
             }
             tmp
         }),
-        "InvalidEndpointException" => super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::InvalidEndpointException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidEndpointException" => {
+            super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::InvalidEndpointException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "LimitExceededException" => super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::LimitExceededException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -104,21 +105,26 @@
             }
             tmp
         }),
-        "ReplicaNotFoundException" => super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::ReplicaNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "ReplicaNotFoundException" => {
+            super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::ReplicaNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::ReplicaNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_replica_not_found_exception::de_replica_not_found_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::ReplicaNotFoundExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_replica_not_found_exception::de_replica_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "ResourceInUseException" => super::super::operation::update_global_table_settings::UpdateGlobalTableSettingsError::ResourceInUseException({
             #[allow(unused_mut)]
             let mut tmp = {
```

### `src/protocol_serde/shape_update_item.rs`

```diff
--- reference/src/protocol_serde/shape_update_item.rs
+++ generated/src/protocol_serde/shape_update_item.rs
@@ -55,8 +55,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -65,27 +66,13 @@
             }
             tmp
         }),
-        "ItemCollectionSizeLimitExceededException" => super::super::operation::update_item::UpdateItemError::ItemCollectionSizeLimitExceededException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::ItemCollectionSizeLimitExceededExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_item_collection_size_limit_exceeded_exception::de_item_collection_size_limit_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ProvisionedThroughputExceededException" => {
-            super::super::operation::update_item::UpdateItemError::ProvisionedThroughputExceededException({
+        "ItemCollectionSizeLimitExceededException" => {
+            super::super::operation::update_item::UpdateItemError::ItemCollectionSizeLimitExceededException({
                 #[allow(unused_mut)]
                 let mut tmp = {
                     #[allow(unused_mut)]
-                    let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
+                    let mut output = super::super::types::error::builders::ItemCollectionSizeLimitExceededExceptionBuilder::default();
+                    output = super::super::protocol_serde::shape_item_collection_size_limit_exceeded_exception::de_item_collection_size_limit_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -95,6 +82,20 @@
                 tmp
             })
         }
+        "ProvisionedThroughputExceededException" => super::super::operation::update_item::UpdateItemError::ProvisionedThroughputExceededException({
+            #[allow(unused_mut)]
+            let mut tmp = {
+                #[allow(unused_mut)]
+                let mut output = super::super::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
+                output = super::super::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
+                let output = output.meta(generic);
+                output.build()
+            };
+            if tmp.message.is_none() {
+                tmp.message = _error_message;
+            }
+            tmp
+        }),
         "ReplicatedWriteConflictException" => super::super::operation::update_item::UpdateItemError::ReplicatedWriteConflictException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -133,8 +134,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -163,9 +167,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::TransactionConflictExceptionBuilder::default();
-                output =
-                    super::super::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
+                output = super::super::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::update_item::UpdateItemError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -207,8 +213,10 @@
 pub(crate) fn de_update_item(
     _value: &[u8],
     mut builder: super::super::operation::update_item::builders::UpdateItemOutputBuilder,
-) -> ::std::result::Result<super::super::operation::update_item::builders::UpdateItemOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::update_item::builders::UpdateItemOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
@@ -217,26 +225,28 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                match key.to_unescaped()?.as_ref() {
-                    "Attributes" => {
-                        builder = builder.set_attributes(super::super::protocol_serde::shape_attribute_map::de_attribute_map(tokens, _value, depth + 1)?);
-                    }
-                    "ConsumedCapacity" => {
-                        builder = builder.set_consumed_capacity(super::super::protocol_serde::shape_consumed_capacity::de_consumed_capacity(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?);
-                    }
-                    "ItemCollectionMetrics" => {
-                        builder = builder.set_item_collection_metrics(
-                            super::super::protocol_serde::shape_item_collection_metrics::de_item_collection_metrics(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Attributes" => {
+                    builder = builder.set_attributes(super::super::protocol_serde::shape_attribute_map::de_attribute_map(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-            }
+                "ConsumedCapacity" => {
+                    builder = builder.set_consumed_capacity(super::super::protocol_serde::shape_consumed_capacity::de_consumed_capacity(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "ItemCollectionMetrics" => {
+                    builder = builder.set_item_collection_metrics(
+                        super::super::protocol_serde::shape_item_collection_metrics::de_item_collection_metrics(tokens, _value, depth + 1)?,
+                    );
+                }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
             other => {
                 return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                     "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_update_kinesis_streaming_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_update_kinesis_streaming_configuration.rs
+++ generated/src/protocol_serde/shape_update_kinesis_streaming_configuration.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_update_kinesis_streaming_configuration(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::UpdateKinesisStreamingConfiguration,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.approximate_creation_date_time_precision {
+        object.key("ApproximateCreationDateTimePrecision").string(var_1.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_update_kinesis_streaming_configuration<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -47,13 +57,3 @@
         )),
     }
 }
-
-pub fn ser_update_kinesis_streaming_configuration(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::UpdateKinesisStreamingConfiguration,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.approximate_creation_date_time_precision {
-        object.key("ApproximateCreationDateTimePrecision").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_update_kinesis_streaming_destination.rs`

```diff
--- reference/src/protocol_serde/shape_update_kinesis_streaming_destination.rs
+++ generated/src/protocol_serde/shape_update_kinesis_streaming_destination.rs
@@ -15,7 +15,9 @@
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled(generic)),
+        None => {
+            return Err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled(generic))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -43,8 +45,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled)?;
+                    output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -60,8 +65,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::LimitExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
+                            .map_err(
+                                super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled,
+                            )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -77,8 +85,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceInUseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
+                            .map_err(
+                                super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled,
+                            )?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -94,9 +105,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled)?;
+                    output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -121,9 +134,11 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationOutputBuilder::default();
-        output = super::super::protocol_serde::shape_update_kinesis_streaming_destination::de_update_kinesis_streaming_destination(_response_body, output)
-            .map_err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled)?;
+        let mut output =
+            super::super::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationOutputBuilder::default();
+        output =
+            super::super::protocol_serde::shape_update_kinesis_streaming_destination::de_update_kinesis_streaming_destination(_response_body, output)
+                .map_err(super::super::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_update_table.rs`

```diff
--- reference/src/protocol_serde/shape_update_table.rs
+++ generated/src/protocol_serde/shape_update_table.rs
@@ -37,8 +37,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_table::UpdateTableError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_table::UpdateTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -82,8 +83,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_table::UpdateTableError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::update_table::UpdateTableError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -125,8 +129,10 @@
 pub(crate) fn de_update_table(
     _value: &[u8],
     mut builder: super::super::operation::update_table::builders::UpdateTableOutputBuilder,
-) -> ::std::result::Result<super::super::operation::update_table::builders::UpdateTableOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
-{
+) -> ::std::result::Result<
+    super::super::operation::update_table::builders::UpdateTableOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
```

### `src/protocol_serde/shape_update_table_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_table_input.rs
+++ generated/src/protocol_serde/shape_update_table_input.rs
@@ -78,7 +78,10 @@
             {
                 #[allow(unused_mut)]
                 let mut object_27 = array_25.value().start_object();
-                super::super::protocol_serde::shape_global_table_witness_group_update::ser_global_table_witness_group_update(&mut object_27, item_26)?;
+                super::super::protocol_serde::shape_global_table_witness_group_update::ser_global_table_witness_group_update(
+                    &mut object_27,
+                    item_26,
+                )?;
                 object_27.finish();
             }
         }
```

### `src/protocol_serde/shape_update_table_replica_auto_scaling.rs`

```diff
--- reference/src/protocol_serde/shape_update_table_replica_auto_scaling.rs
+++ generated/src/protocol_serde/shape_update_table_replica_auto_scaling.rs
@@ -20,21 +20,23 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InternalServerError" => super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::InternalServerError({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InternalServerError" => {
+            super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::InternalServerError({
                 #[allow(unused_mut)]
-                let mut output = super::super::types::error::builders::InternalServerErrorBuilder::default();
-                output = super::super::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = super::super::types::error::builders::InternalServerErrorBuilder::default();
+                    output = super::super::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "LimitExceededException" => {
             super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::LimitExceededException({
                 #[allow(unused_mut)]
@@ -41,8 +43,9 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::LimitExceededExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
+                            .map_err(super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -58,8 +61,9 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceInUseExceptionBuilder::default();
-                    output = super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
-                        .map_err(super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::unhandled)?;
+                    output =
+                        super::super::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
+                            .map_err(super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -75,9 +79,11 @@
                 let mut tmp = {
                     #[allow(unused_mut)]
                     let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                    output =
-                        super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                            .map_err(super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::unhandled)?;
+                    output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                        _response_body,
+                        output,
+                    )
+                    .map_err(super::super::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::unhandled)?;
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -115,7 +121,10 @@
 ) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    super::super::protocol_serde::shape_update_table_replica_auto_scaling_input::ser_update_table_replica_auto_scaling_input_input(&mut object, input)?;
+    super::super::protocol_serde::shape_update_table_replica_auto_scaling_input::ser_update_table_replica_auto_scaling_input_input(
+        &mut object,
+        input,
+    )?;
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
@@ -138,7 +147,11 @@
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "TableAutoScalingDescription" => {
                     builder = builder.set_table_auto_scaling_description(
-                        super::super::protocol_serde::shape_table_auto_scaling_description::de_table_auto_scaling_description(tokens, _value, depth + 1)?,
+                        super::super::protocol_serde::shape_table_auto_scaling_description::de_table_auto_scaling_description(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_update_time_to_live.rs`

```diff
--- reference/src/protocol_serde/shape_update_time_to_live.rs
+++ generated/src/protocol_serde/shape_update_time_to_live.rs
@@ -4,8 +4,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::update_time_to_live::UpdateTimeToLiveOutput, super::super::operation::update_time_to_live::UpdateTimeToLiveError>
-{
+) -> std::result::Result<
+    super::super::operation::update_time_to_live::UpdateTimeToLiveOutput,
+    super::super::operation::update_time_to_live::UpdateTimeToLiveError,
+> {
     #[allow(unused_mut)]
     let mut generic_builder = super::super::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
         .map_err(super::super::operation::update_time_to_live::UpdateTimeToLiveError::unhandled)?;
@@ -38,8 +40,9 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::InvalidEndpointExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_time_to_live::UpdateTimeToLiveError::unhandled)?;
+                output =
+                    super::super::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
+                        .map_err(super::super::operation::update_time_to_live::UpdateTimeToLiveError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -83,8 +86,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = super::super::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
-                    .map_err(super::super::operation::update_time_to_live::UpdateTimeToLiveError::unhandled)?;
+                output = super::super::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(super::super::operation::update_time_to_live::UpdateTimeToLiveError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -102,8 +108,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<super::super::operation::update_time_to_live::UpdateTimeToLiveOutput, super::super::operation::update_time_to_live::UpdateTimeToLiveError>
-{
+) -> std::result::Result<
+    super::super::operation::update_time_to_live::UpdateTimeToLiveOutput,
+    super::super::operation::update_time_to_live::UpdateTimeToLiveError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = super::super::operation::update_time_to_live::builders::UpdateTimeToLiveOutputBuilder::default();
```

### `src/protocol_serde/shape_vector_index.rs`

```diff
--- reference/src/protocol_serde/shape_vector_index.rs
+++ generated/src/protocol_serde/shape_vector_index.rs
@@ -73,15 +73,23 @@
                         }
                         "VectorAttribute" => {
                             builder = builder.set_vector_attribute(
-                                super::super::protocol_serde::shape_vector_attribute_definition::de_vector_attribute_definition(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_vector_attribute_definition::de_vector_attribute_definition(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         "SearchSchema" => {
-                            builder =
-                                builder.set_search_schema(super::super::protocol_serde::shape_search_schema::de_search_schema(tokens, _value, depth + 1)?);
+                            builder = builder.set_search_schema(super::super::protocol_serde::shape_search_schema::de_search_schema(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         "Projection" => {
-                            builder = builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
+                            builder =
+                                builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
                         }
                         "Dimensions" => {
                             builder = builder.set_dimensions(
@@ -106,9 +114,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::vector_index_correct_errors(builder).build().map_err(|err| {
-                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
-            })?))
+            Ok(Some(super::super::serde_util::vector_index_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_vector_index_description.rs`

```diff
--- reference/src/protocol_serde/shape_vector_index_description.rs
+++ generated/src/protocol_serde/shape_vector_index_description.rs
@@ -29,15 +29,23 @@
                             );
                         }
                         "SearchSchema" => {
-                            builder =
-                                builder.set_search_schema(super::super::protocol_serde::shape_search_schema::de_search_schema(tokens, _value, depth + 1)?);
+                            builder = builder.set_search_schema(super::super::protocol_serde::shape_search_schema::de_search_schema(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         "Projection" => {
-                            builder = builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
+                            builder =
+                                builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
                         }
                         "VectorAttribute" => {
                             builder = builder.set_vector_attribute(
-                                super::super::protocol_serde::shape_vector_attribute_definition::de_vector_attribute_definition(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_vector_attribute_definition::de_vector_attribute_definition(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         "Dimensions" => {
```

### `src/protocol_serde/shape_vector_index_description_list.rs`

```diff
--- reference/src/protocol_serde/shape_vector_index_description_list.rs
+++ generated/src/protocol_serde/shape_vector_index_description_list.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<super::super::types::VectorIndexDescription>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<super::super::types::VectorIndexDescription>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -23,7 +26,8 @@
                         break;
                     }
                     _ => {
-                        let value = super::super::protocol_serde::shape_vector_index_description::de_vector_index_description(tokens, _value, depth + 1)?;
+                        let value =
+                            super::super::protocol_serde::shape_vector_index_description::de_vector_index_description(tokens, _value, depth + 1)?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_vector_index_info.rs`

```diff
--- reference/src/protocol_serde/shape_vector_index_info.rs
+++ generated/src/protocol_serde/shape_vector_index_info.rs
@@ -30,15 +30,23 @@
                         }
                         "VectorAttribute" => {
                             builder = builder.set_vector_attribute(
-                                super::super::protocol_serde::shape_vector_attribute_definition::de_vector_attribute_definition(tokens, _value, depth + 1)?,
+                                super::super::protocol_serde::shape_vector_attribute_definition::de_vector_attribute_definition(
+                                    tokens,
+                                    _value,
+                                    depth + 1,
+                                )?,
                             );
                         }
                         "SearchSchema" => {
-                            builder =
-                                builder.set_search_schema(super::super::protocol_serde::shape_search_schema::de_search_schema(tokens, _value, depth + 1)?);
+                            builder = builder.set_search_schema(super::super::protocol_serde::shape_search_schema::de_search_schema(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         "Projection" => {
-                            builder = builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
+                            builder =
+                                builder.set_projection(super::super::protocol_serde::shape_projection::de_projection(tokens, _value, depth + 1)?);
                         }
                         "Dimensions" => {
                             builder = builder.set_dimensions(
```

### `src/protocol_serde/shape_write_request.rs`

```diff
--- reference/src/protocol_serde/shape_write_request.rs
+++ generated/src/protocol_serde/shape_write_request.rs
@@ -41,7 +41,11 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "PutRequest" => {
-                            builder = builder.set_put_request(super::super::protocol_serde::shape_put_request::de_put_request(tokens, _value, depth + 1)?);
+                            builder = builder.set_put_request(super::super::protocol_serde::shape_put_request::de_put_request(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?);
                         }
                         "DeleteRequest" => {
                             builder = builder.set_delete_request(super::super::protocol_serde::shape_delete_request::de_delete_request(
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -14,7 +14,7 @@
     if builder.import_table_description.is_none() {
         builder.import_table_description = {
             let builder = super::types::builders::ImportTableDescriptionBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
@@ -26,7 +26,7 @@
     if builder.import_table_description.is_none() {
         builder.import_table_description = {
             let builder = super::types::builders::ImportTableDescriptionBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
```

### `src/types/_approximate_creation_date_time_precision.rs`

```diff
--- reference/src/types/_approximate_creation_date_time_precision.rs
+++ generated/src/types/_approximate_creation_date_time_precision.rs
@@ -55,7 +55,9 @@
         match s {
             "MICROSECOND" => ApproximateCreationDateTimePrecision::Microsecond,
             "MILLISECOND" => ApproximateCreationDateTimePrecision::Millisecond,
-            other => ApproximateCreationDateTimePrecision::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
+            other => {
+                ApproximateCreationDateTimePrecision::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned()))
+            }
         }
     }
 }
```

### `src/types/_attribute_value.rs`

```diff
--- reference/src/types/_attribute_value.rs
+++ generated/src/types/_attribute_value.rs
@@ -75,7 +75,7 @@
     pub fn is_bool(&self) -> bool {
         self.as_bool().is_ok()
     }
-    /// Tries to convert the enum instance into [`Bs`](crate::types::AttributeValue::Bs), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`Bs`](crate::types::AttributeValue::Bs), extracting the inner [`Vec::<Blob>`](::std::vec::Vec<::aws_smithy_types::Blob>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_bs(&self) -> ::std::result::Result<&::std::vec::Vec<::aws_smithy_types::Blob>, &Self> {
         if let AttributeValue::Bs(val) = &self {
@@ -88,7 +88,7 @@
     pub fn is_bs(&self) -> bool {
         self.as_bs().is_ok()
     }
-    /// Tries to convert the enum instance into [`L`](crate::types::AttributeValue::L), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`L`](crate::types::AttributeValue::L), extracting the inner [`Vec::<AttributeValue>`](::std::vec::Vec<crate::types::AttributeValue>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_l(&self) -> ::std::result::Result<&::std::vec::Vec<super::super::types::AttributeValue>, &Self> {
         if let AttributeValue::L(val) = &self {
@@ -101,7 +101,7 @@
     pub fn is_l(&self) -> bool {
         self.as_l().is_ok()
     }
-    /// Tries to convert the enum instance into [`M`](crate::types::AttributeValue::M), extracting the inner [`HashMap`](::std::collections::HashMap).
+    /// Tries to convert the enum instance into [`M`](crate::types::AttributeValue::M), extracting the inner [`HashMap::<String, AttributeValue>`](::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_m(&self) -> ::std::result::Result<&::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>, &Self> {
         if let AttributeValue::M(val) = &self {
@@ -127,7 +127,7 @@
     pub fn is_n(&self) -> bool {
         self.as_n().is_ok()
     }
-    /// Tries to convert the enum instance into [`Ns`](crate::types::AttributeValue::Ns), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`Ns`](crate::types::AttributeValue::Ns), extracting the inner [`Vec::<String>`](::std::vec::Vec<::std::string::String>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_ns(&self) -> ::std::result::Result<&::std::vec::Vec<::std::string::String>, &Self> {
         if let AttributeValue::Ns(val) = &self {
@@ -166,7 +166,7 @@
     pub fn is_s(&self) -> bool {
         self.as_s().is_ok()
     }
-    /// Tries to convert the enum instance into [`Ss`](crate::types::AttributeValue::Ss), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`Ss`](crate::types::AttributeValue::Ss), extracting the inner [`Vec::<String>`](::std::vec::Vec<::std::string::String>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_ss(&self) -> ::std::result::Result<&::std::vec::Vec<::std::string::String>, &Self> {
         if let AttributeValue::Ss(val) = &self {
```

### `src/types/_condition_check.rs`

```diff
--- reference/src/types/_condition_check.rs
+++ generated/src/types/_condition_check.rs
@@ -62,7 +62,8 @@
     pub(crate) table_name: ::std::option::Option<::std::string::String>,
     pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
     pub(crate) return_values_on_condition_check_failure: ::std::option::Option<super::super::types::ReturnValuesOnConditionCheckFailure>,
 }
 impl ConditionCheckBuilder {
@@ -78,7 +79,10 @@
         self
     }
     /// <p>The primary key of the item to be checked. Each element consists of an attribute name and a value for that attribute.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    ) -> Self {
         self.key = input;
         self
     }
@@ -150,7 +154,11 @@
     /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
     ///
     /// <p>One or more values that can be substituted in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
```

### `src/types/_consumed_capacity.rs`

```diff
--- reference/src/types/_consumed_capacity.rs
+++ generated/src/types/_consumed_capacity.rs
@@ -43,11 +43,15 @@
         self.table.as_ref()
     }
     /// <p>The amount of throughput consumed on each local index affected by the operation.</p>
-    pub fn local_secondary_indexes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::types::Capacity>> {
+    pub fn local_secondary_indexes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::types::Capacity>> {
         self.local_secondary_indexes.as_ref()
     }
     /// <p>The amount of throughput consumed on each global index affected by the operation.</p>
-    pub fn global_secondary_indexes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::types::Capacity>> {
+    pub fn global_secondary_indexes(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::types::Capacity>> {
         self.global_secondary_indexes.as_ref()
     }
     /// <p>The amount of throughput consumed on each vector index affected by the operation. Each entry contains <code>VectorWriteRequestBytes</code> (for write operations) or <code>VectorSearchRequestBytes</code> (for search operations).</p>
@@ -166,7 +170,9 @@
         self
     }
     /// <p>The amount of throughput consumed on each local index affected by the operation.</p>
-    pub fn get_local_secondary_indexes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::Capacity>> {
+    pub fn get_local_secondary_indexes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::Capacity>> {
         &self.local_secondary_indexes
     }
     /// Adds a key-value pair to `global_secondary_indexes`.
@@ -189,7 +195,9 @@
         self
     }
     /// <p>The amount of throughput consumed on each global index affected by the operation.</p>
-    pub fn get_global_secondary_indexes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::Capacity>> {
+    pub fn get_global_secondary_indexes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::Capacity>> {
         &self.global_secondary_indexes
     }
     /// Adds a key-value pair to `vector_indexes`.
@@ -212,7 +220,9 @@
         self
     }
     /// <p>The amount of throughput consumed on each vector index affected by the operation. Each entry contains <code>VectorWriteRequestBytes</code> (for write operations) or <code>VectorSearchRequestBytes</code> (for search operations).</p>
-    pub fn get_vector_indexes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::VectorCapacity>> {
+    pub fn get_vector_indexes(
+        &self,
+    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::VectorCapacity>> {
         &self.vector_indexes
     }
     /// Consumes the builder and constructs a [`ConsumedCapacity`](crate::types::ConsumedCapacity).
```

### `src/types/_continuous_backups_description.rs`

```diff
--- reference/src/types/_continuous_backups_description.rs
+++ generated/src/types/_continuous_backups_description.rs
@@ -55,7 +55,10 @@
         self
     }
     /// <p>The description of the point in time recovery settings applied to the table.</p>
-    pub fn set_point_in_time_recovery_description(mut self, input: ::std::option::Option<super::super::types::PointInTimeRecoveryDescription>) -> Self {
+    pub fn set_point_in_time_recovery_description(
+        mut self,
+        input: ::std::option::Option<super::super::types::PointInTimeRecoveryDescription>,
+    ) -> Self {
         self.point_in_time_recovery_description = input;
         self
     }
```

### `src/types/_create_global_secondary_index_action.rs`

```diff
--- reference/src/types/_create_global_secondary_index_action.rs
+++ generated/src/types/_create_global_secondary_index_action.rs
@@ -165,7 +165,9 @@
     /// This method will fail if any of the following fields are not set:
     /// - [`index_name`](crate::types::builders::CreateGlobalSecondaryIndexActionBuilder::index_name)
     /// - [`key_schema`](crate::types::builders::CreateGlobalSecondaryIndexActionBuilder::key_schema)
-    pub fn build(self) -> ::std::result::Result<super::super::types::CreateGlobalSecondaryIndexAction, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::CreateGlobalSecondaryIndexAction, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::CreateGlobalSecondaryIndexAction {
             index_name: self.index_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_create_replication_group_member_action.rs`

```diff
--- reference/src/types/_create_replication_group_member_action.rs
+++ generated/src/types/_create_replication_group_member_action.rs
@@ -134,7 +134,10 @@
         self
     }
     /// <p>Replica-specific global secondary index settings.</p>
-    pub fn set_global_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndex>>) -> Self {
+    pub fn set_global_secondary_indexes(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndex>>,
+    ) -> Self {
         self.global_secondary_indexes = input;
         self
     }
@@ -159,7 +162,9 @@
     /// Consumes the builder and constructs a [`CreateReplicationGroupMemberAction`](crate::types::CreateReplicationGroupMemberAction).
     /// This method will fail if any of the following fields are not set:
     /// - [`region_name`](crate::types::builders::CreateReplicationGroupMemberActionBuilder::region_name)
-    pub fn build(self) -> ::std::result::Result<super::super::types::CreateReplicationGroupMemberAction, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::CreateReplicationGroupMemberAction, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::CreateReplicationGroupMemberAction {
             region_name: self.region_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_delete.rs`

```diff
--- reference/src/types/_delete.rs
+++ generated/src/types/_delete.rs
@@ -61,7 +61,8 @@
     pub(crate) table_name: ::std::option::Option<::std::string::String>,
     pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
     pub(crate) return_values_on_condition_check_failure: ::std::option::Option<super::super::types::ReturnValuesOnConditionCheckFailure>,
 }
 impl DeleteBuilder {
@@ -77,7 +78,10 @@
         self
     }
     /// <p>The primary key of the item to be deleted. Each element consists of an attribute name and a value for that attribute.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    ) -> Self {
         self.key = input;
         self
     }
@@ -148,7 +152,11 @@
     /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
     ///
     /// <p>One or more values that can be substituted in an expression.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
```

### `src/types/_delete_global_secondary_index_action.rs`

```diff
--- reference/src/types/_delete_global_secondary_index_action.rs
+++ generated/src/types/_delete_global_secondary_index_action.rs
@@ -46,7 +46,9 @@
     /// Consumes the builder and constructs a [`DeleteGlobalSecondaryIndexAction`](crate::types::DeleteGlobalSecondaryIndexAction).
     /// This method will fail if any of the following fields are not set:
     /// - [`index_name`](crate::types::builders::DeleteGlobalSecondaryIndexActionBuilder::index_name)
-    pub fn build(self) -> ::std::result::Result<super::super::types::DeleteGlobalSecondaryIndexAction, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::DeleteGlobalSecondaryIndexAction, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::DeleteGlobalSecondaryIndexAction {
             index_name: self.index_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_delete_replication_group_member_action.rs`

```diff
--- reference/src/types/_delete_replication_group_member_action.rs
+++ generated/src/types/_delete_replication_group_member_action.rs
@@ -46,7 +46,9 @@
     /// Consumes the builder and constructs a [`DeleteReplicationGroupMemberAction`](crate::types::DeleteReplicationGroupMemberAction).
     /// This method will fail if any of the following fields are not set:
     /// - [`region_name`](crate::types::builders::DeleteReplicationGroupMemberActionBuilder::region_name)
-    pub fn build(self) -> ::std::result::Result<super::super::types::DeleteReplicationGroupMemberAction, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::DeleteReplicationGroupMemberAction, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::DeleteReplicationGroupMemberAction {
             region_name: self.region_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_delete_request.rs`

```diff
--- reference/src/types/_delete_request.rs
+++ generated/src/types/_delete_request.rs
@@ -39,7 +39,10 @@
         self
     }
     /// <p>A map of attribute name to attribute values, representing the primary key of the item to delete. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    ) -> Self {
         self.key = input;
         self
     }
```

### `src/types/_get.rs`

```diff
--- reference/src/types/_get.rs
+++ generated/src/types/_get.rs
@@ -61,7 +61,10 @@
         self
     }
     /// <p>A map of attribute names to <code>AttributeValue</code> objects that specifies the primary key of the item to retrieve.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    ) -> Self {
         self.key = input;
         self
     }
```

### `src/types/_global_table_global_secondary_index_settings_update.rs`

```diff
--- reference/src/types/_global_table_global_secondary_index_settings_update.rs
+++ generated/src/types/_global_table_global_secondary_index_settings_update.rs
@@ -85,7 +85,9 @@
         self
     }
     /// <p>Auto scaling settings for managing a global secondary index's write capacity units.</p>
-    pub fn get_provisioned_write_capacity_auto_scaling_settings_update(&self) -> &::std::option::Option<super::super::types::AutoScalingSettingsUpdate> {
+    pub fn get_provisioned_write_capacity_auto_scaling_settings_update(
+        &self,
+    ) -> &::std::option::Option<super::super::types::AutoScalingSettingsUpdate> {
         &self.provisioned_write_capacity_auto_scaling_settings_update
     }
     /// Consumes the builder and constructs a [`GlobalTableGlobalSecondaryIndexSettingsUpdate`](crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate).
@@ -93,7 +95,8 @@
     /// - [`index_name`](crate::types::builders::GlobalTableGlobalSecondaryIndexSettingsUpdateBuilder::index_name)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::types::GlobalTableGlobalSecondaryIndexSettingsUpdate {
             index_name: self.index_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_global_table_settings_replication_mode.rs`

```diff
--- reference/src/types/_global_table_settings_replication_mode.rs
+++ generated/src/types/_global_table_settings_replication_mode.rs
@@ -59,7 +59,9 @@
             "DISABLED" => GlobalTableSettingsReplicationMode::Disabled,
             "ENABLED" => GlobalTableSettingsReplicationMode::Enabled,
             "ENABLED_WITH_OVERRIDES" => GlobalTableSettingsReplicationMode::EnabledWithOverrides,
-            other => GlobalTableSettingsReplicationMode::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
+            other => {
+                GlobalTableSettingsReplicationMode::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned()))
+            }
         }
     }
 }
```

### `src/types/_item_collection_metrics.rs`

```diff
--- reference/src/types/_item_collection_metrics.rs
+++ generated/src/types/_item_collection_metrics.rs
@@ -12,7 +12,9 @@
 }
 impl ItemCollectionMetrics {
     /// <p>The partition key value of the item collection. This value is the same as the partition key value of the item.</p>
-    pub fn item_collection_key(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>> {
+    pub fn item_collection_key(
+        &self,
+    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>> {
         self.item_collection_key.as_ref()
     }
     /// <p>An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p>
```

### `src/types/_point_in_time_recovery_specification.rs`

```diff
--- reference/src/types/_point_in_time_recovery_specification.rs
+++ generated/src/types/_point_in_time_recovery_specification.rs
@@ -66,7 +66,9 @@
     /// Consumes the builder and constructs a [`PointInTimeRecoverySpecification`](crate::types::PointInTimeRecoverySpecification).
     /// This method will fail if any of the following fields are not set:
     /// - [`point_in_time_recovery_enabled`](crate::types::builders::PointInTimeRecoverySpecificationBuilder::point_in_time_recovery_enabled)
-    pub fn build(self) -> ::std::result::Result<super::super::types::PointInTimeRecoverySpecification, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::PointInTimeRecoverySpecification, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::PointInTimeRecoverySpecification {
             point_in_time_recovery_enabled: self.point_in_time_recovery_enabled.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_put.rs`

```diff
--- reference/src/types/_put.rs
+++ generated/src/types/_put.rs
@@ -61,7 +61,8 @@
     pub(crate) table_name: ::std::option::Option<::std::string::String>,
     pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
     pub(crate) return_values_on_condition_check_failure: ::std::option::Option<super::super::types::ReturnValuesOnConditionCheckFailure>,
 }
 impl PutBuilder {
@@ -151,7 +152,11 @@
     /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
     ///
     /// <p>One or more values that can be substituted in an expression.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
```

### `src/types/_replica_auto_scaling_description.rs`

```diff
--- reference/src/types/_replica_auto_scaling_description.rs
+++ generated/src/types/_replica_auto_scaling_description.rs
@@ -37,11 +37,15 @@
         self.global_secondary_indexes.as_deref().unwrap_or_default()
     }
     /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
-    pub fn replica_provisioned_read_capacity_auto_scaling_settings(&self) -> ::std::option::Option<&super::super::types::AutoScalingSettingsDescription> {
+    pub fn replica_provisioned_read_capacity_auto_scaling_settings(
+        &self,
+    ) -> ::std::option::Option<&super::super::types::AutoScalingSettingsDescription> {
         self.replica_provisioned_read_capacity_auto_scaling_settings.as_ref()
     }
     /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
-    pub fn replica_provisioned_write_capacity_auto_scaling_settings(&self) -> ::std::option::Option<&super::super::types::AutoScalingSettingsDescription> {
+    pub fn replica_provisioned_write_capacity_auto_scaling_settings(
+        &self,
+    ) -> ::std::option::Option<&super::super::types::AutoScalingSettingsDescription> {
         self.replica_provisioned_write_capacity_auto_scaling_settings.as_ref()
     }
     /// <p>The current state of the replica:</p>
@@ -71,7 +75,8 @@
 #[non_exhaustive]
 pub struct ReplicaAutoScalingDescriptionBuilder {
     pub(crate) region_name: ::std::option::Option<::std::string::String>,
-    pub(crate) global_secondary_indexes: ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndexAutoScalingDescription>>,
+    pub(crate) global_secondary_indexes:
+        ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndexAutoScalingDescription>>,
     pub(crate) replica_provisioned_read_capacity_auto_scaling_settings: ::std::option::Option<super::super::types::AutoScalingSettingsDescription>,
     pub(crate) replica_provisioned_write_capacity_auto_scaling_settings: ::std::option::Option<super::super::types::AutoScalingSettingsDescription>,
     pub(crate) replica_status: ::std::option::Option<super::super::types::ReplicaStatus>,
```

### `src/types/_replica_auto_scaling_update.rs`

```diff
--- reference/src/types/_replica_auto_scaling_update.rs
+++ generated/src/types/_replica_auto_scaling_update.rs
@@ -7,7 +7,8 @@
     /// <p>The Region where the replica exists.</p>
     pub region_name: ::std::string::String,
     /// <p>Represents the auto scaling settings of global secondary indexes that will be modified.</p>
-    pub replica_global_secondary_index_updates: ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndexAutoScalingUpdate>>,
+    pub replica_global_secondary_index_updates:
+        ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndexAutoScalingUpdate>>,
     /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
     pub replica_provisioned_read_capacity_auto_scaling_update: ::std::option::Option<super::super::types::AutoScalingSettingsUpdate>,
 }
@@ -99,7 +100,9 @@
         self
     }
     /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
-    pub fn get_replica_provisioned_read_capacity_auto_scaling_update(&self) -> &::std::option::Option<super::super::types::AutoScalingSettingsUpdate> {
+    pub fn get_replica_provisioned_read_capacity_auto_scaling_update(
+        &self,
+    ) -> &::std::option::Option<super::super::types::AutoScalingSettingsUpdate> {
         &self.replica_provisioned_read_capacity_auto_scaling_update
     }
     /// Consumes the builder and constructs a [`ReplicaAutoScalingUpdate`](crate::types::ReplicaAutoScalingUpdate).
```

### `src/types/_replica_description.rs`

```diff
--- reference/src/types/_replica_description.rs
+++ generated/src/types/_replica_description.rs
@@ -365,7 +365,9 @@
         self
     }
     /// <p>Replica-specific global secondary index settings.</p>
-    pub fn get_global_secondary_indexes(&self) -> &::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndexDescription>> {
+    pub fn get_global_secondary_indexes(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndexDescription>> {
         &self.global_secondary_indexes
     }
     /// <p>The time at which the replica was first detected as inaccessible. To determine cause of inaccessibility check the <code>ReplicaStatus</code> property.</p>
```

### `src/types/_replica_global_secondary_index_auto_scaling_description.rs`

```diff
--- reference/src/types/_replica_global_secondary_index_auto_scaling_description.rs
+++ generated/src/types/_replica_global_secondary_index_auto_scaling_description.rs
@@ -157,7 +157,9 @@
         self
     }
     /// <p>Represents the auto scaling settings for a global table or global secondary index.</p>
-    pub fn get_provisioned_write_capacity_auto_scaling_settings(&self) -> &::std::option::Option<super::super::types::AutoScalingSettingsDescription> {
+    pub fn get_provisioned_write_capacity_auto_scaling_settings(
+        &self,
+    ) -> &::std::option::Option<super::super::types::AutoScalingSettingsDescription> {
         &self.provisioned_write_capacity_auto_scaling_settings
     }
     /// Consumes the builder and constructs a [`ReplicaGlobalSecondaryIndexAutoScalingDescription`](crate::types::ReplicaGlobalSecondaryIndexAutoScalingDescription).
```

### `src/types/_replica_global_secondary_index_settings_description.rs`

```diff
--- reference/src/types/_replica_global_secondary_index_settings_description.rs
+++ generated/src/types/_replica_global_secondary_index_settings_description.rs
@@ -201,7 +201,9 @@
         self
     }
     /// <p>Auto scaling settings for a global secondary index replica's write capacity units.</p>
-    pub fn get_provisioned_write_capacity_auto_scaling_settings(&self) -> &::std::option::Option<super::super::types::AutoScalingSettingsDescription> {
+    pub fn get_provisioned_write_capacity_auto_scaling_settings(
+        &self,
+    ) -> &::std::option::Option<super::super::types::AutoScalingSettingsDescription> {
         &self.provisioned_write_capacity_auto_scaling_settings
     }
     /// Consumes the builder and constructs a [`ReplicaGlobalSecondaryIndexSettingsDescription`](crate::types::ReplicaGlobalSecondaryIndexSettingsDescription).
@@ -209,7 +211,8 @@
     /// - [`index_name`](crate::types::builders::ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder::index_name)
     pub fn build(
         self,
-    ) -> ::std::result::Result<super::super::types::ReplicaGlobalSecondaryIndexSettingsDescription, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<super::super::types::ReplicaGlobalSecondaryIndexSettingsDescription, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(super::super::types::ReplicaGlobalSecondaryIndexSettingsDescription {
             index_name: self.index_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_replica_global_secondary_index_settings_update.rs`

```diff
--- reference/src/types/_replica_global_secondary_index_settings_update.rs
+++ generated/src/types/_replica_global_secondary_index_settings_update.rs
@@ -85,7 +85,9 @@
         self
     }
     /// <p>Auto scaling settings for managing a global secondary index replica's read capacity units.</p>
-    pub fn get_provisioned_read_capacity_auto_scaling_settings_update(&self) -> &::std::option::Option<super::super::types::AutoScalingSettingsUpdate> {
+    pub fn get_provisioned_read_capacity_auto_scaling_settings_update(
+        &self,
+    ) -> &::std::option::Option<super::super::types::AutoScalingSettingsUpdate> {
         &self.provisioned_read_capacity_auto_scaling_settings_update
     }
     /// Consumes the builder and constructs a [`ReplicaGlobalSecondaryIndexSettingsUpdate`](crate::types::ReplicaGlobalSecondaryIndexSettingsUpdate).
```

### `src/types/_replica_settings_description.rs`

```diff
--- reference/src/types/_replica_settings_description.rs
+++ generated/src/types/_replica_settings_description.rs
@@ -29,7 +29,8 @@
     /// <p>Auto scaling settings for a global table replica's write capacity units.</p>
     pub replica_provisioned_write_capacity_auto_scaling_settings: ::std::option::Option<super::super::types::AutoScalingSettingsDescription>,
     /// <p>Replica global secondary index settings for the global table.</p>
-    pub replica_global_secondary_index_settings: ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndexSettingsDescription>>,
+    pub replica_global_secondary_index_settings:
+        ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndexSettingsDescription>>,
     /// <p>Contains details of the table class.</p>
     pub replica_table_class_summary: ::std::option::Option<super::super::types::TableClassSummary>,
 }
@@ -62,7 +63,9 @@
         self.replica_provisioned_read_capacity_units
     }
     /// <p>Auto scaling settings for a global table replica's read capacity units.</p>
-    pub fn replica_provisioned_read_capacity_auto_scaling_settings(&self) -> ::std::option::Option<&super::super::types::AutoScalingSettingsDescription> {
+    pub fn replica_provisioned_read_capacity_auto_scaling_settings(
+        &self,
+    ) -> ::std::option::Option<&super::super::types::AutoScalingSettingsDescription> {
         self.replica_provisioned_read_capacity_auto_scaling_settings.as_ref()
     }
     /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
@@ -70,7 +73,9 @@
         self.replica_provisioned_write_capacity_units
     }
     /// <p>Auto scaling settings for a global table replica's write capacity units.</p>
-    pub fn replica_provisioned_write_capacity_auto_scaling_settings(&self) -> ::std::option::Option<&super::super::types::AutoScalingSettingsDescription> {
+    pub fn replica_provisioned_write_capacity_auto_scaling_settings(
+        &self,
+    ) -> ::std::option::Option<&super::super::types::AutoScalingSettingsDescription> {
         self.replica_provisioned_write_capacity_auto_scaling_settings.as_ref()
     }
     /// <p>Replica global secondary index settings for the global table.</p>
```

### `src/types/_replica_settings_update.rs`

```diff
--- reference/src/types/_replica_settings_update.rs
+++ generated/src/types/_replica_settings_update.rs
@@ -27,7 +27,9 @@
         self.replica_provisioned_read_capacity_units
     }
     /// <p>Auto scaling settings for managing a global table replica's read capacity units.</p>
-    pub fn replica_provisioned_read_capacity_auto_scaling_settings_update(&self) -> ::std::option::Option<&super::super::types::AutoScalingSettingsUpdate> {
+    pub fn replica_provisioned_read_capacity_auto_scaling_settings_update(
+        &self,
+    ) -> ::std::option::Option<&super::super::types::AutoScalingSettingsUpdate> {
         self.replica_provisioned_read_capacity_auto_scaling_settings_update.as_ref()
     }
     /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
```

### `src/types/_return_values_on_condition_check_failure.rs`

```diff
--- reference/src/types/_return_values_on_condition_check_failure.rs
+++ generated/src/types/_return_values_on_condition_check_failure.rs
@@ -55,7 +55,9 @@
         match s {
             "ALL_OLD" => ReturnValuesOnConditionCheckFailure::AllOld,
             "NONE" => ReturnValuesOnConditionCheckFailure::None,
-            other => ReturnValuesOnConditionCheckFailure::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
+            other => {
+                ReturnValuesOnConditionCheckFailure::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned()))
+            }
         }
     }
 }
```

### `src/types/_source_table_feature_details.rs`

```diff
--- reference/src/types/_source_table_feature_details.rs
+++ generated/src/types/_source_table_feature_details.rs
@@ -80,7 +80,10 @@
         self
     }
     /// <p>Represents the LSI properties for the table when the backup was created. It includes the IndexName, KeySchema and Projection for the LSIs on the table at the time of backup.</p>
-    pub fn set_local_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::LocalSecondaryIndexInfo>>) -> Self {
+    pub fn set_local_secondary_indexes(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::types::LocalSecondaryIndexInfo>>,
+    ) -> Self {
         self.local_secondary_indexes = input;
         self
     }
@@ -100,7 +103,10 @@
         self
     }
     /// <p>Represents the GSI properties for the table when the backup was created. It includes the IndexName, KeySchema, Projection, and ProvisionedThroughput for the GSIs on the table at the time of backup.</p>
-    pub fn set_global_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::GlobalSecondaryIndexInfo>>) -> Self {
+    pub fn set_global_secondary_indexes(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::types::GlobalSecondaryIndexInfo>>,
+    ) -> Self {
         self.global_secondary_indexes = input;
         self
     }
```

### `src/types/_table_description.rs`

```diff
--- reference/src/types/_table_description.rs
+++ generated/src/types/_table_description.rs
@@ -1210,7 +1210,10 @@
         self
     }
     /// <p>The witness Region and its current status in the MRSC global table. Only one witness Region can be configured per MRSC global table.</p>
-    pub fn set_global_table_witnesses(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::GlobalTableWitnessDescription>>) -> Self {
+    pub fn set_global_table_witnesses(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::types::GlobalTableWitnessDescription>>,
+    ) -> Self {
         self.global_table_witnesses = input;
         self
     }
```

### `src/types/_update.rs`

```diff
--- reference/src/types/_update.rs
+++ generated/src/types/_update.rs
@@ -69,7 +69,8 @@
     pub(crate) table_name: ::std::option::Option<::std::string::String>,
     pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
     pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
-    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    pub(crate) expression_attribute_values:
+        ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
     pub(crate) return_values_on_condition_check_failure: ::std::option::Option<super::super::types::ReturnValuesOnConditionCheckFailure>,
 }
 impl UpdateBuilder {
@@ -85,7 +86,10 @@
         self
     }
     /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p>
-    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>) -> Self {
+    pub fn set_key(
+        mut self,
+        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, super::super::types::AttributeValue>>,
+    ) -> Self {
         self.key = input;
         self
     }
@@ -171,7 +175,11 @@
     /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
     ///
     /// <p>One or more values that can be substituted in an expression.</p>
-    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: super::super::types::AttributeValue) -> Self {
+    pub fn expression_attribute_values(
+        mut self,
+        k: impl ::std::convert::Into<::std::string::String>,
+        v: super::super::types::AttributeValue,
+    ) -> Self {
         let mut hash_map = self.expression_attribute_values.unwrap_or_default();
         hash_map.insert(k.into(), v);
         self.expression_attribute_values = ::std::option::Option::Some(hash_map);
```

### `src/types/_update_global_secondary_index_action.rs`

```diff
--- reference/src/types/_update_global_secondary_index_action.rs
+++ generated/src/types/_update_global_secondary_index_action.rs
@@ -114,7 +114,9 @@
     /// Consumes the builder and constructs a [`UpdateGlobalSecondaryIndexAction`](crate::types::UpdateGlobalSecondaryIndexAction).
     /// This method will fail if any of the following fields are not set:
     /// - [`index_name`](crate::types::builders::UpdateGlobalSecondaryIndexActionBuilder::index_name)
-    pub fn build(self) -> ::std::result::Result<super::super::types::UpdateGlobalSecondaryIndexAction, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::UpdateGlobalSecondaryIndexAction, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::UpdateGlobalSecondaryIndexAction {
             index_name: self.index_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_update_replication_group_member_action.rs`

```diff
--- reference/src/types/_update_replication_group_member_action.rs
+++ generated/src/types/_update_replication_group_member_action.rs
@@ -134,7 +134,10 @@
         self
     }
     /// <p>Replica-specific global secondary index settings.</p>
-    pub fn set_global_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndex>>) -> Self {
+    pub fn set_global_secondary_indexes(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<super::super::types::ReplicaGlobalSecondaryIndex>>,
+    ) -> Self {
         self.global_secondary_indexes = input;
         self
     }
@@ -159,7 +162,9 @@
     /// Consumes the builder and constructs a [`UpdateReplicationGroupMemberAction`](crate::types::UpdateReplicationGroupMemberAction).
     /// This method will fail if any of the following fields are not set:
     /// - [`region_name`](crate::types::builders::UpdateReplicationGroupMemberActionBuilder::region_name)
-    pub fn build(self) -> ::std::result::Result<super::super::types::UpdateReplicationGroupMemberAction, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::UpdateReplicationGroupMemberAction, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::UpdateReplicationGroupMemberAction {
             region_name: self.region_name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/error/_replicated_write_conflict_exception.rs`

```diff
--- reference/src/types/error/_replicated_write_conflict_exception.rs
+++ generated/src/types/error/_replicated_write_conflict_exception.rs
@@ -9,10 +9,6 @@
     pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
 }
 impl ReplicatedWriteConflictException {
-    /// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
-    pub fn retryable_error_kind(&self) -> ::aws_smithy_types::retry::ErrorKind {
-        ::aws_smithy_types::retry::ErrorKind::ClientError
-    }
     /// Returns the error message.
     pub fn message(&self) -> ::std::option::Option<&str> {
         self.message.as_deref()
```

### `src/waiters/kinesis_streaming_destination_active.rs`

```diff
--- reference/src/waiters/kinesis_streaming_destination_active.rs
+++ generated/src/waiters/kinesis_streaming_destination_active.rs
@@ -25,7 +25,9 @@
         }
     }
     /// Access the DescribeKinesisStreamingDestination as a reference.
-    pub fn as_input(&self) -> &super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &super::super::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder {
         &self.inner
     }
     /// Wait for `kinesis_streaming_destination_active`
@@ -73,8 +75,11 @@
             let input = input.clone();
             let runtime_plugins = runtime_plugins.clone();
             async move {
-                super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestination::orchestrate(&runtime_plugins, input)
-                    .await
+                super::super::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestination::orchestrate(
+                    &runtime_plugins,
+                    input,
+                )
+                .await
             }
         };
         let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
```

### `src/waiters/matchers.rs`

```diff
--- reference/src/waiters/matchers.rs
+++ generated/src/waiters/matchers.rs
@@ -54,7 +54,10 @@

 /// Matcher union: {"output":{"path":"ExportDescription.ExportStatus","expected":"COMPLETED","comparator":"stringEquals"}}
 pub(crate) fn match_describe_export_a4219f96bc64309c4(
-    _result: ::std::result::Result<&super::super::operation::describe_export::DescribeExportOutput, &super::super::operation::describe_export::DescribeExportError>,
+    _result: ::std::result::Result<
+        &super::super::operation::describe_export::DescribeExportOutput,
+        &super::super::operation::describe_export::DescribeExportError,
+    >,
 ) -> bool {
     fn path_traversal<'a>(
         _output: &'a super::super::operation::describe_export::DescribeExportOutput,
@@ -78,7 +81,10 @@

 /// Matcher union: {"output":{"path":"ExportDescription.ExportStatus","expected":"FAILED","comparator":"stringEquals"}}
 pub(crate) fn match_describe_export_8d899fd0681b891e9(
-    _result: ::std::result::Result<&super::super::operation::describe_export::DescribeExportOutput, &super::super::operation::describe_export::DescribeExportError>,
+    _result: ::std::result::Result<
+        &super::super::operation::describe_export::DescribeExportOutput,
+        &super::super::operation::describe_export::DescribeExportError,
+    >,
 ) -> bool {
     fn path_traversal<'a>(
         _output: &'a super::super::operation::describe_export::DescribeExportOutput,
@@ -102,7 +108,10 @@

 /// Matcher union: {"output":{"path":"ImportTableDescription.ImportStatus","expected":"COMPLETED","comparator":"stringEquals"}}
 pub(crate) fn match_describe_import_6175a829c57972dc3(
-    _result: ::std::result::Result<&super::super::operation::describe_import::DescribeImportOutput, &super::super::operation::describe_import::DescribeImportError>,
+    _result: ::std::result::Result<
+        &super::super::operation::describe_import::DescribeImportOutput,
+        &super::super::operation::describe_import::DescribeImportError,
+    >,
 ) -> bool {
     fn path_traversal<'a>(
         _output: &'a super::super::operation::describe_import::DescribeImportOutput,
@@ -126,7 +135,10 @@

 /// Matcher union: {"output":{"path":"ImportTableDescription.ImportStatus","expected":"FAILED","comparator":"stringEquals"}}
 pub(crate) fn match_describe_import_ba4301a7fe05a6956(
-    _result: ::std::result::Result<&super::super::operation::describe_import::DescribeImportOutput, &super::super::operation::describe_import::DescribeImportError>,
+    _result: ::std::result::Result<
+        &super::super::operation::describe_import::DescribeImportOutput,
+        &super::super::operation::describe_import::DescribeImportError,
+    >,
 ) -> bool {
     fn path_traversal<'a>(
         _output: &'a super::super::operation::describe_import::DescribeImportOutput,
@@ -150,7 +162,10 @@

 /// Matcher union: {"output":{"path":"ImportTableDescription.ImportStatus","expected":"CANCELLED","comparator":"stringEquals"}}
 pub(crate) fn match_describe_import_b4c5c86bd60d42041(
-    _result: ::std::result::Result<&super::super::operation::describe_import::DescribeImportOutput, &super::super::operation::describe_import::DescribeImportError>,
+    _result: ::std::result::Result<
+        &super::super::operation::describe_import::DescribeImportOutput,
+        &super::super::operation::describe_import::DescribeImportError,
+    >,
 ) -> bool {
     fn path_traversal<'a>(
         _output: &'a super::super::operation::describe_import::DescribeImportOutput,
@@ -267,7 +282,10 @@

 /// Matcher union: {"output":{"path":"Table.TableStatus","expected":"ACTIVE","comparator":"stringEquals"}}
 pub(crate) fn match_describe_table_0429b99996ae6dab6(
-    _result: ::std::result::Result<&super::super::operation::describe_table::DescribeTableOutput, &super::super::operation::describe_table::DescribeTableError>,
+    _result: ::std::result::Result<
+        &super::super::operation::describe_table::DescribeTableOutput,
+        &super::super::operation::describe_table::DescribeTableError,
+    >,
 ) -> bool {
     fn path_traversal<'a>(
         _output: &'a super::super::operation::describe_table::DescribeTableOutput,
@@ -291,7 +309,10 @@

 /// Matcher union: {"errorType":"ResourceNotFoundException"}
 pub(crate) fn match_describe_table_1cce2c05524fb92d4(
-    _result: ::std::result::Result<&super::super::operation::describe_table::DescribeTableOutput, &super::super::operation::describe_table::DescribeTableError>,
+    _result: ::std::result::Result<
+        &super::super::operation::describe_table::DescribeTableOutput,
+        &super::super::operation::describe_table::DescribeTableError,
+    >,
 ) -> bool {
     if let ::std::result::Result::Err(err) = _result {
         if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
```

### `src/waiters/table_exists.rs`

```diff
--- reference/src/waiters/table_exists.rs
+++ generated/src/waiters/table_exists.rs
@@ -32,7 +32,10 @@
     pub async fn wait(
         self,
         max_wait: ::std::time::Duration,
-    ) -> ::std::result::Result<super::super::waiters::table_exists::TableExistsFinalPoll, super::super::waiters::table_exists::WaitUntilTableExistsError> {
+    ) -> ::std::result::Result<
+        super::super::waiters::table_exists::TableExistsFinalPoll,
+        super::super::waiters::table_exists::WaitUntilTableExistsError,
+    > {
         let input = self
             .inner
             .build()
```
