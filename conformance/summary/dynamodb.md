# AWS SDK Conformance Report: dynamodb

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## dynamodb
**Progress:** `903/903` files compared · `451` matched · `82` mismatches · `370` missing · `0` extra · `49.94%` match (100.00% means fully matched)

### `src/client/batch_get_item.rs`

```diff
--- reference/src/client/batch_get_item.rs
+++ generated/src/client/batch_get_item.rs
@@ -3,7 +3,7 @@
     /// Constructs a fluent builder for the [`BatchGetItem`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder) operation.
     ///
     /// - The fluent builder is configurable:
-    ///   - [`request_items(impl Into<String>, KeysAndAttributes)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::request_items) / [`set_request_items(Option<HashMap::<String, KeysAndAttributes>>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_request_items):<br>required: **true**<br><p>A map of one or more table names or table ARNs and, for each table, a map that describes one or more items to retrieve from that table. Each table name or ARN can be used only once per <code>BatchGetItem</code> request.</p> <p>Each element in the map of items to retrieve consists of the following:</p> <ul>  <li>   <p><code>ConsistentRead</code> - If <code>true</code>, a strongly consistent read is used; if <code>false</code> (the default), an eventually consistent read is used.</p></li>  <li>   <p><code>ExpressionAttributeNames</code> - One or more substitution tokens for attribute names in the <code>ProjectionExpression</code> parameter. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>   <ul>    <li>     <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>    <li>     <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>    <li>     <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>   </ul>   <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>   <ul>    <li>     <p><code>Percentile</code></p></li>   </ul>   <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>   <ul>    <li>     <p><code>{"#P":"Percentile"}</code></p></li>   </ul>   <p>You could then use this substitution in an expression, as in this example:</p>   <ul>    <li>     <p><code>#P = :val</code></p></li>   </ul><note>    <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>   </note>   <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>  <li>   <p><code>Keys</code> - An array of primary key attribute values that define specific items in the table. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key value. For a composite key, you must provide <i>both</i> the partition key value and the sort key value.</p></li>  <li>   <p><code>ProjectionExpression</code> - A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>   <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p>   <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>  <li>   <p><code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li> </ul><br>
+    ///   - [`request_items(impl Into<String>, KeysAndAttributes)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::request_items) / [`set_request_items(Option<HashMap::<String, KeysAndAttributes>>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_request_items):<br>required: **true**<br><p>A map of one or more table names or table ARNs and, for each table, a map that describes one or more items to retrieve from that table. Each table name or ARN can be used only once per <code>BatchGetItem</code> request.</p> <p>Each element in the map of items to retrieve consists of the following:</p> <ul>  <li>   <p><code>ConsistentRead</code> - If <code>true</code>, a strongly consistent read is used; if <code>false</code> (the default), an eventually consistent read is used.</p></li>  <li>   <p><code>ExpressionAttributeNames</code> - One or more substitution tokens for attribute names in the <code>ProjectionExpression</code> parameter. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>   <ul>    <li>     <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>    <li>     <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>    <li>     <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>   </ul>   <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>   <ul>    <li>     <p><code>Percentile</code></p></li>   </ul>   <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>   <ul>    <li>     <p><code>{"#P":"Percentile"}</code></p></li>   </ul>   <p>You could then use this substitution in an expression, as in this example:</p>   <ul>    <li>     <p><code>#P = :val</code></p></li>   </ul> <note>    <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note>   <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>  <li>   <p><code>Keys</code> - An array of primary key attribute values that define specific items in the table. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key value. For a composite key, you must provide <i>both</i> the partition key value and the sort key value.</p></li>  <li>   <p><code>ProjectionExpression</code> - A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>   <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p>   <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li>  <li>   <p><code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p></li> </ul><br>
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
+    ///   - [`key_schema(KeySchemaElement)`](crate::operation::create_table::builders::CreateTableFluentBuilder::key_schema) / [`set_key_schema(Option<Vec::<KeySchemaElement>>)`](crate::operation::create_table::builders::CreateTableFluentBuilder::set_key_schema):<br>required: **false**<br><p>Specifies the attributes that make up the primary key for a table or an index. The attributes in <code>KeySchema</code> must also be defined in the <code>AttributeDefinitions</code> array. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html">Data Model</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Each <code>KeySchemaElement</code> in the array is composed of:</p> <ul>  <li>   <p><code>AttributeName</code> - The name of this key attribute.</p></li>  <li>   <p><code>KeyType</code> - The role that the key attribute will assume:</p>   <ul>    <li>     <p><code>HASH</code> - partition key</p></li>    <li>     <p><code>RANGE</code> - sort key</p></li>   </ul></li> </ul> <note>  <p>The partition key of an item is also known as its <i>hash attribute</i>. The term "hash attribute" derives from the DynamoDB usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term "range attribute" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note> <p>For a simple primary key (partition key), you must provide exactly one element with a <code>KeyType</code> of <code>HASH</code>.</p> <p>For a composite primary key (partition key and sort key), you must provide exactly two elements, in this order: The first element must have a <code>KeyType</code> of <code>HASH</code>, and the second element must have a <code>KeyType</code> of <code>RANGE</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTables.primary.key">Working with Tables</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
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
@@ -19,12 +19,12 @@
     ///   - [`projection_expression(impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::projection_expression) / [`set_projection_expression(Option<String>)`](crate::operation::query::builders::QueryFluentBuilder::set_projection_expression):<br>required: **false**<br><p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`filter_expression(impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::filter_expression) / [`set_filter_expression(Option<String>)`](crate::operation::query::builders::QueryFluentBuilder::set_filter_expression):<br>required: **false**<br><p>A string that contains conditions that DynamoDB applies after the <code>Query</code> operation, but before the data is returned to you. Items that do not satisfy the <code>FilterExpression</code> criteria are not returned.</p> <p>A <code>FilterExpression</code> does not allow key attributes. You cannot define a filter expression based on a partition key or a sort key.</p><note>  <p>A <code>FilterExpression</code> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.FilterExpression.html">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`key_condition_expression(impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::key_condition_expression) / [`set_key_condition_expression(Option<String>)`](crate::operation::query::builders::QueryFluentBuilder::set_key_condition_expression):<br>required: **false**<br><p>The condition that specifies the key values for items to be retrieved by the <code>Query</code> action.</p> <p>The condition must perform an equality test on a single partition key value.</p> <p>The condition can optionally perform one of several comparison tests on a single sort key value. This allows <code>Query</code> to retrieve one item with a given partition key value and sort key value, or several items that have the same partition key value but different sort key values.</p> <p>The partition key equality test is required, and must be specified in the following format:</p> <p><code>partitionKeyName</code> <i>=</i> <code>:partitionkeyval</code></p> <p>If you also want to provide a condition for the sort key, it must be combined using <code>AND</code> with the condition for the sort key. Following is an example, using the <b>=</b> comparison operator for the sort key:</p> <p><code>partitionKeyName</code> <code>=</code> <code>:partitionkeyval</code> <code>AND</code> <code>sortKeyName</code> <code>=</code> <code>:sortkeyval</code></p> <p>Valid comparisons for the sort key condition are as follows:</p> <ul>  <li>   <p><code>sortKeyName</code> <code>=</code> <code>:sortkeyval</code> - true if the sort key value is equal to <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>&lt;</code> <code>:sortkeyval</code> - true if the sort key value is less than <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>&lt;=</code> <code>:sortkeyval</code> - true if the sort key value is less than or equal to <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>&gt;</code> <code>:sortkeyval</code> - true if the sort key value is greater than <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>&gt;= </code> <code>:sortkeyval</code> - true if the sort key value is greater than or equal to <code>:sortkeyval</code>.</p></li>  <li>   <p><code>sortKeyName</code> <code>BETWEEN</code> <code>:sortkeyval1</code> <code>AND</code> <code>:sortkeyval2</code> - true if the sort key value is greater than or equal to <code>:sortkeyval1</code>, and less than or equal to <code>:sortkeyval2</code>.</p></li>  <li>   <p><code>begins_with (</code> <code>sortKeyName</code>, <code>:sortkeyval</code> <code>)</code> - true if the sort key value begins with a particular operand. (You cannot use this function with a sort key that is of type Number.) Note that the function name <code>begins_with</code> is case-sensitive.</p></li> </ul> <p>Use the <code>ExpressionAttributeValues</code> parameter to replace tokens such as <code>:partitionval</code> and <code>:sortval</code> with actual values at runtime.</p> <p>You can optionally use the <code>ExpressionAttributeNames</code> parameter to replace the names of the partition key and sort key with placeholder tokens. This option might be necessary if an attribute name conflicts with a DynamoDB reserved word. For example, the following <code>KeyConditionExpression</code> parameter causes an error because <i>Size</i> is a reserved word:</p> <ul>  <li>   <p><code>Size = :myval</code></p></li> </ul> <p>To work around this, define a placeholder (such a <code>#S</code>) to represent the attribute name <i>Size</i>. <code>KeyConditionExpression</code> then is as follows:</p> <ul>  <li>   <p><code>#S = :myval</code></p></li> </ul> <p>For a list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>For more information on <code>ExpressionAttributeNames</code> and <code>ExpressionAttributeValues</code>, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ExpressionPlaceholders.html">Using Placeholders for Attribute Names and Values</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::query::builders::QueryFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::query::builders::QueryFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::query::builders::QueryFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`expression_attribute_values(impl Into<String>, AttributeValue)`](crate::operation::query::builders::QueryFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap::<String, AttributeValue>>)`](crate::operation::query::builders::QueryFluentBuilder::set_expression_attribute_values):<br>required: **false**<br><p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following:</p> <p><code>Available | Backordered | Discontinued</code></p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p> <p>You could then use these values in an expression, such as this:</p> <p><code>ProductStatus IN (:avail, :back, :disc)</code></p> <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     /// - On success, responds with [`QueryOutput`](crate::operation::query::QueryOutput) with field(s):
     ///   - [`items(Option<Vec::<HashMap::<String, AttributeValue>>>)`](crate::operation::query::QueryOutput::items): <p>An array of item attributes that match the query criteria. Each element in this array consists of an attribute name and the value for that attribute.</p>
-    ///   - [`count(i32)`](crate::operation::query::QueryOutput::count): <p>The number of items in the response.</p> <p>If you used a <code>QueryFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p> <p>If you did not use a filter in the request, then <code>Count</code> and <code>ScannedCount</code> are the same.</p>
-    ///   - [`scanned_count(i32)`](crate::operation::query::QueryOutput::scanned_count): <p>The number of items evaluated, before any <code>QueryFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Query</code> operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
+    ///   - [`count(Option<i32>)`](crate::operation::query::QueryOutput::count): <p>The number of items in the response.</p> <p>If you used a <code>QueryFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p> <p>If you did not use a filter in the request, then <code>Count</code> and <code>ScannedCount</code> are the same.</p>
+    ///   - [`scanned_count(Option<i32>)`](crate::operation::query::QueryOutput::scanned_count): <p>The number of items evaluated, before any <code>QueryFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Query</code> operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
     ///   - [`last_evaluated_key(Option<HashMap::<String, AttributeValue>>)`](crate::operation::query::QueryOutput::last_evaluated_key): <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p> <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
     ///   - [`consumed_capacity(Option<ConsumedCapacity>)`](crate::operation::query::QueryOutput::consumed_capacity): <p>The capacity units consumed by the <code>Query</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#read-operation-consumption">Capacity unit consumption for read operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     /// - On failure, responds with [`SdkError<QueryError>`](crate::operation::query::QueryError)
```

### `src/client/scan.rs`

```diff
--- reference/src/client/scan.rs
+++ generated/src/client/scan.rs
@@ -17,13 +17,13 @@
     ///   - [`segment(i32)`](crate::operation::scan::builders::ScanFluentBuilder::segment) / [`set_segment(Option<i32>)`](crate::operation::scan::builders::ScanFluentBuilder::set_segment):<br>required: **false**<br><p>For a parallel <code>Scan</code> request, <code>Segment</code> identifies an individual segment to be scanned by an application worker.</p> <p>Segment IDs are zero-based, so the first segment is always 0. For example, if you want to use four application threads to scan a table or an index, then the first thread specifies a <code>Segment</code> value of 0, the second thread specifies 1, and so on.</p> <p>The value of <code>LastEvaluatedKey</code> returned from a parallel <code>Scan</code> request must be used as <code>ExclusiveStartKey</code> with the same segment ID in a subsequent <code>Scan</code> operation.</p> <p>The value for <code>Segment</code> must be greater than or equal to 0, and less than the value provided for <code>TotalSegments</code>.</p> <p>If you provide <code>Segment</code>, you must also provide <code>TotalSegments</code>.</p><br>
     ///   - [`projection_expression(impl Into<String>)`](crate::operation::scan::builders::ScanFluentBuilder::projection_expression) / [`set_projection_expression(Option<String>)`](crate::operation::scan::builders::ScanFluentBuilder::set_projection_expression):<br>required: **false**<br><p>A string that identifies one or more attributes to retrieve from the specified table or index. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`filter_expression(impl Into<String>)`](crate::operation::scan::builders::ScanFluentBuilder::filter_expression) / [`set_filter_expression(Option<String>)`](crate::operation::scan::builders::ScanFluentBuilder::set_filter_expression):<br>required: **false**<br><p>A string that contains conditions that DynamoDB applies after the <code>Scan</code> operation, but before the data is returned to you. Items that do not satisfy the <code>FilterExpression</code> criteria are not returned.</p><note>  <p>A <code>FilterExpression</code> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.FilterExpression">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::scan::builders::ScanFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::scan::builders::ScanFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::scan::builders::ScanFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::scan::builders::ScanFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`expression_attribute_values(impl Into<String>, AttributeValue)`](crate::operation::scan::builders::ScanFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap::<String, AttributeValue>>)`](crate::operation::scan::builders::ScanFluentBuilder::set_expression_attribute_values):<br>required: **false**<br><p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <code>ProductStatus</code> attribute was one of the following:</p> <p><code>Available | Backordered | Discontinued</code></p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p> <p>You could then use these values in an expression, such as this:</p> <p><code>ProductStatus IN (:avail, :back, :disc)</code></p> <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`consistent_read(bool)`](crate::operation::scan::builders::ScanFluentBuilder::consistent_read) / [`set_consistent_read(Option<bool>)`](crate::operation::scan::builders::ScanFluentBuilder::set_consistent_read):<br>required: **false**<br><p>A Boolean value that determines the read consistency model during the scan:</p> <ul>  <li>   <p>If <code>ConsistentRead</code> is <code>false</code>, then the data returned from <code>Scan</code> might not contain the results from other recently completed write operations (<code>PutItem</code>, <code>UpdateItem</code>, or <code>DeleteItem</code>).</p></li>  <li>   <p>If <code>ConsistentRead</code> is <code>true</code>, then all of the write operations that completed before the <code>Scan</code> began are guaranteed to be contained in the <code>Scan</code> response.</p></li> </ul> <p>The default setting for <code>ConsistentRead</code> is <code>false</code>.</p> <p>The <code>ConsistentRead</code> parameter is not supported on global secondary indexes. If you scan a global secondary index with <code>ConsistentRead</code> set to true, you will receive a <code>ValidationException</code>.</p><br>
     /// - On success, responds with [`ScanOutput`](crate::operation::scan::ScanOutput) with field(s):
     ///   - [`items(Option<Vec::<HashMap::<String, AttributeValue>>>)`](crate::operation::scan::ScanOutput::items): <p>An array of item attributes that match the scan criteria. Each element in this array consists of an attribute name and the value for that attribute.</p>
-    ///   - [`count(i32)`](crate::operation::scan::ScanOutput::count): <p>The number of items in the response.</p> <p>If you set <code>ScanFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p> <p>If you did not use a filter in the request, then <code>Count</code> is the same as <code>ScannedCount</code>.</p>
-    ///   - [`scanned_count(i32)`](crate::operation::scan::ScanOutput::scanned_count): <p>The number of items evaluated, before any <code>ScanFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Scan</code> operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
+    ///   - [`count(Option<i32>)`](crate::operation::scan::ScanOutput::count): <p>The number of items in the response.</p> <p>If you set <code>ScanFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p> <p>If you did not use a filter in the request, then <code>Count</code> is the same as <code>ScannedCount</code>.</p>
+    ///   - [`scanned_count(Option<i32>)`](crate::operation::scan::ScanOutput::scanned_count): <p>The number of items evaluated, before any <code>ScanFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Scan</code> operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
     ///   - [`last_evaluated_key(Option<HashMap::<String, AttributeValue>>)`](crate::operation::scan::ScanOutput::last_evaluated_key): <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p> <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
     ///   - [`consumed_capacity(Option<ConsumedCapacity>)`](crate::operation::scan::ScanOutput::consumed_capacity): <p>The capacity units consumed by the <code>Scan</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#read-operation-consumption">Capacity unit consumption for read operations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     /// - On failure, responds with [`SdkError<ScanError>`](crate::operation::scan::ScanError)
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
+    ///   - [`update_expression(impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::update_expression) / [`set_update_expression(Option<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_update_expression):<br>required: **false**<br><p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new values for them.</p> <p>The following action values are available for <code>UpdateExpression</code>.</p> <ul>  <li>   <p><code>SET</code> - Adds one or more attributes and values to an item. If any of these attributes already exist, they are replaced by the new values. You can also use <code>SET</code> to add or subtract from an attribute that is of type Number. For example: <code>SET myNum = myNum + :val</code></p>   <p><code>SET</code> supports the following functions:</p>   <ul>    <li>     <p><code>if_not_exists (path, operand)</code> - if the item does not contain an attribute at the specified path, then <code>if_not_exists</code> evaluates to operand; otherwise, it evaluates to path. You can use this function to avoid overwriting an attribute that may already be present in the item.</p></li>    <li>     <p><code>list_append (operand, operand)</code> - evaluates to a list with a new element added to it. You can append the new element to the start or the end of the list by reversing the order of the operands.</p></li>   </ul>   <p>These function names are case-sensitive.</p></li>  <li>   <p><code>REMOVE</code> - Removes one or more attributes from an item.</p></li>  <li>   <p><code>ADD</code> - Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p>   <ul>    <li>     <p>If the existing attribute is a number, and if <code>Value</code> is also a number, then <code>Value</code> is mathematically added to the existing attribute. If <code>Value</code> is a negative number, then it is subtracted from the existing attribute.</p><note>      <p>If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value.</p> <p>Similarly, if you use <code>ADD</code> for an existing item to increment or decrement an attribute value that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update doesn't have an attribute named <code>itemcount</code>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway. DynamoDB will create the <code>itemcount</code> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <code>itemcount</code> attribute in the item, with a value of <code>3</code>.</p> </note></li>    <li>     <p>If the existing data type is a set and if <code>Value</code> is also a set, then <code>Value</code> is added to the existing set. For example, if the attribute value is the set <code>[1,2]</code>, and the <code>ADD</code> action specified <code>[3]</code>, then the final attribute value is <code>[1,2,3]</code>. An error occurs if an <code>ADD</code> action is specified for a set attribute and the attribute type specified does not match the existing set type.</p>     <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <code>Value</code> must also be a set of strings.</p></li>   </ul> <important>    <p>The <code>ADD</code> action only supports Number and set data types.</p> </important></li>  <li>   <p><code>DELETE</code> - Deletes an element from a set.</p>   <p>If a set of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>[a,b,c]</code> and the <code>DELETE</code> action specifies <code>[a,c]</code>, then the final attribute value is <code>[b]</code>. Specifying an empty set is an error.</p><important>    <p>The <code>DELETE</code> action only supports set data types.</p> </important></li> </ul> <p>You can have many actions in a single expression, such as the following: <code>SET a=:value1, b=:value2 DELETE :value3, :value4, :value5</code></p> <p>For more information on update expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Modifying.html">Modifying Items and Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`condition_expression(impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::condition_expression) / [`set_condition_expression(Option<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_condition_expression):<br>required: **false**<br><p>A condition that must be satisfied in order for a conditional update to succeed.</p> <p>An expression can contain any of the following:</p> <ul>  <li>   <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code></p>   <p>These function names are case-sensitive.</p></li>  <li>   <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code></p></li>  <li>   <p>Logical operators: <code>AND | OR | NOT</code></p></li> </ul> <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
-    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.) To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul><note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
+    ///   - [`expression_attribute_names(impl Into<String>, impl Into<String>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap::<String, String>>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_expression_attribute_names):<br>required: **false**<br><p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>  <li>   <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>  <li>   <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul>  <li>   <p><code>Percentile</code></p></li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.) To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul>  <li>   <p><code>{"#P":"Percentile"}</code></p></li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul>  <li>   <p><code>#P = :val</code></p></li> </ul> <note>  <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information about expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`expression_attribute_values(impl Into<String>, AttributeValue)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap::<String, AttributeValue>>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_expression_attribute_values):<br>required: **false**<br><p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <code>ProductStatus</code> attribute was one of the following:</p> <p><code>Available | Backordered | Discontinued</code></p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p> <p>You could then use these values in an expression, such as this:</p> <p><code>ProductStatus IN (:avail, :back, :disc)</code></p> <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p><br>
     ///   - [`return_values_on_condition_check_failure(ReturnValuesOnConditionCheckFailure)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::return_values_on_condition_check_failure) / [`set_return_values_on_condition_check_failure(Option<ReturnValuesOnConditionCheckFailure>)`](crate::operation::update_item::builders::UpdateItemFluentBuilder::set_return_values_on_condition_check_failure):<br>required: **false**<br><p>An optional parameter that returns the item attributes for an <code>UpdateItem</code> operation that failed a condition check.</p> <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p><br>
     /// - On success, responds with [`UpdateItemOutput`](crate::operation::update_item::UpdateItemOutput) with field(s):
```

### `src/client.rs`

```diff
--- reference/src/client.rs
+++ generated/src/client.rs
@@ -1,338 +1,322 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-#[derive(Debug)]
-pub(crate) struct Handle {
-    pub(crate) conf: crate::Config,
-    #[allow(dead_code)] // unused when a service does not provide any operations
-    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
-}

-/// Client for Amazon DynamoDB
-///
-/// Client for invoking operations on Amazon DynamoDB. Each operation on Amazon DynamoDB is a method on this
-/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
-/// ## Constructing a `Client`
-///
-/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
-/// crate should be used to automatically resolve this config using
-/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
-/// across multiple different AWS SDK clients. This config resolution process can be customized
-/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
-/// the [builder pattern] to customize the default config.
-///
-/// In the simplest case, creating a client looks as follows:
-/// ```rust,no_run
-/// # async fn wrapper() {
-/// let config = aws_config::load_from_env().await;
-/// let client = aws_sdk_dynamodb::Client::new(&config);
-/// # }
-/// ```
-///
-/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that
-/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
-/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be
-/// done as follows:
-///
-/// ```rust,no_run
-/// # async fn wrapper() {
-/// let sdk_config = ::aws_config::load_from_env().await;
-/// let config = aws_sdk_dynamodb::config::Builder::from(&sdk_config)
-/// # /*
-///     .some_service_specific_setting("value")
-/// # */
-///     .build();
-/// # }
-/// ```
-///
-/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
-///
-/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
-/// be done once at application start-up.
-///
-/// [`Config`]: crate::Config
-/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
-/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
-/// [`aws-config` docs]: https://docs.rs/aws-config/*
-/// [`aws-config`]: https://crates.io/crates/aws-config
-/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
-/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
-/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
-/// # Using the `Client`
-///
-/// A client has a function for every operation that can be performed by the service.
-/// For example, the [`BatchExecuteStatement`](crate::operation::batch_execute_statement) operation has
-/// a [`Client::batch_execute_statement`], function which returns a builder for that operation.
-/// The fluent builder ultimately has a `send()` function that returns an async future that
-/// returns a result, as illustrated below:
-///
-/// ```rust,ignore
-/// let result = client.batch_execute_statement()
-///     .return_consumed_capacity("example")
-///     .send()
-///     .await;
-/// ```
-///
-/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
-/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
-/// information.
-/// # Waiters
-///
-/// This client provides `wait_until` methods behind the [`Waiters`](crate::client::Waiters) trait.
-/// To use them, simply import the trait, and then call one of the `wait_until` methods. This will
-/// return a waiter fluent builder that takes various parameters, which are documented on the builder
-/// type. Once parameters have been provided, the `wait` method can be called to initiate waiting.
-///
-/// For example, if there was a `wait_until_thing` method, it could look like:
-/// ```rust,ignore
-/// let result = client.wait_until_thing()
-///     .thing_id("someId")
-///     .wait(Duration::from_secs(120))
-///     .await;
-/// ```
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct Client {
-    handle: ::std::sync::Arc<Handle>,
-}
+#[allow(dead_code)]
+pub(crate) mod transport {
+    use ::std::collections::BTreeMap;
+    use ::std::fmt;
+    use ::std::io::{Read, Write};
+    use ::std::net::TcpStream;
+
+    #[derive(Clone, Copy, Debug)]
+    pub(crate) enum Method {
+        Get,
+        Put,
+        Post,
+        Delete,
+        Head,
+        Patch,
+    }

-impl Client {
-    /// Creates a new client from the service [`Config`](crate::Config).
-    ///
-    /// # Panics
-    ///
-    /// This method will panic in the following cases:
-    ///
-    /// - Retries or timeouts are enabled without a `sleep_impl` configured.
-    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.
-    /// - No `behavior_version` is provided.
-    ///
-    /// The panic message for each of these will have instructions on how to resolve them.
-    #[track_caller]
-    pub fn from_conf(conf: crate::Config) -> Self {
-        let handle = Handle {
-            conf: conf.clone(),
-            runtime_plugins: crate::config::base_client_runtime_plugins(conf),
-        };
-        if let Err(err) = Self::validate_config(&handle) {
-            panic!("Invalid client configuration: {err}");
+    impl Method {
+        fn as_str(self) -> &'static str {
+            match self {
+                Self::Get => "GET",
+                Self::Put => "PUT",
+                Self::Post => "POST",
+                Self::Delete => "DELETE",
+                Self::Head => "HEAD",
+                Self::Patch => "PATCH",
+            }
         }
-        Self {
-            handle: ::std::sync::Arc::new(handle),
+    }
+
+    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
+    pub(crate) struct StatusCode(u16);
+
+    impl StatusCode {
+        pub(crate) const CONFLICT: Self = Self(409);
+        pub(crate) fn is_success(self) -> bool {
+            (200..300).contains(&self.0)
         }
     }

-    /// Returns the client's configuration.
-    pub fn config(&self) -> &crate::Config {
-        &self.handle.conf
+    impl fmt::Display for StatusCode {
+        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
+            self.0.fmt(formatter)
+        }
     }

-    fn validate_config(handle: &Handle) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
-        handle
-            .runtime_plugins
-            .apply_client_configuration(&mut cfg)?
-            .validate_base_client_config(&cfg)?;
-        Ok(())
+    #[derive(Clone, Debug)]
+    pub(crate) struct Response {
+        status: StatusCode,
+        headers: BTreeMap<String, String>,
+        body: Vec<u8>,
     }
-}

-///
-/// Waiter functions for the client.
-///
-/// Import this trait to get `wait_until` methods on the client.
-///
-pub trait Waiters {
-    /// Wait for `contributor_insights_enabled`
-    fn wait_until_contributor_insights_enabled(&self) -> crate::waiters::contributor_insights_enabled::ContributorInsightsEnabledFluentBuilder;
-    /// Wait for `export_completed`
-    fn wait_until_export_completed(&self) -> crate::waiters::export_completed::ExportCompletedFluentBuilder;
-    /// Wait for `import_completed`
-    fn wait_until_import_completed(&self) -> crate::waiters::import_completed::ImportCompletedFluentBuilder;
-    /// Wait for `kinesis_streaming_destination_active`
-    fn wait_until_kinesis_streaming_destination_active(
-        &self,
-    ) -> crate::waiters::kinesis_streaming_destination_active::KinesisStreamingDestinationActiveFluentBuilder;
-    /// Wait for `table_exists`
-    fn wait_until_table_exists(&self) -> crate::waiters::table_exists::TableExistsFluentBuilder;
-    /// Wait for `table_not_exists`
-    fn wait_until_table_not_exists(&self) -> crate::waiters::table_not_exists::TableNotExistsFluentBuilder;
-}
-impl Waiters for Client {
-    fn wait_until_contributor_insights_enabled(&self) -> crate::waiters::contributor_insights_enabled::ContributorInsightsEnabledFluentBuilder {
-        crate::waiters::contributor_insights_enabled::ContributorInsightsEnabledFluentBuilder::new(self.handle.clone())
+    impl Response {
+        pub(crate) fn status(&self) -> StatusCode {
+            self.status
+        }
+        pub(crate) fn header(&self, name: &str) -> Option<&str> {
+            self.headers.get(&name.to_ascii_lowercase()).map(String::as_str)
+        }
+        pub(crate) fn body(&self) -> &[u8] {
+            &self.body
+        }
+        pub(crate) async fn text(&self) -> Result<String, String> {
+            String::from_utf8(self.body.clone()).map_err(|error| error.to_string())
+        }
     }
-    fn wait_until_export_completed(&self) -> crate::waiters::export_completed::ExportCompletedFluentBuilder {
-        crate::waiters::export_completed::ExportCompletedFluentBuilder::new(self.handle.clone())
+
+    #[derive(Clone, Debug, Default)]
+    pub(crate) struct HttpClient;
+
+    impl HttpClient {
+        pub(crate) fn new() -> Self {
+            Self
+        }
+        pub(crate) async fn request(&self, method: Method, url: &str, headers: &[(&str, &str)], body: &[u8]) -> Result<Response, String> {
+            let (host, port, path) = parse_http_url(url)?;
+            let mut stream = TcpStream::connect((host.as_str(), port)).map_err(|error| format!("failed to connect to {host}:{port}: {error}"))?;
+            let mut request = format!(
+                "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nContent-Length: {}\r\n",
+                method.as_str(),
+                path,
+                host,
+                body.len()
+            );
+            for (name, value) in headers {
+                request.push_str(name);
+                request.push_str(": ");
+                request.push_str(value);
+                request.push_str("\r\n");
+            }
+            request.push_str("\r\n");
+            let mut request_bytes = request.into_bytes();
+            request_bytes.extend_from_slice(body);
+            stream
+                .write_all(&request_bytes)
+                .map_err(|error| format!("failed to write HTTP request: {error}"))?;
+            let mut bytes = Vec::new();
+            stream
+                .read_to_end(&mut bytes)
+                .map_err(|error| format!("failed to read HTTP response: {error}"))?;
+            parse_response(&bytes)
+        }
+    }
+
+    fn parse_http_url(url: &str) -> Result<(String, u16, String), String> {
+        let authority_and_path = url
+            .strip_prefix("http://")
+            .ok_or_else(|| format!("only http:// endpoints are supported: {url}"))?;
+        let (authority, path) = authority_and_path
+            .split_once('/')
+            .map_or((authority_and_path, "/"), |(authority, _path)| {
+                (authority, &authority_and_path[authority.len()..])
+            });
+        if authority.is_empty() {
+            return Err(format!("endpoint has no host: {url}"));
+        }
+        let (host, port) = if let Some((host, port)) = authority.rsplit_once(':') {
+            let port = port.parse::<u16>().map_err(|error| format!("invalid endpoint port in {url}: {error}"))?;
+            (host.to_owned(), port)
+        } else {
+            (authority.to_owned(), 80)
+        };
+        Ok((host, port, path.to_owned()))
+    }
+
+    fn parse_response(bytes: &[u8]) -> Result<Response, String> {
+        let header_end = bytes
+            .windows(4)
+            .position(|window| window == b"\r\n\r\n")
+            .ok_or_else(|| "HTTP response did not contain a header terminator".to_owned())?;
+        let header = ::std::str::from_utf8(&bytes[..header_end]).map_err(|error| format!("HTTP response headers were not UTF-8: {error}"))?;
+        let status = header
+            .lines()
+            .next()
+            .and_then(|line| line.split_whitespace().nth(1))
+            .ok_or_else(|| "HTTP response did not contain a status code".to_owned())?
+            .parse::<u16>()
+            .map_err(|error| format!("HTTP response status was invalid: {error}"))?;
+        let mut headers = BTreeMap::new();
+        for line in header.lines().skip(1) {
+            if let Some((name, value)) = line.split_once(':') {
+                headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
+            }
+        }
+        Ok(Response {
+            status: StatusCode(status),
+            headers,
+            body: bytes[header_end + 4..].to_vec(),
+        })
+    }
+
+    pub(crate) fn encode_path(value: &str) -> String {
+        value.bytes().fold(String::new(), |mut result, byte| {
+            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~' | b'/') {
+                result.push(byte as char);
+            } else {
+                result.push('%');
+                result.push(hex(byte >> 4));
+                result.push(hex(byte & 0x0f));
+            }
+            result
+        })
+    }
+    fn hex(value: u8) -> char {
+        match value {
+            0..=9 => (b'0' + value) as char,
+            _ => (b'A' + value - 10) as char,
+        }
     }
-    fn wait_until_import_completed(&self) -> crate::waiters::import_completed::ImportCompletedFluentBuilder {
-        crate::waiters::import_completed::ImportCompletedFluentBuilder::new(self.handle.clone())
+    pub(crate) fn xml_escape(value: &str) -> String {
+        value
+            .replace('&', "&amp;")
+            .replace('<', "&lt;")
+            .replace('>', "&gt;")
+            .replace('\"', "&quot;")
+            .replace('\'', "&apos;")
     }
-    fn wait_until_kinesis_streaming_destination_active(
-        &self,
-    ) -> crate::waiters::kinesis_streaming_destination_active::KinesisStreamingDestinationActiveFluentBuilder {
-        crate::waiters::kinesis_streaming_destination_active::KinesisStreamingDestinationActiveFluentBuilder::new(self.handle.clone())
+    pub(crate) fn xml_unescape(value: &str) -> String {
+        value
+            .replace("&lt;", "<")
+            .replace("&gt;", ">")
+            .replace("&apos;", "'")
+            .replace("&amp;", "&")
     }
-    fn wait_until_table_exists(&self) -> crate::waiters::table_exists::TableExistsFluentBuilder {
-        crate::waiters::table_exists::TableExistsFluentBuilder::new(self.handle.clone())
+    pub(crate) fn xml_first(xml: &str, tag: &str) -> Option<String> {
+        xml_tags(xml, tag).into_iter().next().map(|value| xml_unescape(&value))
     }
-    fn wait_until_table_not_exists(&self) -> crate::waiters::table_not_exists::TableNotExistsFluentBuilder {
-        crate::waiters::table_not_exists::TableNotExistsFluentBuilder::new(self.handle.clone())
+    pub(crate) fn xml_tags(xml: &str, tag: &str) -> Vec<String> {
+        let open = format!("<{tag}>");
+        let close = format!("</{tag}>");
+        let mut values = Vec::new();
+        let mut remaining = xml;
+        while let Some(start) = remaining.find(&open) {
+            let value_start = start + open.len();
+            let Some(end) = remaining[value_start..].find(&close) else { break };
+            values.push(remaining[value_start..value_start + end].to_owned());
+            remaining = &remaining[value_start + end + close.len()..];
+        }
+        values
     }
 }

+// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+
+#[derive(Clone, Debug, Default)]
+pub struct Client {
+    config: Config,
+    http: transport::HttpClient,
+}
 impl Client {
-    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).
-    ///
-    /// # Panics
-    ///
-    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
-    ///   the `sleep_impl` on the Config passed into this function to fix it.
-    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
-    ///   `http_connector` on the Config passed into this function to fix it.
-    /// - This method will panic if no `BehaviorVersion` is provided. If you experience this panic, set `behavior_version` on the Config or enable the `behavior-version-latest` Cargo feature.
-    #[track_caller]
-    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Self::from_conf(sdk_config.into())
+    pub fn new(config: &Config) -> Self {
+        Self {
+            config: config.clone(),
+            http: transport::HttpClient::new(),
+        }
+    }
+    pub fn config(&self) -> &Config {
+        &self.config
+    }
+    pub(crate) async fn request(
+        &self,
+        method: transport::Method,
+        path: &str,
+        headers: &[(&str, &str)],
+        body: &[u8],
+    ) -> ::std::result::Result<transport::Response, ::std::string::String> {
+        let url = format!("{}{}", self.config.endpoint_url.trim_end_matches('/'), path);
+        self.http.request(method, &url, headers, body).await
     }
 }

-mod batch_execute_statement;
-
-mod batch_get_item;
-
-mod batch_write_item;
-
-mod create_backup;
-
-mod create_global_table;
-
-mod create_table;
-
-/// Operation customization and supporting types.
-///
-/// The underlying HTTP requests made during an operation can be customized
-/// by calling the `customize()` method on the builder returned from a client
-/// operation call. For example, this can be used to add an additional HTTP header:
-///
-/// ```ignore
-/// # async fn wrapper() -> ::std::result::Result<(), aws_sdk_dynamodb::Error> {
-/// # let client: aws_sdk_dynamodb::Client = unimplemented!();
-/// use ::http_1x::header::{HeaderName, HeaderValue};
-///
-/// let result = client.batch_execute_statement()
-///     .customize()
-///     .mutate_request(|req| {
-///         // Add `x-example-header` with value
-///         req.headers_mut()
-///             .insert(
-///                 HeaderName::from_static("x-example-header"),
-///                 HeaderValue::from_static("1"),
-///             );
-///     })
-///     .send()
-///     .await;
-/// # }
-/// ```
-pub mod customize;
-
-mod delete_backup;
-
-mod delete_item;
-
-mod delete_resource_policy;
-
-mod delete_table;
-
-mod describe_backup;
-
-mod describe_continuous_backups;
-
-mod describe_contributor_insights;
-
-mod describe_endpoints;
-
-mod describe_export;
-
-mod describe_global_table;
-
-mod describe_global_table_settings;
-
-mod describe_import;
-
-mod describe_kinesis_streaming_destination;
-
-mod describe_limits;
-
-mod describe_table;
-
-mod describe_table_replica_auto_scaling;
-
-mod describe_time_to_live;
-
-mod disable_kinesis_streaming_destination;
-
-mod enable_kinesis_streaming_destination;
-
-mod execute_statement;
-
-mod execute_transaction;
-
-mod export_table_to_point_in_time;
-
-mod get_item;
-
-mod get_resource_policy;
-
-mod import_table;
-
-mod list_backups;
-
-mod list_contributor_insights;
-
-mod list_exports;
-
-mod list_global_tables;
-
-mod list_imports;
-
-mod list_tables;
-
-mod list_tags_of_resource;
-
-mod put_item;
-
-mod put_resource_policy;
-
-mod query;
-
-mod restore_table_from_backup;
-
-mod restore_table_to_point_in_time;
-
-mod scan;
-
-mod search_vectors;
-
-mod tag_resource;
-
-mod transact_get_items;
-
-mod transact_write_items;
-
-mod untag_resource;
-
-mod update_continuous_backups;
-
-mod update_contributor_insights;
-
-mod update_global_table;
-
-mod update_global_table_settings;
-
-mod update_item;
-
-mod update_kinesis_streaming_destination;
-
-mod update_table;
-
-mod update_table_replica_auto_scaling;
-
-mod update_time_to_live;
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/batch_execute_statement.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/batch_get_item.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/batch_write_item.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/create_backup.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/create_global_table.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/create_table.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/delete_backup.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/delete_item.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/delete_resource_policy.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/delete_table.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_backup.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_continuous_backups.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/describe_contributor_insights.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_endpoints.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_export.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_global_table.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/describe_global_table_settings.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_import.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/describe_kinesis_streaming_destination.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_limits.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_table.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/describe_table_replica_auto_scaling.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/describe_time_to_live.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/disable_kinesis_streaming_destination.rs"
+));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/enable_kinesis_streaming_destination.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/execute_statement.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/execute_transaction.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/export_table_to_point_in_time.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/get_item.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/get_resource_policy.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/import_table.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/list_backups.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/list_contributor_insights.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/list_exports.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/list_global_tables.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/list_imports.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/list_tables.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/list_tags_of_resource.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/put_item.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/put_resource_policy.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/query.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/restore_table_from_backup.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/restore_table_to_point_in_time.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/scan.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/search_vectors.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/tag_resource.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/transact_get_items.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/transact_write_items.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/untag_resource.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/update_continuous_backups.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/update_contributor_insights.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/update_global_table.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/update_global_table_settings.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/update_item.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/update_kinesis_streaming_destination.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/update_table.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/dynamodb/src/client/update_table_replica_auto_scaling.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client/update_time_to_live.rs"));
```

### `src/config.rs`

```diff
--- reference/src/config.rs
+++ generated/src/config.rs
@@ -1,1769 +1,45 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-#![allow(clippy::empty_line_after_doc_comments)]
-/// Configuration for a aws_sdk_dynamodb service client.
-///
-/// Service configuration allows for customization of endpoints, region, credentials providers,
-/// and retry configuration. Generally, it is constructed automatically for you from a shared
-/// configuration loaded by the `aws-config` crate. For example:
-///
-/// ```ignore
-/// // Load a shared config from the environment
-/// let shared_config = aws_config::from_env().load().await;
-/// // The client constructor automatically converts the shared config into the service config
-/// let client = Client::new(&shared_config);
-/// ```
-///
-/// The service config can also be constructed manually using its builder.
-///
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
+
+#[derive(Clone, Debug)]
 pub struct Config {
-    // Both `config` and `cloneable` are the same config, but the cloneable one
-    // is kept around so that it is possible to convert back into a builder. This can be
-    // optimized in the future.
-    pub(crate) config: crate::config::FrozenLayer,
-    cloneable: ::aws_smithy_types::config_bag::CloneableLayer,
-    pub(crate) runtime_components: crate::config::RuntimeComponentsBuilder,
-    pub(crate) runtime_plugins: ::std::vec::Vec<crate::config::SharedRuntimePlugin>,
-    pub(crate) behavior_version: ::std::option::Option<crate::config::BehaviorVersion>,
+    pub(crate) endpoint_url: ::std::string::String,
 }
-impl Config {
-    ///
-    /// Constructs a config builder.
-    /// <div class="warning">
-    /// Note that a config created from this builder will not have the same safe defaults as one created by
-    /// the <a href="https://crates.io/crates/aws-config" target="_blank">aws-config</a> crate.
-    /// </div>
-    ///
-    pub fn builder() -> Builder {
-        Builder::default()
-    }
-    /// Converts this config back into a builder so that it can be tweaked.
-    pub fn to_builder(&self) -> Builder {
-        Builder {
-            config: self.cloneable.clone(),
-            runtime_components: self.runtime_components.clone(),
-            runtime_plugins: self.runtime_plugins.clone(),
-            behavior_version: self.behavior_version,
-        }
-    }
-    /// Return a reference to the stalled stream protection configuration contained in this config, if any.
-    pub fn stalled_stream_protection(&self) -> ::std::option::Option<&crate::config::StalledStreamProtectionConfig> {
-        self.config.load::<crate::config::StalledStreamProtectionConfig>()
-    }
-    /// Return the [`SharedHttpClient`](crate::config::SharedHttpClient) to use when making requests, if any.
-    pub fn http_client(&self) -> Option<crate::config::SharedHttpClient> {
-        self.runtime_components.http_client()
-    }
-    /// Return the auth schemes configured on this service config
-    pub fn auth_schemes(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::auth::SharedAuthScheme> + '_ {
-        self.runtime_components.auth_schemes()
-    }
-
-    /// Return the auth scheme resolver configured on this service config
-    pub fn auth_scheme_resolver(&self) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
-        self.runtime_components.auth_scheme_option_resolver()
-    }
-    /// Returns the configured auth scheme preference
-    pub fn auth_scheme_preference(&self) -> ::std::option::Option<&::aws_smithy_runtime_api::client::auth::AuthSchemePreference> {
-        self.config.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
-    }
-    /// Returns the endpoint resolver.
-    pub fn endpoint_resolver(&self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
-        self.runtime_components.endpoint_resolver().expect("resolver defaulted if not set")
-    }
-    /// Return a reference to the retry configuration contained in this config, if any.
-    pub fn retry_config(&self) -> ::std::option::Option<&::aws_smithy_types::retry::RetryConfig> {
-        self.config.load::<::aws_smithy_types::retry::RetryConfig>()
-    }

-    /// Return a cloned shared async sleep implementation from this config, if any.
-    pub fn sleep_impl(&self) -> ::std::option::Option<crate::config::SharedAsyncSleep> {
-        self.runtime_components.sleep_impl()
-    }
-
-    /// Return a reference to the timeout configuration contained in this config, if any.
-    pub fn timeout_config(&self) -> ::std::option::Option<&::aws_smithy_types::timeout::TimeoutConfig> {
-        self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>()
-    }
-
-    /// Returns a reference to the retry partition contained in this config, if any.
-    ///
-    /// WARNING: This method is unstable and may be removed at any time. Do not rely on this
-    /// method for anything!
-    pub fn retry_partition(&self) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
-        self.config.load::<::aws_smithy_runtime::client::retries::RetryPartition>()
-    }
-    /// Returns the configured identity cache for auth.
-    pub fn identity_cache(&self) -> ::std::option::Option<crate::config::SharedIdentityCache> {
-        self.runtime_components.identity_cache()
-    }
-    /// Returns interceptors currently registered by the user.
-    pub fn interceptors(&self) -> impl Iterator<Item = crate::config::SharedInterceptor> + '_ {
-        self.runtime_components.interceptors()
-    }
-    /// Return time source used for this service.
-    pub fn time_source(&self) -> ::std::option::Option<::aws_smithy_async::time::SharedTimeSource> {
-        self.runtime_components.time_source()
-    }
-    /// Returns retry classifiers currently registered by the user.
-    pub fn retry_classifiers(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
-        self.runtime_components.retry_classifiers()
-    }
-    /// Returns the name of the app that is using the client, if it was provided.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn app_name(&self) -> ::std::option::Option<&::aws_types::app_name::AppName> {
-        self.config.load::<::aws_types::app_name::AppName>()
-    }
-    /// Returns the framework metadata that has been configured, if any.
-    ///
-    /// This _optional_ metadata identifies software frameworks or third-party libraries
-    /// being used with the client, rendered into the user agent as `lib/{name}/{version}`.
-    /// Entries are returned in first-seen (insertion) order, matching the order they are
-    /// rendered into the user agent.
-    pub fn framework_metadata(&self) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
-        // `StoreAppend` loads entries newest-first; reverse to first-seen order so
-        // this getter agrees with both the user agent and `SdkConfig::framework_metadata`.
-        let mut entries: ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> =
-            self.config.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>().collect();
-        entries.reverse();
-        entries
-    }
-    /// Returns the invocation ID generator if one was given in config.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(&self) -> ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator> {
-        self.config.load::<::aws_runtime::invocation_id::SharedInvocationIdGenerator>().cloned()
-    }
-    /// Creates a new [service config](crate::Config) from a [shared `config`](::aws_types::sdk_config::SdkConfig).
-    pub fn new(config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Builder::from(config).build()
-    }
-    /// The signature version 4 service signing name to use in the credential scope when signing requests.
-    ///
-    /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
-    /// [`SigningName`](aws_types::SigningName) during operation construction
-    pub fn signing_name(&self) -> &'static str {
-        "dynamodb"
-    }
-    /// Returns the AWS region, if it was provided.
-    pub fn region(&self) -> ::std::option::Option<&crate::config::Region> {
-        self.config.load::<crate::config::Region>()
-    }
-    /// This function was intended to be removed, and has been broken since release-2023-11-15 as it always returns a `None`. Do not use.
-    #[deprecated(
-        note = "This function was intended to be removed, and has been broken since release-2023-11-15 as it always returns a `None`. Do not use."
-    )]
-    pub fn credentials_provider(&self) -> Option<crate::config::SharedCredentialsProvider> {
-        ::std::option::Option::None
-    }
-}
-/// Builder for creating a `Config`.
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct Builder {
-    pub(crate) config: ::aws_smithy_types::config_bag::CloneableLayer,
-    pub(crate) runtime_components: crate::config::RuntimeComponentsBuilder,
-    pub(crate) runtime_plugins: ::std::vec::Vec<crate::config::SharedRuntimePlugin>,
-    pub(crate) behavior_version: ::std::option::Option<crate::config::BehaviorVersion>,
-}
-impl ::std::default::Default for Builder {
+impl ::std::default::Default for Config {
     fn default() -> Self {
         Self {
-            config: ::std::default::Default::default(),
-            runtime_components: crate::config::RuntimeComponentsBuilder::new("service config"),
-            runtime_plugins: ::std::default::Default::default(),
-            behavior_version: ::std::default::Default::default(),
+            endpoint_url: ::std::env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localhost:4566".to_owned()),
         }
     }
 }
-impl Builder {
-    ///
-    /// Constructs a config builder.
-    /// <div class="warning">
-    /// Note that a config created from this builder will not have the same safe defaults as one created by
-    /// the <a href="https://crates.io/crates/aws-config" target="_blank">aws-config</a> crate.
-    /// </div>
-    ///
-    pub fn new() -> Self {
-        Self::default()
-    }
-    /// Constructs a config builder from the given `config_bag`, setting only fields stored in the config bag,
-    /// but not those in runtime components.
-    #[allow(unused)]
-    pub(crate) fn from_config_bag(config_bag: &::aws_smithy_types::config_bag::ConfigBag) -> Self {
-        let mut builder = Self::new();
-        builder.set_stalled_stream_protection(config_bag.load::<crate::config::StalledStreamProtectionConfig>().cloned());
-        builder.set_auth_scheme_preference(config_bag.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>().cloned());
-        builder.set_retry_config(config_bag.load::<::aws_smithy_types::retry::RetryConfig>().cloned());
-        builder.set_timeout_config(config_bag.load::<::aws_smithy_types::timeout::TimeoutConfig>().cloned());
-        builder.set_retry_partition(config_bag.load::<::aws_smithy_runtime::client::retries::RetryPartition>().cloned());
-        builder.set_app_name(config_bag.load::<::aws_types::app_name::AppName>().cloned());
-        for framework_metadata in config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>() {
-            builder.push_framework_metadata(framework_metadata.clone());
-        }
-        builder.set_account_id_endpoint_mode(config_bag.load::<::aws_types::endpoint_config::AccountIdEndpointMode>().cloned());
-        builder.set_endpoint_url(config_bag.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()));
-        builder.set_use_dual_stack(config_bag.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0));
-        builder.set_use_fips(config_bag.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0));
-        builder.set_region(config_bag.load::<crate::config::Region>().cloned());
-        builder
-    }
-    /// Names operation-input members whose values are captured *and* emitted as
-    /// attributes on the client's built-in metrics (e.g. `["Bucket"]`).
-    ///
-    /// Emitting implies capture, so an emitted member is also readable in-process
-    /// via `CapturedTelemetryAttributes` on the config bag. Names are Smithy input
-    /// member names; only string-valued, non-sensitive members are eligible, and
-    /// naming any other member has no effect. Off by default.
-    ///
-    /// Prefer bounded identifiers here: an emitted member becomes a metric label, so
-    /// high-cardinality values (like object keys) fragment the metrics and inflate
-    /// cost. Use [`Self::capture_input_attributes`] for values you want to read
-    /// in-process without emitting them on the metrics.
-    pub fn emit_input_attributes(mut self, names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>) -> Self {
-        let mut requested = self
-            .config
-            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
-            .cloned()
-            .unwrap_or_default();
-        requested.emit(names.into_iter().map(|n| n.into()));
-        self.config.store_put(requested);
-        self
-    }

-    /// Names operation-input members whose values are captured into
-    /// `CapturedTelemetryAttributes` for in-process reads (e.g. from a custom
-    /// interceptor), but are **not** emitted on the built-in metrics.
-    ///
-    /// Use this for values you need during the operation lifecycle but do not want on
-    /// the metric label set (for example, high-cardinality identifiers). Names follow
-    /// the same eligibility rules as [`Self::emit_input_attributes`]. Off by default.
-    pub fn capture_input_attributes(
-        mut self,
-        names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>,
-    ) -> Self {
-        let mut requested = self
-            .config
-            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
-            .cloned()
-            .unwrap_or_default();
-        requested.capture_only(names.into_iter().map(|n| n.into()));
-        self.config.store_put(requested);
-        self
-    }
-    /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
-    /// to configure protection for stalled streams.
-    pub fn stalled_stream_protection(mut self, stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig) -> Self {
-        self.set_stalled_stream_protection(::std::option::Option::Some(stalled_stream_protection_config));
-        self
-    }
-    /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
-    /// to configure protection for stalled streams.
-    pub fn set_stalled_stream_protection(
-        &mut self,
-        stalled_stream_protection_config: ::std::option::Option<crate::config::StalledStreamProtectionConfig>,
-    ) -> &mut Self {
-        self.config.store_or_unset(stalled_stream_protection_config);
-        self
-    }
-    /// Sets the idempotency token provider to use for service calls that require tokens.
-    pub fn idempotency_token_provider(
-        mut self,
-        idempotency_token_provider: impl ::std::convert::Into<crate::idempotency_token::IdempotencyTokenProvider>,
-    ) -> Self {
-        self.set_idempotency_token_provider(::std::option::Option::Some(idempotency_token_provider.into()));
-        self
-    }
-    /// Sets the idempotency token provider to use for service calls that require tokens.
-    pub fn set_idempotency_token_provider(
-        &mut self,
-        idempotency_token_provider: ::std::option::Option<crate::idempotency_token::IdempotencyTokenProvider>,
-    ) -> &mut Self {
-        self.config.store_or_unset(idempotency_token_provider);
-        self
-    }
-    /// Sets the HTTP client to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # #[cfg(test)]
-    /// # mod tests {
-    /// # #[test]
-    /// # fn example() {
-    /// use std::time::Duration;
-    /// use aws_sdk_dynamodb::config::Config;
-    /// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
-    ///
-    /// let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
-    ///     .with_webpki_roots()
-    ///     .https_only()
-    ///     .enable_http1()
-    ///     .enable_http2()
-    ///     .build();
-    /// let hyper_client = HyperClientBuilder::new().build(https_connector);
-    ///
-    /// // This connector can then be given to a generated service Config
-    /// let config = my_service_client::Config::builder()
-    ///     .endpoint_url("https://example.com")
-    ///     .http_client(hyper_client)
-    ///     .build();
-    /// let client = my_service_client::Client::from_conf(config);
-    /// # }
-    /// # }
-    /// ```
-    pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
-        self
-    }
-
-    /// Sets the HTTP client to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # #[cfg(test)]
-    /// # mod tests {
-    /// # #[test]
-    /// # fn example() {
-    /// use std::time::Duration;
-    /// use aws_sdk_dynamodb::config::{Builder, Config};
-    /// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
-    ///
-    /// fn override_http_client(builder: &mut Builder) {
-    ///     let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
-    ///         .with_webpki_roots()
-    ///         .https_only()
-    ///         .enable_http1()
-    ///         .enable_http2()
-    ///         .build();
-    ///     let hyper_client = HyperClientBuilder::new().build(https_connector);
-    ///     builder.set_http_client(Some(hyper_client));
-    /// }
-    ///
-    /// let mut builder = aws_sdk_dynamodb::Config::builder();
-    /// override_http_client(&mut builder);
-    /// let config = builder.build();
-    /// # }
-    /// # }
-    /// ```
-    pub fn set_http_client(&mut self, http_client: Option<crate::config::SharedHttpClient>) -> &mut Self {
-        self.runtime_components.set_http_client(http_client);
-        self
+pub mod config {
+    #[derive(Clone, Debug, Default)]
+    pub struct Builder {
+        endpoint_url: ::std::option::Option<::std::string::String>,
     }
-    /// Adds an auth scheme to the builder
-    ///
-    /// If `auth_scheme` has an existing [AuthSchemeId](aws_smithy_runtime_api::client::auth::AuthSchemeId) in the runtime, the current identity
-    /// resolver and signer for that scheme will be replaced by those from `auth_scheme`.
-    ///
-    /// _Important:_ When introducing a custom auth scheme, ensure you override either
-    /// [`Self::auth_scheme_resolver`] or [`Self::set_auth_scheme_resolver`]
-    /// so that the custom auth scheme is included in the list of resolved auth scheme options.
-    /// [The default auth scheme resolver](crate::config::auth::DefaultAuthSchemeResolver) will not recognize your custom auth scheme.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::{
-    /// #     box_error::BoxError,
-    /// #     client::{
-    /// #         auth::{
-    /// #             AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, AuthSchemeOption,
-    /// #             AuthSchemeOptionsFuture, Sign,
-    /// #         },
-    /// #         identity::{Identity, IdentityFuture, ResolveIdentity, SharedIdentityResolver},
-    /// #         orchestrator::HttpRequest,
-    /// #         runtime_components::{GetIdentityResolver, RuntimeComponents},
-    /// #   },
-    /// #   shared::IntoShared,
-    /// # };
-    /// # use aws_smithy_types::config_bag::ConfigBag;
-    /// // Auth scheme with customer identity resolver and signer
-    /// #[derive(Debug)]
-    /// struct CustomAuthScheme {
-    ///     id: AuthSchemeId,
-    ///     identity_resolver: SharedIdentityResolver,
-    ///     signer: CustomSigner,
-    /// }
-    /// impl Default for CustomAuthScheme {
-    ///     fn default() -> Self {
-    ///         Self {
-    ///             id: AuthSchemeId::new("custom"),
-    ///             identity_resolver: CustomIdentityResolver.into_shared(),
-    ///             signer: CustomSigner,
-    ///         }
-    ///     }
-    /// }
-    /// impl AuthScheme for CustomAuthScheme {
-    ///     fn scheme_id(&self) -> AuthSchemeId {
-    ///         self.id.clone()
-    ///     }
-    ///     fn identity_resolver(
-    ///         &self,
-    ///         _identity_resolvers: &dyn GetIdentityResolver,
-    ///     ) -> Option<SharedIdentityResolver> {
-    ///         Some(self.identity_resolver.clone())
-    ///     }
-    ///     fn signer(&self) -> &dyn Sign {
-    ///         &self.signer
-    ///     }
-    /// }
-    ///
-    /// #[derive(Debug, Default)]
-    /// struct CustomSigner;
-    /// impl Sign for CustomSigner {
-    ///     fn sign_http_request(
-    ///         &self,
-    ///         _request: &mut HttpRequest,
-    ///         _identity: &Identity,
-    ///         _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
-    ///         _runtime_components: &RuntimeComponents,
-    ///         _config_bag: &ConfigBag,
-    ///     ) -> Result<(), BoxError> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// #[derive(Debug)]
-    /// struct CustomIdentityResolver;
-    /// impl ResolveIdentity for CustomIdentityResolver {
-    ///     fn resolve_identity<'a>(
-    ///         &'a self,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///         _config_bag: &'a ConfigBag,
-    ///     ) -> IdentityFuture<'a> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// // Auth scheme resolver that favors `CustomAuthScheme`
-    /// #[derive(Debug)]
-    /// struct CustomAuthSchemeResolver;
-    /// impl aws_sdk_dynamodb::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_dynamodb::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         AuthSchemeOptionsFuture::ready(Ok(vec![AuthSchemeOption::from(AuthSchemeId::new(
-    ///             "custom",
-    ///         ))]))
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .push_auth_scheme(CustomAuthScheme::default())
-    ///     .auth_scheme_resolver(CustomAuthSchemeResolver)
-    ///     // other configurations
-    ///     .build();
-    /// ```
-    pub fn push_auth_scheme(mut self, auth_scheme: impl ::aws_smithy_runtime_api::client::auth::AuthScheme + 'static) -> Self {
-        self.runtime_components.push_auth_scheme(auth_scheme);
-        self
-    }
-
-    /// Set the auth scheme resolver for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::{
-    /// #     client::{
-    /// #         auth::AuthSchemeOptionsFuture,
-    /// #         runtime_components::RuntimeComponents,
-    /// #   },
-    /// # };
-    /// # use aws_smithy_types::config_bag::ConfigBag;
-    /// #[derive(Debug)]
-    /// struct CustomAuthSchemeResolver;
-    /// impl aws_sdk_dynamodb::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_dynamodb::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .auth_scheme_resolver(CustomAuthSchemeResolver)
-    ///     // other configurations
-    ///     .build();
-    /// ```
-    pub fn auth_scheme_resolver(mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> Self {
-        self.set_auth_scheme_resolver(auth_scheme_resolver);
-        self
-    }
-
-    /// Set the auth scheme resolver for the builder
-    ///
-    /// # Examples
-    /// See an example for [`Self::auth_scheme_resolver`].
-    pub fn set_auth_scheme_resolver(&mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> &mut Self {
-        self.runtime_components
-            .set_auth_scheme_option_resolver(::std::option::Option::Some(auth_scheme_resolver.into_shared_resolver()));
-        self
-    }
-
-    /// Enable no authentication regardless of what authentication mechanisms operations support
-    ///
-    /// This adds [NoAuthScheme](aws_smithy_runtime::client::auth::no_auth::NoAuthScheme) as a fallback
-    /// and the auth scheme resolver will use it when no other auth schemes are applicable.
-    pub fn allow_no_auth(mut self) -> Self {
-        self.set_allow_no_auth();
-        self
-    }
-
-    /// Enable no authentication regardless of what authentication mechanisms operations support
-    ///
-    /// This adds [NoAuthScheme](aws_smithy_runtime::client::auth::no_auth::NoAuthScheme) as a fallback
-    /// and the auth scheme resolver will use it when no other auth schemes are applicable.
-    pub fn set_allow_no_auth(&mut self) -> &mut Self {
-        self.push_runtime_plugin(::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePluginV2::new().into_shared());
-        self
-    }
-    /// Set the auth scheme preference for an auth scheme resolver
-    /// (typically the default auth scheme resolver).
-    ///
-    /// Each operation has a predefined order of auth schemes, as determined by the service,
-    /// for auth scheme resolution. By using the auth scheme preference, customers
-    /// can reorder the schemes resolved by the auth scheme resolver.
-    ///
-    /// The preference list is intended as a hint rather than a strict override.
-    /// Any schemes not present in the originally resolved auth schemes will be ignored.
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::client::auth::AuthSchemeId;
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-
-    pub fn auth_scheme_preference(
-        mut self,
-        preference: impl ::std::convert::Into<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
-    ) -> Self {
-        self.set_auth_scheme_preference(::std::option::Option::Some(preference.into()));
-        self
-    }
-
-    /// Set the auth scheme preference for an auth scheme resolver
-    /// (typically the default auth scheme resolver).
-    ///
-    /// Each operation has a predefined order of auth schemes, as determined by the service,
-    /// for auth scheme resolution. By using the auth scheme preference, customers
-    /// can reorder the schemes resolved by the auth scheme resolver.
-    ///
-    /// The preference list is intended as a hint rather than a strict override.
-    /// Any schemes not present in the originally resolved auth schemes will be ignored.
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::client::auth::AuthSchemeId;
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-
-    pub fn set_auth_scheme_preference(
-        &mut self,
-        preference: ::std::option::Option<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
-    ) -> &mut Self {
-        self.config.store_or_unset(preference);
-        self
-    }
-    /// Sets the endpoint resolver to use when making requests.
-    ///
-    ///
-    /// When unset, the client will used a generated endpoint resolver based on the endpoint resolution
-    /// rules for `aws_sdk_dynamodb`.
-    ///
-    ///
-    /// Note: setting an endpoint resolver will replace any endpoint URL that has been set.
-    /// This method accepts an endpoint resolver [specific to this service](crate::config::endpoint::ResolveEndpoint). If you want to
-    /// provide a shared endpoint resolver, use [`Self::set_endpoint_resolver`].
-    ///
-    /// # Examples
-    /// Create a custom endpoint resolver that resolves a different endpoing per-stage, e.g. staging vs. production.
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::endpoint::{ResolveEndpoint, EndpointFuture, Params, Endpoint};
-    /// #[derive(Debug)]
-    /// struct StageResolver { stage: String }
-    /// impl ResolveEndpoint for StageResolver {
-    ///     fn resolve_endpoint(&self, params: &Params) -> EndpointFuture<'_> {
-    ///         let stage = &self.stage;
-    ///         EndpointFuture::ready(Ok(Endpoint::builder().url(format!("{stage}.myservice.com")).build()))
-    ///     }
-    /// }
-    /// let resolver = StageResolver { stage: std::env::var("STAGE").unwrap() };
-    /// let config = aws_sdk_dynamodb::Config::builder().endpoint_resolver(resolver).build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
-        self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
-        self
-    }
-
-    /// Sets the endpoint resolver to use when making requests.
-    ///
-    ///
-    /// When unset, the client will used a generated endpoint resolver based on the endpoint resolution
-    /// rules for `aws_sdk_dynamodb`.
-    ///
-    pub fn set_endpoint_resolver(
-        &mut self,
-        endpoint_resolver: ::std::option::Option<::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver>,
-    ) -> &mut Self {
-        self.runtime_components.set_endpoint_resolver(endpoint_resolver);
-        self
-    }
-    /// Set the retry_config for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::Config;
-    /// use aws_sdk_dynamodb::config::retry::RetryConfig;
-    ///
-    /// let retry_config = RetryConfig::standard().with_max_attempts(5);
-    /// let config = Config::builder().retry_config(retry_config).build();
-    /// ```
-    ///
-    /// # Retry token bucket
-    ///
-    /// [`RetryConfig`](::aws_smithy_types::retry::RetryConfig) controls *how many* times to retry and *how long* to back
-    /// off. Retries are **also** gated by a retry token bucket (also called the retry quota) that
-    /// is shared across a [`RetryPartition`](::aws_smithy_runtime::client::retries::RetryPartition). To configure the token bucket — for
-    /// example, to set
-    /// its capacity or to give a workload its own bucket — see [`Self::retry_partition`] and
-    /// [`RetryPartition::custom`](::aws_smithy_runtime::client::retries::RetryPartition::custom).
-    pub fn retry_config(mut self, retry_config: ::aws_smithy_types::retry::RetryConfig) -> Self {
-        self.set_retry_config(Some(retry_config));
-        self
-    }
-
-    /// Set the retry_config for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::{Builder, Config};
-    /// use aws_sdk_dynamodb::config::retry::RetryConfig;
-    ///
-    /// fn disable_retries(builder: &mut Builder) {
-    ///     let retry_config = RetryConfig::standard().with_max_attempts(1);
-    ///     builder.set_retry_config(Some(retry_config));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// disable_retries(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_retry_config(&mut self, retry_config: ::std::option::Option<::aws_smithy_types::retry::RetryConfig>) -> &mut Self {
-        retry_config.map(|r| self.config.store_put(r));
-        self
-    }
-    /// Set the sleep_impl for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::{AsyncSleep, Config, SharedAsyncSleep, Sleep};
-    ///
-    /// #[derive(Debug)]
-    /// pub struct ForeverSleep;
-    ///
-    /// impl AsyncSleep for ForeverSleep {
-    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
-    ///         Sleep::new(std::future::pending())
-    ///     }
-    /// }
-    ///
-    /// let sleep_impl = SharedAsyncSleep::new(ForeverSleep);
-    /// let config = Config::builder().sleep_impl(sleep_impl).build();
-    /// ```
-    pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
-        self
-    }
-
-    /// Set the sleep_impl for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::{AsyncSleep, Builder, Config, SharedAsyncSleep, Sleep};
-    ///
-    /// #[derive(Debug)]
-    /// pub struct ForeverSleep;
-    ///
-    /// impl AsyncSleep for ForeverSleep {
-    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
-    ///         Sleep::new(std::future::pending())
-    ///     }
-    /// }
-    ///
-    /// fn set_never_ending_sleep_impl(builder: &mut Builder) {
-    ///     let sleep_impl = SharedAsyncSleep::new(ForeverSleep);
-    ///     builder.set_sleep_impl(Some(sleep_impl));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// set_never_ending_sleep_impl(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_sleep_impl(&mut self, sleep_impl: ::std::option::Option<crate::config::SharedAsyncSleep>) -> &mut Self {
-        self.runtime_components.set_sleep_impl(sleep_impl);
-        self
-    }
-    /// Set the timeout_config for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use std::time::Duration;
-    /// use aws_sdk_dynamodb::config::Config;
-    /// use aws_sdk_dynamodb::config::timeout::TimeoutConfig;
-    ///
-    /// let timeout_config = TimeoutConfig::builder()
-    ///     .operation_attempt_timeout(Duration::from_secs(1))
-    ///     .build();
-    /// let config = Config::builder().timeout_config(timeout_config).build();
-    /// ```
-    pub fn timeout_config(mut self, timeout_config: ::aws_smithy_types::timeout::TimeoutConfig) -> Self {
-        self.set_timeout_config(Some(timeout_config));
-        self
-    }
-
-    /// Set the timeout_config for the builder.
-    ///
-    /// Setting this to `None` has no effect if another source of configuration has set timeouts. If you
-    /// are attempting to disable timeouts, use [`TimeoutConfig::disabled`](::aws_smithy_types::timeout::TimeoutConfig::disabled)
-    ///
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use std::time::Duration;
-    /// use aws_sdk_dynamodb::config::{Builder, Config};
-    /// use aws_sdk_dynamodb::config::timeout::TimeoutConfig;
-    ///
-    /// fn set_request_timeout(builder: &mut Builder) {
-    ///     let timeout_config = TimeoutConfig::builder()
-    ///         .operation_attempt_timeout(Duration::from_secs(1))
-    ///         .build();
-    ///     builder.set_timeout_config(Some(timeout_config));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// set_request_timeout(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_timeout_config(&mut self, timeout_config: ::std::option::Option<::aws_smithy_types::timeout::TimeoutConfig>) -> &mut Self {
-        // passing None has no impact.
-        let Some(mut timeout_config) = timeout_config else { return self };
-
-        if let Some(base) = self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>() {
-            timeout_config.take_defaults_from(base);
+    impl Builder {
+        pub fn endpoint_url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
+            self.endpoint_url = Some(value.into());
+            self
         }
-        self.config.store_put(timeout_config);
-        self
-    }
-    /// Set the partition for retry-related state. When clients share a retry partition, they will
-    /// also share components such as token buckets and client rate limiters.
-    /// See the [`RetryPartition`](::aws_smithy_runtime::client::retries::RetryPartition) documentation for more details.
-    ///
-    /// # Default Behavior
-    ///
-    /// When no retry partition is explicitly set, the SDK automatically creates a default retry partition named `dynamodb`
-    /// (or `dynamodb-<region>` if a region is configured).
-    /// All DynamoDB clients without an explicit retry partition will share this default partition.
-    ///
-    /// # Notes
-    ///
-    /// - This is an advanced setting. A common reason to set it is to size or isolate the retry
-    ///   token bucket — for example, giving a high-throughput workload its own bucket. Otherwise
-    ///   most users won't need to modify it.
-    /// - A configured client rate limiter has no effect unless [`RetryConfig::adaptive`](::aws_smithy_types::retry::RetryConfig::adaptive) is used.
-    ///
-    /// # Examples
-    ///
-    /// Creating a custom retry partition with a token bucket:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::Config;
-    /// use aws_sdk_dynamodb::config::retry::{RetryPartition, TokenBucket};
-    ///
-    /// let token_bucket = TokenBucket::new(10);
-    /// let config = Config::builder()
-    ///     .retry_partition(RetryPartition::custom("custom")
-    ///         .token_bucket(token_bucket)
-    ///         .build()
-    ///     )
-    ///     .build();
-    /// ```
-    ///
-    /// Sizing the retry token bucket (for example, for a high-throughput workload), or giving a
-    /// workload its own bucket:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::Config;
-    /// use aws_sdk_dynamodb::config::retry::{RetryPartition, TokenBucket};
-    ///
-    /// let config = Config::builder()
-    ///     .retry_partition(
-    ///         RetryPartition::custom("high-throughput")
-    ///             .token_bucket(TokenBucket::builder().capacity(5000).build())
-    ///             .build(),
-    ///     )
-    ///     .build();
-    /// ```
-    ///
-    /// Configuring a client rate limiter with adaptive retry mode:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::Config;
-    /// use aws_sdk_dynamodb::config::retry::{ClientRateLimiter, RetryConfig, RetryPartition};
-    ///
-    /// let client_rate_limiter = ClientRateLimiter::new(10.0);
-    /// let config = Config::builder()
-    ///     .retry_partition(RetryPartition::custom("custom")
-    ///         .client_rate_limiter(client_rate_limiter)
-    ///         .build()
-    ///     )
-    ///     .retry_config(RetryConfig::adaptive())
-    ///     .build();
-    /// ```
-    pub fn retry_partition(mut self, retry_partition: ::aws_smithy_runtime::client::retries::RetryPartition) -> Self {
-        self.set_retry_partition(Some(retry_partition));
-        self
-    }
-    /// Like [`Self::retry_partition`], but takes a mutable reference to the builder and an optional `RetryPartition`
-    pub fn set_retry_partition(
-        &mut self,
-        retry_partition: ::std::option::Option<::aws_smithy_runtime::client::retries::RetryPartition>,
-    ) -> &mut Self {
-        retry_partition.map(|r| self.config.store_put(r));
-        self
-    }
-    /// Set the identity cache for auth.
-    ///
-    /// The identity cache defaults to a lazy caching implementation that will resolve
-    /// an identity when it is requested, and place it in the cache thereafter. Subsequent
-    /// requests will take the value from the cache while it is still valid. Once it expires,
-    /// the next request will result in refreshing the identity.
-    ///
-    /// This configuration allows you to disable or change the default caching mechanism.
-    /// To use a custom caching mechanism, implement the [`ResolveCachedIdentity`](crate::config::ResolveCachedIdentity)
-    /// trait and pass that implementation into this function.
-    ///
-    /// # Examples
-    ///
-    /// Disabling identity caching:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn identity_cache(mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> Self {
-        self.set_identity_cache(identity_cache);
-        self
-    }
-
-    /// Set the identity cache for auth.
-    ///
-    /// The identity cache defaults to a lazy caching implementation that will resolve
-    /// an identity when it is requested, and place it in the cache thereafter. Subsequent
-    /// requests will take the value from the cache while it is still valid. Once it expires,
-    /// the next request will result in refreshing the identity.
-    ///
-    /// This configuration allows you to disable or change the default caching mechanism.
-    /// To use a custom caching mechanism, implement the [`ResolveCachedIdentity`](crate::config::ResolveCachedIdentity)
-    /// trait and pass that implementation into this function.
-    ///
-    /// # Examples
-    ///
-    /// Disabling identity caching:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn set_identity_cache(&mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> &mut Self {
-        self.runtime_components.set_identity_cache(::std::option::Option::Some(identity_cache));
-        self
-    }
-    /// Add an [interceptor](crate::config::Intercept) that runs at specific stages of the request execution pipeline.
-    ///
-    /// Interceptors targeted at a certain stage are executed according to the pre-defined priority.
-    /// The SDK provides a default set of interceptors. An interceptor configured by this method
-    /// will run after those default interceptors.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # fn example() {
-    /// use aws_smithy_runtime_api::box_error::BoxError;
-    /// use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextMut;
-    /// use aws_smithy_runtime_api::client::interceptors::Intercept;
-    /// use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
-    /// use aws_smithy_types::config_bag::ConfigBag;
-    /// use aws_sdk_dynamodb::config::Config;
-    /// use ::http::uri::Uri;
-    ///
-    /// fn base_url() -> String {
-    ///     // ...
-    ///     # String::new()
-    /// }
-    ///
-    /// #[derive(Debug)]
-    /// pub struct UriModifierInterceptor;
-    /// impl Intercept for UriModifierInterceptor {
-    ///     fn name(&self) -> &'static str {
-    ///         "UriModifierInterceptor"
-    ///     }
-    ///     fn modify_before_signing(
-    ///         &self,
-    ///         context: &mut BeforeTransmitInterceptorContextMut<'_>,
-    ///         _runtime_components: &RuntimeComponents,
-    ///         _cfg: &mut ConfigBag,
-    ///     ) -> Result<(), BoxError> {
-    ///         let request = context.request_mut();
-    ///         let uri = format!("{}{}", base_url(), request.uri());
-    ///         *request.uri_mut() = uri.parse::<Uri>()?.into();
-    ///
-    ///         Ok(())
-    ///     }
-    /// }
-    ///
-    /// let config = Config::builder()
-    ///     .interceptor(UriModifierInterceptor)
-    ///     .build();
-    /// # }
-    /// ```
-    pub fn interceptor(mut self, interceptor: impl crate::config::Intercept + 'static) -> Self {
-        self.push_interceptor(crate::config::SharedInterceptor::new(interceptor));
-        self
-    }
-
-    /// Like [`Self::interceptor`], but takes a [`SharedInterceptor`](crate::config::SharedInterceptor).
-    pub fn push_interceptor(&mut self, interceptor: crate::config::SharedInterceptor) -> &mut Self {
-        self.runtime_components.push_interceptor(interceptor);
-        self
-    }
-
-    /// Set [`SharedInterceptor`](crate::config::SharedInterceptor)s for the builder.
-    pub fn set_interceptors(&mut self, interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>) -> &mut Self {
-        self.runtime_components.set_interceptors(interceptors.into_iter());
-        self
-    }
-    /// Sets the time source used for this service
-    pub fn time_source(mut self, time_source: impl ::aws_smithy_async::time::TimeSource + 'static) -> Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
-            time_source,
-        )));
-        self
-    }
-    /// Sets the time source used for this service
-    pub fn set_time_source(&mut self, time_source: ::std::option::Option<::aws_smithy_async::time::SharedTimeSource>) -> &mut Self {
-        self.runtime_components.set_time_source(time_source);
-        self
-    }
-    /// Add type implementing [`ClassifyRetry`](::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry) that will be used by the
-    /// [`RetryStrategy`](::aws_smithy_runtime_api::client::retries::RetryStrategy) to determine what responses should be retried.
-    ///
-    /// A retry classifier configured by this method will run according to its [priority](::aws_smithy_runtime_api::client::retries::classifiers::RetryClassifierPriority).
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # fn example() {
-    /// use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
-    /// use aws_smithy_runtime_api::client::orchestrator::OrchestratorError;
-    /// use aws_smithy_runtime_api::client::retries::classifiers::{
-    ///     ClassifyRetry, RetryAction, RetryClassifierPriority,
-    /// };
-    /// use aws_smithy_types::error::metadata::ProvideErrorMetadata;
-    /// use aws_smithy_types::retry::ErrorKind;
-    /// use std::error::Error as StdError;
-    /// use std::marker::PhantomData;
-    /// use std::fmt;
-    /// use aws_sdk_dynamodb::config::Config;
-    /// # #[derive(Debug)]
-    /// # struct SomeOperationError {}
-    /// # impl StdError for SomeOperationError {}
-    /// # impl fmt::Display for SomeOperationError {
-    /// #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
-    /// # }
-    /// # impl ProvideErrorMetadata for SomeOperationError {
-    /// #    fn meta(&self) -> &aws_sdk_dynamodb::error::ErrorMetadata { todo!() }
-    /// # }
-    ///
-    /// const RETRYABLE_ERROR_CODES: &[&str] = &[
-    ///     // List error codes to be retried here...
-    /// ];
-    ///
-    /// // When classifying at an operation's error type, classifiers require a generic parameter.
-    /// // When classifying the HTTP response alone, no generic is needed.
-    /// #[derive(Debug, Default)]
-    /// pub struct ExampleErrorCodeClassifier<E> {
-    ///     _inner: PhantomData<E>,
-    /// }
-    ///
-    /// impl<E> ExampleErrorCodeClassifier<E> {
-    ///     pub fn new() -> Self {
-    ///         Self {
-    ///             _inner: PhantomData,
-    ///         }
-    ///     }
-    /// }
-    ///
-    /// impl<E> ClassifyRetry for ExampleErrorCodeClassifier<E>
-    /// where
-    ///     // Adding a trait bound for ProvideErrorMetadata allows us to inspect the error code.
-    ///     E: StdError + ProvideErrorMetadata + Send + Sync + 'static,
-    /// {
-    ///     fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
-    ///         // Check for a result
-    ///         let output_or_error = ctx.output_or_error();
-    ///         // Check for an error
-    ///         let error = match output_or_error {
-    ///             Some(Ok(_)) | None => return RetryAction::NoActionIndicated,
-    ///               Some(Err(err)) => err,
-    ///         };
-    ///
-    ///         // Downcast the generic error and extract the code
-    ///         let error_code = OrchestratorError::as_operation_error(error)
-    ///             .and_then(|err| err.downcast_ref::<E>())
-    ///             .and_then(|err| err.code());
-    ///
-    ///         // If this error's code is in our list, return an action that tells the RetryStrategy to retry this request.
-    ///         if let Some(error_code) = error_code {
-    ///             if RETRYABLE_ERROR_CODES.contains(&error_code) {
-    ///                 return RetryAction::transient_error();
-    ///             }
-    ///         }
-    ///
-    ///         // Otherwise, return that no action is indicated i.e. that this classifier doesn't require a retry.
-    ///         // Another classifier may still classify this response as retryable.
-    ///         RetryAction::NoActionIndicated
-    ///     }
-    ///
-    ///     fn name(&self) -> &'static str { "Example Error Code Classifier" }
-    /// }
-    ///
-    /// let config = Config::builder()
-    ///     .retry_classifier(ExampleErrorCodeClassifier::<SomeOperationError>::new())
-    ///     .build();
-    /// # }
-    /// ```
-    pub fn retry_classifier(
-        mut self,
-        retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry + 'static,
-    ) -> Self {
-        self.push_retry_classifier(::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
-            retry_classifier,
-        ));
-        self
-    }
-
-    /// Like [`Self::retry_classifier`], but takes a [`SharedRetryClassifier`](::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier).
-    pub fn push_retry_classifier(
-        &mut self,
-        retry_classifier: ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
-    ) -> &mut Self {
-        self.runtime_components.push_retry_classifier(retry_classifier);
-        self
-    }
-
-    /// Set [`SharedRetryClassifier`](::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier)s for the builder, replacing any that
-    /// were previously set.
-    pub fn set_retry_classifiers(
-        &mut self,
-        retry_classifiers: impl IntoIterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier>,
-    ) -> &mut Self {
-        self.runtime_components.set_retry_classifiers(retry_classifiers.into_iter());
-        self
-    }
-    /// Sets the name of the app that is using the client.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn app_name(mut self, app_name: ::aws_types::app_name::AppName) -> Self {
-        self.set_app_name(Some(app_name));
-        self
-    }
-    /// Sets the name of the app that is using the client.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn set_app_name(&mut self, app_name: ::std::option::Option<::aws_types::app_name::AppName>) -> &mut Self {
-        self.config.store_or_unset(app_name);
-        self
-    }
-    /// Appends framework metadata to the user agent.
-    ///
-    /// This _optional_ metadata identifies a software framework or third-party library
-    /// that is being used with the client. It is rendered into the user agent string
-    /// (as `lib/{name}/{version}`) so that libraries built on top of the AWS SDK can
-    /// self-identify in the requests they make. Multiple entries may be added; each call
-    /// appends another entry rather than replacing previous ones.
-    ///
-    /// Entries are de-duplicated on `(name, version)`, rendered in first-seen order, and
-    /// the total number of unique entries included in the user agent is capped (currently
-    /// at 10); additional entries beyond the cap are dropped with a warning.
-    pub fn framework_metadata(mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> Self {
-        self.push_framework_metadata(framework_metadata);
-        self
-    }
-    /// Appends framework metadata to the user agent.
-    ///
-    /// This _optional_ metadata identifies a software framework or third-party library
-    /// that is being used with the client. It is rendered into the user agent string
-    /// (as `lib/{name}/{version}`) so that libraries built on top of the AWS SDK can
-    /// self-identify in the requests they make. Multiple entries may be added; each call
-    /// appends another entry rather than replacing previous ones.
-    pub fn push_framework_metadata(&mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> &mut Self {
-        self.config.store_append(framework_metadata);
-        self
-    }
-    /// Overrides the default invocation ID generator.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(mut self, gen: impl ::aws_runtime::invocation_id::InvocationIdGenerator + 'static) -> Self {
-        self.set_invocation_id_generator(::std::option::Option::Some(
-            ::aws_runtime::invocation_id::SharedInvocationIdGenerator::new(gen),
-        ));
-        self
-    }
-    /// Overrides the default invocation ID generator.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn set_invocation_id_generator(
-        &mut self,
-        gen: ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator>,
-    ) -> &mut Self {
-        self.config.store_or_unset(gen);
-        self
-    }
-    /// The AccountId Endpoint Mode.
-    pub fn account_id_endpoint_mode(mut self, account_id_endpoint_mode: ::aws_types::endpoint_config::AccountIdEndpointMode) -> Self {
-        self.set_account_id_endpoint_mode(::std::option::Option::Some(account_id_endpoint_mode));
-        self
-    }
-    /// The AccountId Endpoint Mode.
-    pub fn set_account_id_endpoint_mode(
-        &mut self,
-        account_id_endpoint_mode: ::std::option::Option<::aws_types::endpoint_config::AccountIdEndpointMode>,
-    ) -> &mut Self {
-        self.config.store_or_unset(account_id_endpoint_mode);
-        self
-    }
-    /// Sets the endpoint URL used to communicate with this service.
-    ///
-    /// Note: this is used in combination with other endpoint rules, e.g. an API that applies a host-label prefix
-    /// will be prefixed onto this URL. To fully override the endpoint resolver, use
-    /// [`Builder::endpoint_resolver`].
-    pub fn endpoint_url(mut self, endpoint_url: impl Into<::std::string::String>) -> Self {
-        self.set_endpoint_url(Some(endpoint_url.into()));
-        self
-    }
-    /// Sets the endpoint URL used to communicate with this service.
-    ///
-    /// Note: this is used in combination with other endpoint rules, e.g. an API that applies a host-label prefix
-    /// will be prefixed onto this URL. To fully override the endpoint resolver, use
-    /// [`Builder::endpoint_resolver`].
-    pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
-        self
-    }
-    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
-    pub fn use_dual_stack(mut self, use_dual_stack: impl Into<bool>) -> Self {
-        self.set_use_dual_stack(Some(use_dual_stack.into()));
-        self
-    }
-    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
-    pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
-        self
-    }
-    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
-    pub fn use_fips(mut self, use_fips: impl Into<bool>) -> Self {
-        self.set_use_fips(Some(use_fips.into()));
-        self
-    }
-    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
-    pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
-        self
-    }
-    /// Sets the AWS region to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_types::region::Region;
-    /// use aws_sdk_dynamodb::config::{Builder, Config};
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .region(Region::new("us-east-1"))
-    ///     .build();
-    /// ```
-    pub fn region(mut self, region: impl ::std::convert::Into<::std::option::Option<crate::config::Region>>) -> Self {
-        self.set_region(region.into());
-        self
-    }
-    /// Sets the AWS region to use when making requests.
-    pub fn set_region(&mut self, region: ::std::option::Option<crate::config::Region>) -> &mut Self {
-        self.config.store_or_unset(region);
-        self
-    }
-    /// Sets the credentials provider for this service
-    pub fn credentials_provider(mut self, credentials_provider: impl crate::config::ProvideCredentials + 'static) -> Self {
-        self.set_credentials_provider(::std::option::Option::Some(crate::config::SharedCredentialsProvider::new(
-            credentials_provider,
-        )));
-        self
-    }
-    /// Sets the credentials provider for this service
-    pub fn set_credentials_provider(&mut self, credentials_provider: ::std::option::Option<crate::config::SharedCredentialsProvider>) -> &mut Self {
-        if let Some(credentials_provider) = credentials_provider {
-            self.runtime_components
-                .set_identity_resolver(::aws_runtime::auth::sigv4::SCHEME_ID, credentials_provider);
+        pub fn build(self) -> super::Config {
+            super::Config {
+                endpoint_url: self.endpoint_url.unwrap_or_else(|| super::Config::default().endpoint_url),
+            }
         }
-        self
     }
-    /// Sets the [`behavior major version`](crate::config::BehaviorVersion).
-    ///
-    /// Over time, new best-practice behaviors are introduced. However, these behaviors might not be backwards
-    /// compatible. For example, a change which introduces new default timeouts or a new retry-mode for
-    /// all operations might be the ideal behavior but could break existing applications.
-    ///
-    /// # Examples
-    ///
-    /// Set the behavior major version to `latest`. This is equivalent to enabling the `behavior-version-latest` cargo feature.
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn behavior_version(mut self, behavior_version: crate::config::BehaviorVersion) -> Self {
-        self.set_behavior_version(Some(behavior_version));
-        self
-    }
-
-    /// Sets the [`behavior major version`](crate::config::BehaviorVersion).
-    ///
-    /// Over time, new best-practice behaviors are introduced. However, these behaviors might not be backwards
-    /// compatible. For example, a change which introduces new default timeouts or a new retry-mode for
-    /// all operations might be the ideal behavior but could break existing applications.
-    ///
-    /// # Examples
-    ///
-    /// Set the behavior major version to `latest`. This is equivalent to enabling the `behavior-version-latest` cargo feature.
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_dynamodb::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_dynamodb::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_dynamodb::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn set_behavior_version(&mut self, behavior_version: Option<crate::config::BehaviorVersion>) -> &mut Self {
-        self.behavior_version = behavior_version;
-        self
-    }
-
-    /// Convenience method to set the latest behavior major version
-    ///
-    /// This is equivalent to enabling the `behavior-version-latest` Cargo feature
-    pub fn behavior_version_latest(mut self) -> Self {
-        self.set_behavior_version(Some(crate::config::BehaviorVersion::latest()));
-        self
-    }
-    /// Adds a runtime plugin to the config.
-    #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, plugin: impl crate::config::RuntimePlugin + 'static) -> Self {
-        self.push_runtime_plugin(crate::config::SharedRuntimePlugin::new(plugin));
-        self
-    }
-    /// Adds a runtime plugin to the config.
-    #[allow(unused)]
-    pub(crate) fn push_runtime_plugin(&mut self, plugin: crate::config::SharedRuntimePlugin) -> &mut Self {
-        self.runtime_plugins.push(plugin);
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. NOTE: Consider migrating to use `apply_test_defaults_v2` instead.
-    pub fn apply_test_defaults(&mut self) -> &mut Self {
-        self.set_idempotency_token_provider(Some("00000000-0000-4000-8000-000000000000".into()));
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_async::time::SharedTimeSource::new(
-            ::aws_smithy_async::time::StaticTimeSource::new(::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890)),
-        )));
-        self.config.store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
-        self.set_credentials_provider(Some(crate::config::SharedCredentialsProvider::new(
-            ::aws_credential_types::Credentials::for_tests(),
-        )));
-        self.behavior_version = ::std::option::Option::Some(crate::config::BehaviorVersion::latest());
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. NOTE: Consider migrating to use `with_test_defaults_v2` instead.
-    pub fn with_test_defaults(mut self) -> Self {
-        self.apply_test_defaults();
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. V2 of this function sets additional test defaults such as region configuration (if applicable).
-    pub fn apply_test_defaults_v2(&mut self) -> &mut Self {
-        self.apply_test_defaults();
-        if self.config.load::<crate::config::Region>().is_none() {
-            self.set_region(::std::option::Option::Some(crate::config::Region::new("us-east-1")));
-        }
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. V2 of this function sets additional test defaults such as region configuration (if applicable).
-    pub fn with_test_defaults_v2(mut self) -> Self {
-        self.apply_test_defaults_v2();
-        self
-    }
-    /// Builds a [`Config`].
-    #[allow(unused_mut)]
-    pub fn build(mut self) -> Config {
-        let mut layer = self.config;
-        if self.runtime_components.time_source().is_none() {
-            self.runtime_components
-                .set_time_source(::std::option::Option::Some(::std::default::Default::default()));
-        }
-        layer.store_put(crate::meta::API_METADATA.clone());
-        layer.store_put(::aws_types::SigningName::from_static("dynamodb"));
-        layer
-            .load::<::aws_types::region::Region>()
-            .cloned()
-            .map(|r| layer.store_put(::aws_types::region::SigningRegion::from(r)));
-        Config {
-            config: crate::config::Layer::from(layer.clone())
-                .with_name("aws_sdk_dynamodb::config::Config")
-                .freeze(),
-            cloneable: layer,
-            runtime_components: self.runtime_components,
-            runtime_plugins: self.runtime_plugins,
-            behavior_version: self.behavior_version,
-        }
-    }
-}
-#[derive(::std::fmt::Debug)]
-pub(crate) struct ServiceRuntimePlugin {
-    config: ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer>,
-    runtime_components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-}
-
-impl ServiceRuntimePlugin {
-    pub fn new(_service_config: crate::config::Config) -> Self {
-        let config = {
-            let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DynamoDB_20120810");
-            cfg.store_put(crate::idempotency_token::default_provider());
-            cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
-            ::std::option::Option::Some(cfg.freeze())
-        };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
-        runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
-            use crate::config::auth::ResolveAuthScheme;
-            crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
-        }));
-        runtime_components.set_endpoint_resolver(::std::option::Option::Some({
-            use crate::config::endpoint::ResolveEndpoint;
-            crate::config::endpoint::DefaultResolver::new().into_shared_resolver()
-        }));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_smithy_runtime::client::http::connection_poisoning::ConnectionPoisoningInterceptor::new(),
-        ));
-        runtime_components.push_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::HttpStatusCodeClassifier::default());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::sdk_feature_tracker::retry_mode::RetryModeFeatureTrackerInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::service_clock_skew::ServiceClockSkewInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
-        runtime_components.push_interceptor(::aws_runtime::user_agent::UserAgentInterceptor::new());
-        runtime_components.push_interceptor(::aws_runtime::invocation_id::InvocationIdInterceptor::new());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::account_id_endpoint::AccountIdEndpointFeatureTrackerInterceptor,
-        ));
-        runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
-            ::aws_runtime::auth::sigv4::SigV4AuthScheme::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::config::endpoint::EndpointOverrideFeatureTrackerInterceptor,
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::observability_feature::ObservabilityFeatureTrackerInterceptor,
-        ));
-        Self { config, runtime_components }
-    }
-}
-
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ServiceRuntimePlugin {
-    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
-        self.config.clone()
-    }
-
-    fn order(&self) -> ::aws_smithy_runtime_api::client::runtime_plugin::Order {
-        ::aws_smithy_runtime_api::client::runtime_plugin::Order::Defaults
-    }
-
-    fn runtime_components(
-        &self,
-        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
-        ::std::borrow::Cow::Borrowed(&self.runtime_components)
-    }
-}
-
-// Cross-operation shared-state singletons
-
-/// A plugin that enables configuration for a single operation invocation
-///
-/// The `config` method will return a `FrozenLayer` by storing values from `config_override`.
-/// In the case of default values requested, they will be obtained from `client_config`.
-#[derive(Debug)]
-pub(crate) struct ConfigOverrideRuntimePlugin {
-    pub(crate) config: ::aws_smithy_types::config_bag::FrozenLayer,
-    pub(crate) components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-}
-
-impl ConfigOverrideRuntimePlugin {
-    #[allow(dead_code)] // unused when a service does not provide any operations
-    pub(crate) fn new(
-        config_override: Builder,
-        initial_config: ::aws_smithy_types::config_bag::FrozenLayer,
-        initial_components: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> Self {
-        let mut layer = config_override.config;
-        let mut components = config_override.runtime_components;
-        #[allow(unused_mut)]
-        let mut resolver =
-            ::aws_smithy_runtime::client::config_override::Resolver::overrid(initial_config, initial_components, &mut layer, &mut components);
-
-        resolver
-            .config_mut()
-            .load::<::aws_types::region::Region>()
-            .cloned()
-            .map(|r| resolver.config_mut().store_put(::aws_types::region::SigningRegion::from(r)));
-
-        let _ = resolver;
-
-        // When the config override supplies an identity resolver for any auth scheme
-        // known to the client or the override itself, we give this operation its own
-        // short-lived identity cache so that new partitions don't accumulate in the
-        // shared client cache. A lazy cache (not `no_cache`) is used so that resolved
-        // identities are served from the short-lived identity cache on retries.
-        //
-        // This is skipped if the override already sets its own identity cache.
-        if components.has_identity_resolvers() && components.identity_cache().is_none() {
-            components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
-            ));
-        }
-
-        Self {
-            config: ::aws_smithy_types::config_bag::Layer::from(layer)
-                .with_name("aws_sdk_dynamodb::config::ConfigOverrideRuntimePlugin")
-                .freeze(),
-            components,
-        }
-    }
-}
-
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ConfigOverrideRuntimePlugin {
-    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
-        Some(self.config.clone())
-    }
-
-    fn runtime_components(
-        &self,
-        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
-        ::std::borrow::Cow::Borrowed(&self.components)
-    }
-}
-
-pub use ::aws_smithy_runtime::client::identity::IdentityCache;
-pub use ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
-pub use ::aws_smithy_types::config_bag::ConfigBag;
-
-pub use ::aws_credential_types::Credentials;
-
-impl From<&::aws_types::sdk_config::SdkConfig> for Builder {
-    fn from(input: &::aws_types::sdk_config::SdkConfig) -> Self {
-        let mut builder = Builder::default();
-        builder.set_credentials_provider(input.credentials_provider());
-        builder = builder.region(input.region().cloned());
-        builder.set_use_fips(input.use_fips());
-        builder.set_use_dual_stack(input.use_dual_stack());
-        if input.get_origin("endpoint_url").is_client_config() {
-            builder.set_endpoint_url(input.endpoint_url().map(|s| s.to_string()));
-        } else {
-            builder.set_endpoint_url(
-                input
-                    .service_config()
-                    .and_then(|conf| {
-                        conf.load_config(service_config_key("DynamoDB", "AWS_ENDPOINT_URL", "endpoint_url"))
-                            .map(|it| it.parse().unwrap())
-                    })
-                    .or_else(|| input.endpoint_url().map(|s| s.to_string())),
-            );
-        }
-        builder.set_account_id_endpoint_mode(input.account_id_endpoint_mode().cloned());
-        // resiliency
-        builder.set_retry_config(input.retry_config().cloned());
-        builder.set_timeout_config(input.timeout_config().cloned());
-        builder.set_sleep_impl(input.sleep_impl());
-
-        builder.set_http_client(input.http_client());
-        builder.set_time_source(input.time_source());
-        builder.set_behavior_version(input.behavior_version());
-        builder.set_auth_scheme_preference(input.auth_scheme_preference().cloned());
-        // setting `None` here removes the default
-        if let Some(config) = input.stalled_stream_protection() {
-            builder.set_stalled_stream_protection(Some(config));
-        }
-
-        if let Some(cache) = input.identity_cache() {
-            builder.set_identity_cache(cache);
-        }
-        if let ::std::option::Option::Some(existing_rc) = input.retry_config().cloned() {
-            if existing_rc
-                .retry_spec()
-                .is_some_and(|s| s.is_at_least(::aws_smithy_types::retry::RetrySpec::V2_1))
-            {
-                let mut rc = existing_rc.with_retry_spec(
-                    ::aws_smithy_types::retry::RetrySpec::v2_1().with_non_throttling_initial_backoff(::std::time::Duration::from_millis(25)),
-                );
-                if !input.get_origin("retry_config").is_client_config() {
-                    rc = rc.with_max_attempts(4);
-                }
-                builder = builder.retry_config(rc);
+    impl From<&super::Config> for Builder {
+        fn from(config: &super::Config) -> Self {
+            Self {
+                endpoint_url: Some(config.endpoint_url.clone()),
             }
         }
-        builder.set_app_name(input.app_name().cloned());
-        for framework_metadata in input.framework_metadata() {
-            builder.push_framework_metadata(framework_metadata.clone());
-        }
-
-        builder
-    }
-}
-
-impl From<&::aws_types::sdk_config::SdkConfig> for Config {
-    fn from(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Builder::from(sdk_config).build()
     }
 }

-pub use ::aws_types::app_name::AppName;
-pub use ::aws_types::sdk_ua_metadata::FrameworkMetadata;
-
-#[allow(dead_code)]
-fn service_config_key<'a>(service_id: &'a str, env: &'a str, profile: &'a str) -> aws_types::service_config::ServiceConfigKey<'a> {
-    ::aws_types::service_config::ServiceConfigKey::builder()
-        .service_id(service_id)
-        .env(env)
-        .profile(profile)
-        .build()
-        .expect("all field sets explicitly, can't fail")
-}
-
-pub use ::aws_smithy_async::rt::sleep::Sleep;
-
-pub(crate) fn base_client_runtime_plugins(mut config: crate::Config) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
-    let mut configured_plugins = ::std::vec::Vec::new();
-    ::std::mem::swap(&mut config.runtime_plugins, &mut configured_plugins);
-    #[cfg(feature = "behavior-version-latest")]
-    {
-        if config.behavior_version.is_none() {
-            config.behavior_version = Some(::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion::latest());
-        }
-    }
-
-    let default_retry_partition = "dynamodb";
-    let default_retry_partition = match config.region() {
-        Some(region) => ::std::borrow::Cow::from(format!("{default_retry_partition}-{region}")),
-        None => ::std::borrow::Cow::from(default_retry_partition),
-    };
-
-    let scope = "aws-sdk-dynamodb";
-
-    #[allow(deprecated)]
-                    let mut plugins = ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins::new()
-                        // defaults
-                        .with_client_plugins(::aws_smithy_runtime::client::defaults::default_plugins(
-                            ::aws_smithy_runtime::client::defaults::DefaultPluginParams::new()
-                                .with_retry_partition_name(default_retry_partition)
-                                .with_behavior_version(config.behavior_version.expect("Invalid client configuration: A behavior major version must be set when sending a request or constructing a client. You must set it during client construction or by enabling the `behavior-version-latest` cargo feature."))
-                                .with_is_aws_sdk(true)
-                        ))
-                        // user config
-                        .with_client_plugin(
-                            ::aws_smithy_runtime_api::client::runtime_plugin::StaticRuntimePlugin::new()
-                                .with_config(config.config.clone())
-                                .with_runtime_components(config.runtime_components.clone())
-                        )
-                        // codegen config
-                        .with_client_plugin(crate::config::ServiceRuntimePlugin::new(config.clone()))
-                        .with_client_plugin(::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePlugin::new())
-                        .with_client_plugin(
-                            ::aws_smithy_runtime::client::metrics::MetricsRuntimePlugin::builder()
-                                .with_scope(scope)
-                                .with_time_source(config.runtime_components.time_source().unwrap_or_default())
-                                .build()
-                                .expect("All required fields have been set")
-                        );
-
-    for plugin in configured_plugins {
-        plugins = plugins.with_client_plugin(plugin);
+impl Config {
+    pub fn builder() -> config::Builder {
+        config::Builder::default()
     }
-    plugins
 }
-
-pub use ::aws_smithy_types::config_bag::FrozenLayer;
-
-pub use ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
-
-pub use ::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin;
-
-pub use ::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
-
-pub use ::aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
-
-pub use ::aws_smithy_runtime_api::client::http::SharedHttpClient;
-
-pub use ::aws_smithy_async::rt::sleep::SharedAsyncSleep;
-
-pub use ::aws_smithy_runtime_api::client::identity::SharedIdentityCache;
-
-pub use ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor;
-
-pub use ::aws_types::region::Region;
-
-pub use ::aws_credential_types::provider::SharedCredentialsProvider;
-
-pub use ::aws_smithy_runtime_api::client::http::HttpClient;
-
-pub use ::aws_smithy_runtime_api::shared::IntoShared;
-
-pub use ::aws_smithy_async::rt::sleep::AsyncSleep;
-
-pub use ::aws_smithy_runtime_api::client::identity::ResolveCachedIdentity;
-
-pub use ::aws_smithy_runtime_api::client::interceptors::Intercept;
-
-pub use ::aws_credential_types::provider::ProvideCredentials;
-
-pub use ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
-
-pub use ::aws_smithy_types::config_bag::Layer;
-
-/// Types needed to configure endpoint resolution.
-pub mod endpoint;
-
-/// HTTP request and response types.
-pub mod http;
-
-/// Types needed to implement [`Intercept`](crate::config::Intercept).
-pub mod interceptors;
-
-/// Retry configuration.
-///
-/// [`RetryConfig`](crate::config::retry::RetryConfig) sets the number of retry attempts and the backoff between them. Retries are additionally bounded by a retry token bucket (a shared retry quota): [`TokenBucket`](crate::config::retry::TokenBucket) holds the tokens and [`RetryPartition`](crate::config::retry::RetryPartition) determines which clients and operations share one. To size the token bucket or give a workload its own, use [`Builder::retry_partition`](crate::config::Builder::retry_partition).
-pub mod retry;
-
-/// Timeout configuration.
-pub mod timeout;
-
-/// Types needed to configure auth scheme resolution.
-pub mod auth;
```

### `src/error.rs`

```diff
--- reference/src/error.rs
+++ generated/src/error.rs
@@ -1,17 +1,46 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use ::aws_smithy_runtime_api::box_error::BoxError;

-/// Error type returned by the client.
-pub type SdkError<E, R = ::aws_smithy_runtime_api::client::orchestrator::HttpResponse> = ::aws_smithy_runtime_api::client::result::SdkError<E, R>;
-pub use ::aws_smithy_runtime_api::client::result::ConnectorError;
-pub use ::aws_smithy_types::error::operation::BuildError;
-
-pub use ::aws_smithy_types::error::display::DisplayErrorContext;
-pub use ::aws_smithy_types::error::metadata::ErrorMetadata;
-pub use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
-
-/// The given enum value failed to parse since it is not a known value.
-#[derive(Debug)]
+#[derive(Clone, Debug)]
+pub struct Error {
+    message: ::std::string::String,
+}
+impl Error {
+    pub fn unhandled(message: impl ::std::convert::Into<::std::string::String>) -> Self {
+        Self { message: message.into() }
+    }
+    pub fn meta(&self) -> ErrorMetadata {
+        ErrorMetadata::default()
+    }
+}
+impl ::std::fmt::Display for Error {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        f.write_str(&self.message)
+    }
+}
+impl ::std::error::Error for Error {}
+#[derive(Clone, Debug, Default)]
+pub struct ErrorMetadata {
+    request_id: ::std::option::Option<::std::string::String>,
+    extended_request_id: ::std::option::Option<::std::string::String>,
+}
+impl ErrorMetadata {
+    pub(crate) fn from_request_ids(
+        request_id: ::std::option::Option<::std::string::String>,
+        extended_request_id: ::std::option::Option<::std::string::String>,
+    ) -> Self {
+        Self {
+            request_id,
+            extended_request_id,
+        }
+    }
+    pub fn request_id(&self) -> ::std::option::Option<&str> {
+        self.request_id.as_deref()
+    }
+    pub fn extended_request_id(&self) -> ::std::option::Option<&str> {
+        self.extended_request_id.as_deref()
+    }
+}
+#[derive(Clone, Debug)]
 pub struct UnknownVariantError {
     value: ::std::string::String,
 }
@@ -21,10 +50,31 @@
     }
 }
 impl ::std::fmt::Display for UnknownVariantError {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         write!(f, "unknown enum variant: '{}'", self.value)
     }
 }
 impl ::std::error::Error for UnknownVariantError {}
+pub mod error {
+    pub use super::{BuildError, Error, ErrorMetadata, UnknownVariantError};
+}

-pub(crate) mod sealed_unhandled;
+#[derive(Clone, Debug)]
+pub struct BuildError {
+    field: ::std::string::String,
+    message: ::std::string::String,
+}
+impl BuildError {
+    pub fn missing_field(field: impl ::std::convert::Into<::std::string::String>, message: impl ::std::convert::Into<::std::string::String>) -> Self {
+        Self {
+            field: field.into(),
+            message: message.into(),
+        }
+    }
+}
+impl ::std::fmt::Display for BuildError {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        write!(f, "{}: {}", self.field, self.message)
+    }
+}
+impl ::std::error::Error for BuildError {}
```

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -1,238 +1,18 @@
-#![allow(deprecated)]
-#![allow(unknown_lints)]
-#![allow(clippy::module_inception)]
-#![allow(clippy::upper_case_acronyms)]
-#![allow(clippy::large_enum_variant)]
-#![allow(clippy::wrong_self_convention)]
-#![allow(clippy::should_implement_trait)]
-#![allow(clippy::disallowed_names)]
-#![allow(clippy::vec_init_then_push)]
-#![allow(clippy::type_complexity)]
-#![allow(clippy::needless_return)]
-#![allow(clippy::derive_partial_eq_without_eq)]
-#![allow(clippy::result_large_err)]
-#![allow(clippy::unnecessary_map_on_constructor)]
-#![allow(clippy::useless_conversion)]
-#![allow(clippy::deprecated_semver)]
-#![allow(rustdoc::bare_urls)]
-#![allow(rustdoc::redundant_explicit_links)]
-#![allow(rustdoc::broken_intra_doc_links)]
-#![allow(rustdoc::invalid_html_tags)]
-#![forbid(unsafe_code)]
-#![warn(missing_docs)]
-#![cfg_attr(docsrs, feature(doc_cfg))]
-//! Amazon DynamoDB is a fully managed NoSQL database service that provides fast and predictable performance with seamless scalability. DynamoDB lets you offload the administrative burdens of operating and scaling a distributed database, so that you don't have to worry about hardware provisioning, setup and configuration, replication, software patching, or cluster scaling.
-//!
-//! With DynamoDB, you can create database tables that can store and retrieve any amount of data, and serve any level of request traffic. You can scale up or scale down your tables' throughput capacity without downtime or performance degradation, and use the Amazon Web Services Management Console to monitor resource utilization and performance metrics.
-//!
-//! DynamoDB automatically spreads the data and traffic for your tables over a sufficient number of servers to handle your throughput and storage requirements, while maintaining consistent and fast performance. All of your data is stored on solid state disks (SSDs) and automatically replicated across multiple Availability Zones in an Amazon Web Services Region, providing built-in high availability and data durability.
-//!
-//! ## Getting Started
-//!
-//! > Examples are available for many services and operations, check out the
-//! > [usage examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1).
-//!
-//! The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
-//! as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-dynamodb` to
-//! your project, add the following to your **Cargo.toml** file:
-//!
-//! ```toml
-//! [dependencies]
-//! aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
-//! aws-sdk-dynamodb = "1.122.0"
-//! tokio = { version = "1", features = ["full"] }
-//! ```
-//!
-//! Then in code, a client can be created with the following:
-//!
-//! ```rust,no_run
-//! use aws_sdk_dynamodb as dynamodb;
-//!
-//! #[::tokio::main]
-//! async fn main() -> Result<(), dynamodb::Error> {
-//!     let config = aws_config::load_from_env().await;
-//!     let client = aws_sdk_dynamodb::Client::new(&config);
-//!
-//!     // ... make some calls with the client
-//!
-//!     Ok(())
-//! }
-//! ```
-//!
-//! See the [client documentation](https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/client/struct.Client.html)
-//! for information on what calls can be made, and the inputs and outputs for each of those calls.
-//!
-//! ## Using the SDK
-//!
-//! Until the SDK is released, we will be adding information about using the SDK to the
-//! [Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
-//! additional sections for the guide by opening an issue and describing what you are trying to do.
-//!
-//! ## Getting Help
-//!
-//! * [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
-//! * [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
-//! * [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
-//! * [Usage examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1)
-//!
-//!
-//! # Crate Organization
-//!
-//! The entry point for most customers will be [`Client`], which exposes one method for each API
-//! offered by Amazon DynamoDB. The return value of each of these methods is a "fluent builder",
-//! where the different inputs for that API are added by builder-style function call chaining,
-//! followed by calling `send()` to get a [`Future`](std::future::Future) that will result in
-//! either a successful output or a [`SdkError`](crate::error::SdkError).
-//!
-//! Some of these API inputs may be structs or enums to provide more complex structured information.
-//! These structs and enums live in [`types`](crate::types). There are some simpler types for
-//! representing data such as date times or binary blobs that live in [`primitives`](crate::primitives).
-//!
-//! All types required to configure a client via the [`Config`](crate::Config) struct live
-//! in [`config`](crate::config).
-//!
-//! The [`operation`](crate::operation) module has a submodule for every API, and in each submodule
-//! is the input, output, and error type for that API, as well as builders to construct each of those.
-//!
-//! There is a top-level [`Error`](crate::Error) type that encompasses all the errors that the
-//! client can return. Any other error type can be converted to this `Error` type via the
-//! [`From`](std::convert::From) trait.
-//!
-//! The other modules within this crate are not required for normal usage.
-
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use error_meta::Error;
-
-#[doc(inline)]
-pub use config::Config;

-/// Client for calling Amazon DynamoDB.
-/// ## Constructing a `Client`
-///
-/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
-/// crate should be used to automatically resolve this config using
-/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
-/// across multiple different AWS SDK clients. This config resolution process can be customized
-/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
-/// the [builder pattern] to customize the default config.
-///
-/// In the simplest case, creating a client looks as follows:
-/// ```rust,no_run
-/// # async fn wrapper() {
-/// let config = aws_config::load_from_env().await;
-/// let client = aws_sdk_dynamodb::Client::new(&config);
-/// # }
-/// ```
-///
-/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that
-/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
-/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be
-/// done as follows:
-///
-/// ```rust,no_run
-/// # async fn wrapper() {
-/// let sdk_config = ::aws_config::load_from_env().await;
-/// let config = aws_sdk_dynamodb::config::Builder::from(&sdk_config)
-/// # /*
-///     .some_service_specific_setting("value")
-/// # */
-///     .build();
-/// # }
-/// ```
-///
-/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
-///
-/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
-/// be done once at application start-up.
-///
-/// [`Config`]: crate::Config
-/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
-/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
-/// [`aws-config` docs]: https://docs.rs/aws-config/*
-/// [`aws-config`]: https://crates.io/crates/aws-config
-/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
-/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
-/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
-/// # Using the `Client`
-///
-/// A client has a function for every operation that can be performed by the service.
-/// For example, the [`BatchExecuteStatement`](crate::operation::batch_execute_statement) operation has
-/// a [`Client::batch_execute_statement`], function which returns a builder for that operation.
-/// The fluent builder ultimately has a `send()` function that returns an async future that
-/// returns a result, as illustrated below:
-///
-/// ```rust,ignore
-/// let result = client.batch_execute_statement()
-///     .return_consumed_capacity("example")
-///     .send()
-///     .await;
-/// ```
-///
-/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
-/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
-/// information.
-/// # Waiters
-///
-/// This client provides `wait_until` methods behind the [`Waiters`](crate::client::Waiters) trait.
-/// To use them, simply import the trait, and then call one of the `wait_until` methods. This will
-/// return a waiter fluent builder that takes various parameters, which are documented on the builder
-/// type. Once parameters have been provided, the `wait` method can be called to initiate waiting.
-///
-/// For example, if there was a `wait_until_thing` method, it could look like:
-/// ```rust,ignore
-/// let result = client.wait_until_thing()
-///     .thing_id("someId")
-///     .wait(Duration::from_secs(120))
-///     .await;
-/// ```
-pub mod client;
-
-/// Configuration for Amazon DynamoDB.
-pub mod config;
-
-/// Common errors and error handling utilities.
-pub mod error;
-
-mod error_meta;
-
-/// Information about this crate.
-pub mod meta;
-
-/// All operations that this crate can perform.
-pub mod operation;
-
-/// Primitives such as `Blob` or `DateTime` used by other types.
-pub mod primitives;
-
-/// Data structures used by operation inputs/outputs.
-pub mod types;
-
-mod account_id_endpoint;
-
-pub(crate) mod client_idempotency_token;
-
-mod idempotency_token;
-
-mod observability_feature;
-
-pub(crate) mod protocol_serde;
-
-mod sdk_feature_tracker;
-
-mod serialization_settings;
-
-mod endpoint_lib;
-
-mod lens;
-
-mod serde_util;
-
-/// Supporting types for waiters.
-///
-/// Note: to use waiters, import the [`Waiters`](crate::client::Waiters) trait, which adds methods prefixed with `wait_until` to the client.
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/primitives.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/error.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/meta.rs"));
+pub mod types {
+    include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/types.rs"));
+}
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/client.rs"));
+mod serde_util {
+    include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/serde_util.rs"));
+}
+mod lens {
+    include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/lens.rs"));
+}
 pub mod waiters;
-
-mod json_errors;
-
-#[doc(inline)]
-pub use client::Client;
```

### `src/operation/batch_execute_statement.rs`

```diff
--- reference/src/operation/batch_execute_statement.rs
+++ generated/src/operation/batch_execute_statement.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,22 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("BatchExecuteStatement")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                BatchExecuteStatementEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::batch_execute_statement::BatchExecuteStatementError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::batch_execute_statement::BatchExecuteStatementError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::batch_execute_statement::BatchExecuteStatementError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("BatchExecuteStatement")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    BatchExecuteStatementTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    BatchExecuteStatementEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::batch_execute_statement::BatchExecuteStatementError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::batch_execute_statement::BatchExecuteStatementError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::batch_execute_statement::BatchExecuteStatementError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -147,6 +159,44 @@
 }

 #[derive(Debug)]
+struct BatchExecuteStatementTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for BatchExecuteStatementTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "BatchExecuteStatementTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<BatchExecuteStatementInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct BatchExecuteStatementResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for BatchExecuteStatementResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -204,17 +254,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.BatchExecuteStatement",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_batch_execute_statement::ser_batch_execute_statement_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_batch_execute_statement_input::ser_batch_execute_statement_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -248,15 +293,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -387,6 +426,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::batch_execute_statement::BatchExecuteStatementError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::batch_execute_statement::BatchExecuteStatementError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/batch_get_item.rs`

```diff
--- reference/src/operation/batch_get_item.rs
+++ generated/src/operation/batch_get_item.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("BatchGetItem", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -124,6 +124,9 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("BatchGetItem")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                BatchGetItemTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -135,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::batch_get_item::BatchGetItemError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::batch_get_item::BatchGetItemError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::batch_get_item::BatchGetItemError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -144,6 +153,44 @@
 }

 #[derive(Debug)]
+struct BatchGetItemTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for BatchGetItemTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "BatchGetItemTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<BatchGetItemInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct BatchGetItemResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for BatchGetItemResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -201,15 +248,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.BatchGetItem",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_batch_get_item::ser_batch_get_item_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_batch_get_item_input::ser_batch_get_item_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -243,16 +285,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
-            .set_resource_arn_list(get_resource_arn_list(_input))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -266,13 +301,6 @@
 // The get_* functions below are generated from JMESPath expressions in the
 // operationContextParams trait. They target the operation's input shape.

-// Generated from JMESPath Expression: keys(RequestItems)
-fn get_resource_arn_list(input: &crate::operation::batch_get_item::BatchGetItemInput) -> Option<::std::vec::Vec<::std::string::String>> {
-    let _fld_2 = input.request_items.as_ref()?;
-    let _ret_1 = _fld_2.keys().map(Clone::clone).collect::<Vec<String>>();
-    Some(_ret_1)
-}
-
 /// Error type for the `BatchGetItemError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -279,7 +307,6 @@
 pub enum BatchGetItemError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The request was denied due to request throttling. For detailed information about why the request was throttled and the ARN of the impacted resource, find the <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ThrottlingReason.html">ThrottlingReason</a> field in the returned exception. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     ProvisionedThroughputExceededException(crate::types::error::ProvisionedThroughputExceededException),
@@ -420,6 +447,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::batch_get_item::BatchGetItemError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::batch_get_item::BatchGetItemError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/batch_write_item.rs`

```diff
--- reference/src/operation/batch_write_item.rs
+++ generated/src/operation/batch_write_item.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -127,6 +127,9 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("BatchWriteItem")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                BatchWriteItemTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -138,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::batch_write_item::BatchWriteItemError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::batch_write_item::BatchWriteItemError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::batch_write_item::BatchWriteItemError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -147,6 +156,44 @@
 }

 #[derive(Debug)]
+struct BatchWriteItemTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for BatchWriteItemTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "BatchWriteItemTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<BatchWriteItemInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct BatchWriteItemResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for BatchWriteItemResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -204,15 +251,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.BatchWriteItem",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_batch_write_item::ser_batch_write_item_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_batch_write_item_input::ser_batch_write_item_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -246,16 +290,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
-            .set_resource_arn_list(get_resource_arn_list(_input))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -269,13 +306,6 @@
 // The get_* functions below are generated from JMESPath expressions in the
 // operationContextParams trait. They target the operation's input shape.

-// Generated from JMESPath Expression: keys(RequestItems)
-fn get_resource_arn_list(input: &crate::operation::batch_write_item::BatchWriteItemInput) -> Option<::std::vec::Vec<::std::string::String>> {
-    let _fld_2 = input.request_items.as_ref()?;
-    let _ret_1 = _fld_2.keys().map(Clone::clone).collect::<Vec<String>>();
-    Some(_ret_1)
-}
-
 /// Error type for the `BatchWriteItemError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -282,7 +312,6 @@
 pub enum BatchWriteItemError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
     ItemCollectionSizeLimitExceededException(crate::types::error::ItemCollectionSizeLimitExceededException),
@@ -414,10 +443,7 @@
         ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
     }
     fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
-        match self {
-            Self::ReplicatedWriteConflictException(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
-            _ => ::std::option::Option::None,
-        }
+        ::std::option::Option::None
     }
 }
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for BatchWriteItemError {
@@ -446,6 +472,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::batch_write_item::BatchWriteItemError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::batch_write_item::BatchWriteItemError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_backup.rs`

```diff
--- reference/src/operation/create_backup.rs
+++ generated/src/operation/create_backup.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateBackup", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_backup::CreateBackupError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_backup::CreateBackupError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_backup::CreateBackupError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.CreateBackup",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_backup::ser_create_backup_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_backup_input::ser_create_backup_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,15 +295,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -333,7 +328,6 @@
     ContinuousBackupsUnavailableException(crate::types::error::ContinuousBackupsUnavailableException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -486,6 +480,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_backup::CreateBackupError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_backup::CreateBackupError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_global_table.rs`

```diff
--- reference/src/operation/create_global_table.rs
+++ generated/src/operation/create_global_table.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateGlobalTable")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateGlobalTableTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateGlobalTableEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::create_global_table::CreateGlobalTableError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::create_global_table::CreateGlobalTableError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_global_table::CreateGlobalTableError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateGlobalTable")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(CreateGlobalTableTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(CreateGlobalTableEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::create_global_table::CreateGlobalTableError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::create_global_table::CreateGlobalTableError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_global_table::CreateGlobalTableError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.CreateGlobalTable",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_global_table::ser_create_global_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_global_table_input::ser_create_global_table_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,15 +281,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .global_table_name
@@ -331,7 +314,6 @@
     GlobalTableAlreadyExistsException(crate::types::error::GlobalTableAlreadyExistsException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -466,6 +448,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_global_table::CreateGlobalTableError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_global_table::CreateGlobalTableError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_table.rs`

```diff
--- reference/src/operation/create_table.rs
+++ generated/src/operation/create_table.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateTable", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_table::CreateTableError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_table::CreateTableError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_table::CreateTableError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,15 +263,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.CreateTable",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_table::ser_create_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_table_input::ser_create_table_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,15 +300,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -334,7 +329,6 @@
 pub enum CreateTableError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -470,6 +464,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_table::CreateTableError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_table::CreateTableError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_backup.rs`

```diff
--- reference/src/operation/delete_backup.rs
+++ generated/src/operation/delete_backup.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteBackup", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_backup::DeleteBackupError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_backup::DeleteBackupError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_backup::DeleteBackupError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DeleteBackup",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_backup::ser_delete_backup_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_backup_input::ser_delete_backup_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,15 +290,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .backup_arn
@@ -328,7 +323,6 @@
     BackupNotFoundException(crate::types::error::BackupNotFoundException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -461,6 +455,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_backup::DeleteBackupError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_backup::DeleteBackupError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_item.rs`

```diff
--- reference/src/operation/delete_item.rs
+++ generated/src/operation/delete_item.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteItem", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_item::DeleteItemError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_item::DeleteItemError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_item::DeleteItemError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +256,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DeleteItem",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_item::ser_delete_item_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_item_input::ser_delete_item_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,15 +293,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -329,7 +324,6 @@
     ConditionalCheckFailedException(crate::types::error::ConditionalCheckFailedException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
     ItemCollectionSizeLimitExceededException(crate::types::error::ItemCollectionSizeLimitExceededException),
@@ -477,10 +471,7 @@
         ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
     }
     fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
-        match self {
-            Self::ReplicatedWriteConflictException(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
-            _ => ::std::option::Option::None,
-        }
+        ::std::option::Option::None
     }
 }
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DeleteItemError {
@@ -511,6 +502,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_item::DeleteItemError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_item::DeleteItemError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_resource_policy.rs`

```diff
--- reference/src/operation/delete_resource_policy.rs
+++ generated/src/operation/delete_resource_policy.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteResourcePolicy")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteResourcePolicyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteResourcePolicyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::delete_resource_policy::DeleteResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::delete_resource_policy::DeleteResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_resource_policy::DeleteResourcePolicyError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteResourcePolicy")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteResourcePolicyTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteResourcePolicyEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::delete_resource_policy::DeleteResourcePolicyError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::delete_resource_policy::DeleteResourcePolicyError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_resource_policy::DeleteResourcePolicyError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,17 +247,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DeleteResourcePolicy",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_resource_policy::ser_delete_resource_policy_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_delete_resource_policy_input::ser_delete_resource_policy_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,15 +286,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .resource_arn
@@ -334,7 +315,6 @@
 pub enum DeleteResourcePolicyError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -491,6 +471,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_resource_policy::DeleteResourcePolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_resource_policy::DeleteResourcePolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_table.rs`

```diff
--- reference/src/operation/delete_table.rs
+++ generated/src/operation/delete_table.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteTable", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_table::DeleteTableError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_table::DeleteTableError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_table::DeleteTableError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DeleteTable",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_table::ser_delete_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_delete_table_input::ser_delete_table_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,15 +290,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -324,7 +319,6 @@
 pub enum DeleteTableError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -470,6 +464,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_table::DeleteTableError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_table::DeleteTableError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_backup.rs`

```diff
--- reference/src/operation/describe_backup.rs
+++ generated/src/operation/describe_backup.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_backup::DescribeBackupError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_backup::DescribeBackupError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_backup::DescribeBackupError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +256,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeBackup",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_backup::ser_describe_backup_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_backup_input::ser_describe_backup_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,15 +293,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .backup_arn
@@ -329,7 +324,6 @@
     BackupNotFoundException(crate::types::error::BackupNotFoundException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
     #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
@@ -438,6 +432,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_backup::DescribeBackupError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_backup::DescribeBackupError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_continuous_backups.rs`

```diff
--- reference/src/operation/describe_continuous_backups.rs
+++ generated/src/operation/describe_continuous_backups.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::describe_continuous_backups::DescribeContinuousBackupsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -256,16 +264,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeContinuousBackups",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_describe_continuous_backups::ser_describe_continuous_backups_input(&input)?,
+            crate::protocol_serde::shape_describe_continuous_backups_input::ser_describe_continuous_backups_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -300,15 +303,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -335,7 +332,6 @@
 pub enum DescribeContinuousBackupsError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>A source table with the name <code>TableName</code> does not currently exist within the subscriber's account or the subscriber is operating in the wrong Amazon Web Services Region.</p>
     TableNotFoundException(crate::types::error::TableNotFoundException),
@@ -446,6 +442,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_continuous_backups::DescribeContinuousBackupsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_continuous_backups::DescribeContinuousBackupsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_contributor_insights.rs`

```diff
--- reference/src/operation/describe_contributor_insights.rs
+++ generated/src/operation/describe_contributor_insights.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_contributor_insights::DescribeContributorInsightsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_contributor_insights::DescribeContributorInsightsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::describe_contributor_insights::DescribeContributorInsightsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -261,16 +269,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeContributorInsights",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_describe_contributor_insights::ser_describe_contributor_insights_input(&input)?,
+            crate::protocol_serde::shape_describe_contributor_insights_input::ser_describe_contributor_insights_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -305,15 +308,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -441,6 +438,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_contributor_insights::DescribeContributorInsightsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_contributor_insights::DescribeContributorInsightsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_endpoints.rs`

```diff
--- reference/src/operation/describe_endpoints.rs
+++ generated/src/operation/describe_endpoints.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -127,6 +127,9 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DescribeEndpoints")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                DescribeEndpointsTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -138,9 +141,16 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_endpoints::DescribeEndpointsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_endpoints::DescribeEndpointsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_endpoints::DescribeEndpointsError>::builder(
+                )
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -147,6 +157,44 @@
 }

 #[derive(Debug)]
+struct DescribeEndpointsTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DescribeEndpointsTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "DescribeEndpointsTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<DescribeEndpointsInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct DescribeEndpointsResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for DescribeEndpointsResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -204,15 +252,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeEndpoints",
-            );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_endpoints::ser_describe_endpoints_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -243,15 +285,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -352,6 +388,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_endpoints::DescribeEndpointsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_endpoints::DescribeEndpointsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_export.rs`

```diff
--- reference/src/operation/describe_export.rs
+++ generated/src/operation/describe_export.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_export::DescribeExportError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_export::DescribeExportError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_export::DescribeExportError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +256,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeExport",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_export::ser_describe_export_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_export_input::ser_describe_export_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,15 +293,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .export_arn
@@ -444,6 +439,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_export::DescribeExportError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_export::DescribeExportError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_global_table.rs`

```diff
--- reference/src/operation/describe_global_table.rs
+++ generated/src/operation/describe_global_table.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DescribeGlobalTable")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DescribeGlobalTableTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DescribeGlobalTableEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::describe_global_table::DescribeGlobalTableError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::describe_global_table::DescribeGlobalTableError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_global_table::DescribeGlobalTableError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DescribeGlobalTable")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DescribeGlobalTableTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DescribeGlobalTableEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::describe_global_table::DescribeGlobalTableError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::describe_global_table::DescribeGlobalTableError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_global_table::DescribeGlobalTableError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,17 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeGlobalTable",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_global_table::ser_describe_global_table_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_describe_global_table_input::ser_describe_global_table_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,15 +281,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .global_table_name
@@ -333,7 +314,6 @@
     GlobalTableNotFoundException(crate::types::error::GlobalTableNotFoundException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
     #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
@@ -442,6 +422,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_global_table::DescribeGlobalTableError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_global_table::DescribeGlobalTableError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_global_table_settings.rs`

```diff
--- reference/src/operation/describe_global_table_settings.rs
+++ generated/src/operation/describe_global_table_settings.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -256,16 +264,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeGlobalTableSettings",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_describe_global_table_settings::ser_describe_global_table_settings_input(&input)?,
+            crate::protocol_serde::shape_describe_global_table_settings_input::ser_describe_global_table_settings_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -300,15 +303,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .global_table_name
@@ -339,7 +336,6 @@
     GlobalTableNotFoundException(crate::types::error::GlobalTableNotFoundException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
     #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
@@ -448,6 +444,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_import.rs`

```diff
--- reference/src/operation/describe_import.rs
+++ generated/src/operation/describe_import.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_import::DescribeImportError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_import::DescribeImportError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_import::DescribeImportError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +256,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeImport",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_import::ser_describe_import_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_import_input::ser_describe_import_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,15 +293,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .import_arn
@@ -418,6 +413,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_import::DescribeImportError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_import::DescribeImportError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_kinesis_streaming_destination.rs`

```diff
--- reference/src/operation/describe_kinesis_streaming_destination.rs
+++ generated/src/operation/describe_kinesis_streaming_destination.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -260,16 +268,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeKinesisStreamingDestination",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_describe_kinesis_streaming_destination::ser_describe_kinesis_streaming_destination_input(&input)?,
+            crate::protocol_serde::shape_describe_kinesis_streaming_destination_input::ser_describe_kinesis_streaming_destination_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -304,15 +307,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -339,7 +336,6 @@
 pub enum DescribeKinesisStreamingDestinationError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
     ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
@@ -450,6 +446,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_limits.rs`

```diff
--- reference/src/operation/describe_limits.rs
+++ generated/src/operation/describe_limits.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -127,6 +127,9 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DescribeLimits")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                DescribeLimitsTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -138,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_limits::DescribeLimitsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_limits::DescribeLimitsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_limits::DescribeLimitsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -147,6 +156,44 @@
 }

 #[derive(Debug)]
+struct DescribeLimitsTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DescribeLimitsTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "DescribeLimitsTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<DescribeLimitsInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct DescribeLimitsResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for DescribeLimitsResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -204,15 +251,9 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeLimits",
-            );
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_limits::ser_describe_limits_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -243,15 +284,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -271,7 +306,6 @@
 pub enum DescribeLimitsError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
     #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
@@ -372,6 +406,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_limits::DescribeLimitsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_limits::DescribeLimitsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_table.rs`

```diff
--- reference/src/operation/describe_table.rs
+++ generated/src/operation/describe_table.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DescribeTable", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_table::DescribeTableError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_table::DescribeTableError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_table::DescribeTableError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeTable",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_table::ser_describe_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_table_input::ser_describe_table_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,15 +290,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -324,7 +319,6 @@
 pub enum DescribeTableError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
     ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
@@ -435,6 +429,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_table::DescribeTableError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_table::DescribeTableError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_table_replica_auto_scaling.rs`

```diff
--- reference/src/operation/describe_table_replica_auto_scaling.rs
+++ generated/src/operation/describe_table_replica_auto_scaling.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -258,16 +266,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeTableReplicaAutoScaling",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_describe_table_replica_auto_scaling::ser_describe_table_replica_auto_scaling_input(&input)?,
+            crate::protocol_serde::shape_describe_table_replica_auto_scaling_input::ser_describe_table_replica_auto_scaling_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -302,15 +305,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -438,6 +435,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/describe_time_to_live.rs`

```diff
--- reference/src/operation/describe_time_to_live.rs
+++ generated/src/operation/describe_time_to_live.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DescribeTimeToLive")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DescribeTimeToLiveTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DescribeTimeToLiveEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::describe_time_to_live::DescribeTimeToLiveError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::describe_time_to_live::DescribeTimeToLiveError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::describe_time_to_live::DescribeTimeToLiveError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DescribeTimeToLive")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DescribeTimeToLiveTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DescribeTimeToLiveEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::describe_time_to_live::DescribeTimeToLiveError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::describe_time_to_live::DescribeTimeToLiveError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::describe_time_to_live::DescribeTimeToLiveError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,17 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DescribeTimeToLive",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_describe_time_to_live::ser_describe_time_to_live_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_describe_time_to_live_input::ser_describe_time_to_live_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,15 +281,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -329,7 +310,6 @@
 pub enum DescribeTimeToLiveError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
     ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
@@ -440,6 +420,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::describe_time_to_live::DescribeTimeToLiveError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::describe_time_to_live::DescribeTimeToLiveError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/disable_kinesis_streaming_destination.rs`

```diff
--- reference/src/operation/disable_kinesis_streaming_destination.rs
+++ generated/src/operation/disable_kinesis_streaming_destination.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -265,16 +273,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.DisableKinesisStreamingDestination",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_disable_kinesis_streaming_destination::ser_disable_kinesis_streaming_destination_input(&input)?,
+            crate::protocol_serde::shape_disable_kinesis_streaming_destination_input::ser_disable_kinesis_streaming_destination_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -309,15 +312,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -344,7 +341,6 @@
 pub enum DisableKinesisStreamingDestinationError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -490,6 +486,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/enable_kinesis_streaming_destination.rs`

```diff
--- reference/src/operation/enable_kinesis_streaming_destination.rs
+++ generated/src/operation/enable_kinesis_streaming_destination.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -265,16 +273,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.EnableKinesisStreamingDestination",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_enable_kinesis_streaming_destination::ser_enable_kinesis_streaming_destination_input(&input)?,
+            crate::protocol_serde::shape_enable_kinesis_streaming_destination_input::ser_enable_kinesis_streaming_destination_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -309,15 +312,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -344,7 +341,6 @@
 pub enum EnableKinesisStreamingDestinationError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -490,6 +486,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/execute_statement.rs`

```diff
--- reference/src/operation/execute_statement.rs
+++ generated/src/operation/execute_statement.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::execute_statement::ExecuteStatementError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::execute_statement::ExecuteStatementError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::execute_statement::ExecuteStatementError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,15 +261,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ExecuteStatement",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_execute_statement::ser_execute_statement_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_execute_statement_input::ser_execute_statement_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -297,15 +300,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -496,6 +493,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::execute_statement::ExecuteStatementError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::execute_statement::ExecuteStatementError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/execute_transaction.rs`

```diff
--- reference/src/operation/execute_transaction.rs
+++ generated/src/operation/execute_transaction.rs
@@ -114,9 +114,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -132,25 +132,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ExecuteTransaction")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ExecuteTransactionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ExecuteTransactionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::execute_transaction::ExecuteTransactionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::execute_transaction::ExecuteTransactionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::execute_transaction::ExecuteTransactionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ExecuteTransaction")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ExecuteTransactionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ExecuteTransactionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::execute_transaction::ExecuteTransactionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::execute_transaction::ExecuteTransactionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::execute_transaction::ExecuteTransactionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,15 +249,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ExecuteTransaction",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_execute_transaction::ser_execute_transaction_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_execute_transaction_input::ser_execute_transaction_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,15 +288,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -650,6 +633,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::execute_transaction::ExecuteTransactionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::execute_transaction::ExecuteTransactionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/export_table_to_point_in_time.rs`

```diff
--- reference/src/operation/export_table_to_point_in_time.rs
+++ generated/src/operation/export_table_to_point_in_time.rs
@@ -115,9 +115,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -149,9 +149,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -283,16 +291,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ExportTableToPointInTime",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_export_table_to_point_in_time::ser_export_table_to_point_in_time_input(&input)?,
+            crate::protocol_serde::shape_export_table_to_point_in_time_input::ser_export_table_to_point_in_time_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -327,15 +330,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_arn
@@ -509,6 +506,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_item.rs`

```diff
--- reference/src/operation/get_item.rs
+++ generated/src/operation/get_item.rs
@@ -100,9 +100,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetItem", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -134,9 +134,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_item::GetItemError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_item::GetItemError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_item::GetItemError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -246,15 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.GetItem",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_item::ser_get_item_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_item_input::ser_get_item_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -288,15 +289,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -323,7 +318,6 @@
 pub enum GetItemError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The request was denied due to request throttling. For detailed information about why the request was throttled and the ARN of the impacted resource, find the <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ThrottlingReason.html">ThrottlingReason</a> field in the returned exception. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     ProvisionedThroughputExceededException(crate::types::error::ProvisionedThroughputExceededException),
@@ -464,6 +458,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_item::GetItemError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_item::GetItemError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_resource_policy.rs`

```diff
--- reference/src/operation/get_resource_policy.rs
+++ generated/src/operation/get_resource_policy.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetResourcePolicy")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetResourcePolicyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetResourcePolicyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_resource_policy::GetResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_resource_policy::GetResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_resource_policy::GetResourcePolicyError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetResourcePolicy")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetResourcePolicyTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetResourcePolicyEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_resource_policy::GetResourcePolicyError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_resource_policy::GetResourcePolicyError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_resource_policy::GetResourcePolicyError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.GetResourcePolicy",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_resource_policy::ser_get_resource_policy_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_get_resource_policy_input::ser_get_resource_policy_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,15 +281,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .resource_arn
@@ -327,7 +310,6 @@
 pub enum GetResourcePolicyError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The operation tried to access a nonexistent resource-based policy.</p>
     /// <p>If you specified an <code>ExpectedRevisionId</code>, it's possible that a policy is present for the resource but its revision ID didn't match the expected value.</p>
@@ -449,6 +431,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_resource_policy::GetResourcePolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_resource_policy::GetResourcePolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/import_table.rs`

```diff
--- reference/src/operation/import_table.rs
+++ generated/src/operation/import_table.rs
@@ -111,9 +111,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ImportTable", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -145,9 +145,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::import_table::ImportTableError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::import_table::ImportTableError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::import_table::ImportTableError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -254,15 +260,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ImportTable",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_import_table::ser_import_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_import_table_input::ser_import_table_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -296,16 +297,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
-            .set_resource_arn(get_resource_arn(_input).cloned())
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -319,13 +313,6 @@
 // The get_* functions below are generated from JMESPath expressions in the
 // operationContextParams trait. They target the operation's input shape.

-// Generated from JMESPath Expression: TableCreationParameters.TableName
-fn get_resource_arn(input: &crate::operation::import_table::ImportTableInput) -> Option<&::std::string::String> {
-    let _fld_1 = input.table_creation_parameters.as_ref()?;
-    let _fld_2 = &_fld_1.table_name;
-    Some(_fld_2)
-}
-
 /// Error type for the `ImportTableError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -458,6 +445,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::import_table::ImportTableError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::import_table::ImportTableError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_backups.rs`

```diff
--- reference/src/operation/list_backups.rs
+++ generated/src/operation/list_backups.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListBackups", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_backups::ListBackupsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_backups::ListBackupsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_backups::ListBackupsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ListBackups",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_backups::ser_list_backups_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_backups_input::ser_list_backups_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,15 +295,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(_input.table_name.clone())
             .build()
             .map_err(|err| {
@@ -323,7 +318,6 @@
 pub enum ListBackupsError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
     #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
@@ -424,6 +418,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_backups::ListBackupsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_backups::ListBackupsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_contributor_insights.rs`

```diff
--- reference/src/operation/list_contributor_insights.rs
+++ generated/src/operation/list_contributor_insights.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_contributor_insights::ListContributorInsightsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_contributor_insights::ListContributorInsightsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_contributor_insights::ListContributorInsightsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,16 +263,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ListContributorInsights",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_list_contributor_insights::ser_list_contributor_insights_input(&input)?,
+            crate::protocol_serde::shape_list_contributor_insights_input::ser_list_contributor_insights_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -299,15 +302,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(_input.table_name.clone())
             .build()
             .map_err(|err| {
@@ -429,6 +426,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_contributor_insights::ListContributorInsightsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_contributor_insights::ListContributorInsightsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_exports.rs`

```diff
--- reference/src/operation/list_exports.rs
+++ generated/src/operation/list_exports.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListExports", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_exports::ListExportsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_exports::ListExportsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_exports::ListExportsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ListExports",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_exports::ser_list_exports_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_exports_input::ser_list_exports_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,15 +295,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(_input.table_arn.clone())
             .build()
             .map_err(|err| {
@@ -430,6 +425,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_exports::ListExportsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_exports::ListExportsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_global_tables.rs`

```diff
--- reference/src/operation/list_global_tables.rs
+++ generated/src/operation/list_global_tables.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_global_tables::ListGlobalTablesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_global_tables::ListGlobalTablesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_global_tables::ListGlobalTablesError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,15 +261,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ListGlobalTables",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_global_tables::ser_list_global_tables_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_global_tables_input::ser_list_global_tables_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -297,15 +300,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -325,7 +322,6 @@
 pub enum ListGlobalTablesError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
     #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
@@ -426,6 +422,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_global_tables::ListGlobalTablesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_global_tables::ListGlobalTablesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_imports.rs`

```diff
--- reference/src/operation/list_imports.rs
+++ generated/src/operation/list_imports.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListImports", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_imports::ListImportsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_imports::ListImportsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_imports::ListImportsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,15 +258,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ListImports",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_imports::ser_list_imports_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_imports_input::ser_list_imports_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -294,15 +295,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(_input.table_arn.clone())
             .build()
             .map_err(|err| {
@@ -420,6 +415,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_imports::ListImportsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_imports::ListImportsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_tables.rs`

```diff
--- reference/src/operation/list_tables.rs
+++ generated/src/operation/list_tables.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListTables", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_tables::ListTablesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_tables::ListTablesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_tables::ListTablesError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -245,15 +251,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ListTables",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_tables::ser_list_tables_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_tables_input::ser_list_tables_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -287,15 +288,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -315,7 +310,6 @@
 pub enum ListTablesError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
     #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
@@ -416,6 +410,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_tables::ListTablesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_tables::ListTablesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_tags_of_resource.rs`

```diff
--- reference/src/operation/list_tags_of_resource.rs
+++ generated/src/operation/list_tags_of_resource.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListTagsOfResource")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListTagsOfResourceTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListTagsOfResourceEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_tags_of_resource::ListTagsOfResourceError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_tags_of_resource::ListTagsOfResourceError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_tags_of_resource::ListTagsOfResourceError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListTagsOfResource")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListTagsOfResourceTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListTagsOfResourceEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::list_tags_of_resource::ListTagsOfResourceError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::list_tags_of_resource::ListTagsOfResourceError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_tags_of_resource::ListTagsOfResourceError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,17 +247,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.ListTagsOfResource",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_list_tags_of_resource::ser_list_tags_of_resource_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_list_tags_of_resource_input::ser_list_tags_of_resource_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -299,15 +286,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .resource_arn
@@ -334,7 +315,6 @@
 pub enum ListTagsOfResourceError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
     ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
@@ -445,6 +425,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_tags_of_resource::ListTagsOfResourceError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_tags_of_resource::ListTagsOfResourceError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_item.rs`

```diff
--- reference/src/operation/put_item.rs
+++ generated/src/operation/put_item.rs
@@ -100,9 +100,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("PutItem", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -134,9 +134,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::put_item::PutItemError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_item::PutItemError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::put_item::PutItemError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -246,15 +252,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.PutItem",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_put_item::ser_put_item_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_put_item_input::ser_put_item_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -288,15 +289,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -325,7 +320,6 @@
     ConditionalCheckFailedException(crate::types::error::ConditionalCheckFailedException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
     ItemCollectionSizeLimitExceededException(crate::types::error::ItemCollectionSizeLimitExceededException),
@@ -473,10 +467,7 @@
         ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
     }
     fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
-        match self {
-            Self::ReplicatedWriteConflictException(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
-            _ => ::std::option::Option::None,
-        }
+        ::std::option::Option::None
     }
 }
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for PutItemError {
@@ -507,6 +498,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_item::PutItemError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_item::PutItemError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_resource_policy.rs`

```diff
--- reference/src/operation/put_resource_policy.rs
+++ generated/src/operation/put_resource_policy.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PutResourcePolicy")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                PutResourcePolicyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                PutResourcePolicyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::put_resource_policy::PutResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::put_resource_policy::PutResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_resource_policy::PutResourcePolicyError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PutResourcePolicy")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(PutResourcePolicyTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(PutResourcePolicyEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::put_resource_policy::PutResourcePolicyError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::put_resource_policy::PutResourcePolicyError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::put_resource_policy::PutResourcePolicyError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,18 +249,16 @@
             ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
+                let builder = crate::protocol_serde::shape_put_resource_policy::ser_put_resource_policy_headers(input, builder)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.PutResourcePolicy",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_put_resource_policy::ser_put_resource_policy_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_put_resource_policy_input::ser_put_resource_policy_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -302,15 +292,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .resource_arn
@@ -337,7 +321,6 @@
 pub enum PutResourcePolicyError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -494,6 +477,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_resource_policy::PutResourcePolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_resource_policy::PutResourcePolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/query/_query_output.rs`

```diff
--- reference/src/operation/query/_query_output.rs
+++ generated/src/operation/query/_query_output.rs
@@ -9,10 +9,10 @@
     /// <p>The number of items in the response.</p>
     /// <p>If you used a <code>QueryFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p>
     /// <p>If you did not use a filter in the request, then <code>Count</code> and <code>ScannedCount</code> are the same.</p>
-    pub count: i32,
+    pub count: ::std::option::Option<i32>,
     /// <p>The number of items evaluated, before any <code>QueryFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Query</code> operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     /// <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
-    pub scanned_count: i32,
+    pub scanned_count: ::std::option::Option<i32>,
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
     /// <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
     /// <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
@@ -31,12 +31,12 @@
     /// <p>The number of items in the response.</p>
     /// <p>If you used a <code>QueryFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p>
     /// <p>If you did not use a filter in the request, then <code>Count</code> and <code>ScannedCount</code> are the same.</p>
-    pub fn count(&self) -> i32 {
+    pub fn count(&self) -> ::std::option::Option<i32> {
         self.count
     }
     /// <p>The number of items evaluated, before any <code>QueryFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Query</code> operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     /// <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
-    pub fn scanned_count(&self) -> i32 {
+    pub fn scanned_count(&self) -> ::std::option::Option<i32> {
         self.scanned_count
     }
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
@@ -192,8 +192,8 @@
     pub fn build(self) -> crate::operation::query::QueryOutput {
         crate::operation::query::QueryOutput {
             items: self.items,
-            count: self.count.unwrap_or_default(),
-            scanned_count: self.scanned_count.unwrap_or_default(),
+            count: self.count,
+            scanned_count: self.scanned_count,
             last_evaluated_key: self.last_evaluated_key,
             consumed_capacity: self.consumed_capacity,
             _request_id: self._request_id,
```

### `src/operation/query.rs`

```diff
--- reference/src/operation/query.rs
+++ generated/src/operation/query.rs
@@ -95,9 +95,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Query", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -129,9 +129,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::query::QueryError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::query::QueryError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::query::QueryError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -256,15 +262,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.Query",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_query::ser_query_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_query_input::ser_query_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -295,15 +296,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -330,7 +325,6 @@
 pub enum QueryError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The request was denied due to request throttling. For detailed information about why the request was throttled and the ARN of the impacted resource, find the <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ThrottlingReason.html">ThrottlingReason</a> field in the returned exception. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     ProvisionedThroughputExceededException(crate::types::error::ProvisionedThroughputExceededException),
@@ -471,6 +465,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::query::QueryError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::query::QueryError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/restore_table_from_backup.rs`

```diff
--- reference/src/operation/restore_table_from_backup.rs
+++ generated/src/operation/restore_table_from_backup.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RestoreTableFromBackup")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                RestoreTableFromBackupTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                RestoreTableFromBackupEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RestoreTableFromBackup")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    RestoreTableFromBackupTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    RestoreTableFromBackupEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,16 +264,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.RestoreTableFromBackup",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_restore_table_from_backup::ser_restore_table_from_backup_input(&input)?,
+            crate::protocol_serde::shape_restore_table_from_backup_input::ser_restore_table_from_backup_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -299,15 +303,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .target_table_name
@@ -340,7 +338,6 @@
     BackupNotFoundException(crate::types::error::BackupNotFoundException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -493,6 +490,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::restore_table_from_backup::RestoreTableFromBackupError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::restore_table_from_backup::RestoreTableFromBackupError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/restore_table_to_point_in_time.rs`

```diff
--- reference/src/operation/restore_table_to_point_in_time.rs
+++ generated/src/operation/restore_table_to_point_in_time.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -266,16 +274,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.RestoreTableToPointInTime",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_restore_table_to_point_in_time::ser_restore_table_to_point_in_time_input(&input)?,
+            crate::protocol_serde::shape_restore_table_to_point_in_time_input::ser_restore_table_to_point_in_time_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -310,15 +313,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .target_table_name
@@ -347,7 +344,6 @@
 pub enum RestoreTableToPointInTimeError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>An invalid restore time was specified. RestoreDateTime must be between EarliestRestorableDateTime and LatestRestorableDateTime.</p>
     InvalidRestoreTimeException(crate::types::error::InvalidRestoreTimeException),
@@ -514,6 +510,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/scan/_scan_output.rs`

```diff
--- reference/src/operation/scan/_scan_output.rs
+++ generated/src/operation/scan/_scan_output.rs
@@ -9,10 +9,10 @@
     /// <p>The number of items in the response.</p>
     /// <p>If you set <code>ScanFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p>
     /// <p>If you did not use a filter in the request, then <code>Count</code> is the same as <code>ScannedCount</code>.</p>
-    pub count: i32,
+    pub count: ::std::option::Option<i32>,
     /// <p>The number of items evaluated, before any <code>ScanFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Scan</code> operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     /// <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
-    pub scanned_count: i32,
+    pub scanned_count: ::std::option::Option<i32>,
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
     /// <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>
     /// <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
@@ -31,12 +31,12 @@
     /// <p>The number of items in the response.</p>
     /// <p>If you set <code>ScanFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p>
     /// <p>If you did not use a filter in the request, then <code>Count</code> is the same as <code>ScannedCount</code>.</p>
-    pub fn count(&self) -> i32 {
+    pub fn count(&self) -> ::std::option::Option<i32> {
         self.count
     }
     /// <p>The number of items evaluated, before any <code>ScanFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Scan</code> operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     /// <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
-    pub fn scanned_count(&self) -> i32 {
+    pub fn scanned_count(&self) -> ::std::option::Option<i32> {
         self.scanned_count
     }
     /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>
@@ -192,8 +192,8 @@
     pub fn build(self) -> crate::operation::scan::ScanOutput {
         crate::operation::scan::ScanOutput {
             items: self.items,
-            count: self.count.unwrap_or_default(),
-            scanned_count: self.scanned_count.unwrap_or_default(),
+            count: self.count,
+            scanned_count: self.scanned_count,
             last_evaluated_key: self.last_evaluated_key,
             consumed_capacity: self.consumed_capacity,
             _request_id: self._request_id,
```

### `src/operation/scan.rs`

```diff
--- reference/src/operation/scan.rs
+++ generated/src/operation/scan.rs
@@ -95,9 +95,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Scan", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -129,9 +129,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::scan::ScanError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::scan::ScanError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::scan::ScanError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -251,15 +257,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.Scan",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_scan::ser_scan_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_scan_input::ser_scan_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -290,15 +291,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -325,7 +320,6 @@
 pub enum ScanError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The request was denied due to request throttling. For detailed information about why the request was throttled and the ARN of the impacted resource, find the <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ThrottlingReason.html">ThrottlingReason</a> field in the returned exception. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     ProvisionedThroughputExceededException(crate::types::error::ProvisionedThroughputExceededException),
@@ -466,6 +460,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::scan::ScanError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::scan::ScanError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/search_vectors.rs`

```diff
--- reference/src/operation/search_vectors.rs
+++ generated/src/operation/search_vectors.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("SearchVectors", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::search_vectors::SearchVectorsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::search_vectors::SearchVectorsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::search_vectors::SearchVectorsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -262,15 +268,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.SearchVectors",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_search_vectors::ser_search_vectors_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_search_vectors_input::ser_search_vectors_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -304,15 +305,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_is_search_operation(Some(true))
             .set_resource_arn(Some(
                 _input
@@ -461,6 +456,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::search_vectors::SearchVectorsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::search_vectors::SearchVectorsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/tag_resource.rs`

```diff
--- reference/src/operation/tag_resource.rs
+++ generated/src/operation/tag_resource.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("TagResource", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::tag_resource::TagResourceError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::tag_resource::TagResourceError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::tag_resource::TagResourceError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.TagResource",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_resource::ser_tag_resource_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_resource_input::ser_tag_resource_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,15 +290,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .resource_arn
@@ -324,7 +319,6 @@
 pub enum TagResourceError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -470,6 +464,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::tag_resource::TagResourceError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::tag_resource::TagResourceError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/transact_get_items.rs`

```diff
--- reference/src/operation/transact_get_items.rs
+++ generated/src/operation/transact_get_items.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -127,6 +127,9 @@
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("TransactGetItems")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                TransactGetItemsTelemetryInputCaptureInterceptor,
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -138,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::transact_get_items::TransactGetItemsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::transact_get_items::TransactGetItemsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::transact_get_items::TransactGetItemsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -147,6 +156,44 @@
 }

 #[derive(Debug)]
+struct TransactGetItemsTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TransactGetItemsTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "TransactGetItemsTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<TransactGetItemsInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct TransactGetItemsResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for TransactGetItemsResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -204,15 +251,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.TransactGetItems",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_transact_get_items::ser_transact_get_items_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_transact_get_items_input::ser_transact_get_items_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -246,16 +290,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
-            .set_resource_arn_list(get_resource_arn_list(_input).map(|v| v.into_iter().cloned().collect::<Vec<_>>()))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -269,24 +306,6 @@
 // The get_* functions below are generated from JMESPath expressions in the
 // operationContextParams trait. They target the operation's input shape.

-// Generated from JMESPath Expression: TransactItems[*].Get.TableName
-fn get_resource_arn_list(input: &crate::operation::transact_get_items::TransactGetItemsInput) -> Option<::std::vec::Vec<&::std::string::String>> {
-    let _fld_1 = input.transact_items.as_ref()?;
-    let _prj_4 = _fld_1
-        .iter()
-        .flat_map(|v| {
-            #[allow(clippy::let_and_return)]
-            fn map(_v: &crate::types::TransactGetItem) -> ::std::option::Option<&::std::string::String> {
-                let _fld_2 = _v.get.as_ref();
-                let _fld_3 = _fld_2.map(|v| &v.table_name);
-                _fld_3
-            }
-            map(v)
-        })
-        .collect::<::std::vec::Vec<_>>();
-    Some(_prj_4)
-}
-
 /// Error type for the `TransactGetItemsError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -293,7 +312,6 @@
 pub enum TransactGetItemsError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The request was denied due to request throttling. For detailed information about why the request was throttled and the ARN of the impacted resource, find the <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ThrottlingReason.html">ThrottlingReason</a> field in the returned exception. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     ProvisionedThroughputExceededException(crate::types::error::ProvisionedThroughputExceededException),
@@ -575,6 +593,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::transact_get_items::TransactGetItemsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::transact_get_items::TransactGetItemsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/transact_write_items/builders.rs`

```diff
--- reference/src/operation/transact_write_items/builders.rs
+++ generated/src/operation/transact_write_items/builders.rs
@@ -26,13 +26,13 @@
 /// <p>The actions are completed atomically so that either all of them succeed, or all of them fail. They are defined by the following objects:</p>
 /// <ul>
 /// <li>
-/// <p><code>Put</code> &nbsp;— &nbsp; Initiates a <code>PutItem</code> operation to write a new item. This structure specifies the primary key of the item to be written, the name of the table to write it in, an optional condition expression that must be satisfied for the write to succeed, a list of the item's attributes, and a field indicating whether to retrieve the item's attributes if the condition is not met.</p></li>
+/// <p><code>Put</code> — Initiates a <code>PutItem</code> operation to write a new item. This structure specifies the primary key of the item to be written, the name of the table to write it in, an optional condition expression that must be satisfied for the write to succeed, a list of the item's attributes, and a field indicating whether to retrieve the item's attributes if the condition is not met.</p></li>
 /// <li>
-/// <p><code>Update</code> &nbsp;— &nbsp; Initiates an <code>UpdateItem</code> operation to update an existing item. This structure specifies the primary key of the item to be updated, the name of the table where it resides, an optional condition expression that must be satisfied for the update to succeed, an expression that defines one or more attributes to be updated, and a field indicating whether to retrieve the item's attributes if the condition is not met.</p></li>
+/// <p><code>Update</code> — Initiates an <code>UpdateItem</code> operation to update an existing item. This structure specifies the primary key of the item to be updated, the name of the table where it resides, an optional condition expression that must be satisfied for the update to succeed, an expression that defines one or more attributes to be updated, and a field indicating whether to retrieve the item's attributes if the condition is not met.</p></li>
 /// <li>
-/// <p><code>Delete</code> &nbsp;— &nbsp; Initiates a <code>DeleteItem</code> operation to delete an existing item. This structure specifies the primary key of the item to be deleted, the name of the table where it resides, an optional condition expression that must be satisfied for the deletion to succeed, and a field indicating whether to retrieve the item's attributes if the condition is not met.</p></li>
+/// <p><code>Delete</code> — Initiates a <code>DeleteItem</code> operation to delete an existing item. This structure specifies the primary key of the item to be deleted, the name of the table where it resides, an optional condition expression that must be satisfied for the deletion to succeed, and a field indicating whether to retrieve the item's attributes if the condition is not met.</p></li>
 /// <li>
-/// <p><code>ConditionCheck</code> &nbsp;— &nbsp; Applies a condition to an item that is not being modified by the transaction. This structure specifies the primary key of the item to be checked, the name of the table where it resides, a condition expression that must be satisfied for the transaction to succeed, and a field indicating whether to retrieve the item's attributes if the condition is not met.</p></li>
+/// <p><code>ConditionCheck</code> — Applies a condition to an item that is not being modified by the transaction. This structure specifies the primary key of the item to be checked, the name of the table where it resides, a condition expression that must be satisfied for the transaction to succeed, and a field indicating whether to retrieve the item's attributes if the condition is not met.</p></li>
 /// </ul>
 /// <p>DynamoDB rejects the entire <code>TransactWriteItems</code> request if any of the following is true:</p>
 /// <ul>
```

### `src/operation/transact_write_items.rs`

```diff
--- reference/src/operation/transact_write_items.rs
+++ generated/src/operation/transact_write_items.rs
@@ -114,9 +114,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -132,25 +132,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("TransactWriteItems")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                TransactWriteItemsTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                TransactWriteItemsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::transact_write_items::TransactWriteItemsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::transact_write_items::TransactWriteItemsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::transact_write_items::TransactWriteItemsError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("TransactWriteItems")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(TransactWriteItemsTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(TransactWriteItemsEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::transact_write_items::TransactWriteItemsError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::transact_write_items::TransactWriteItemsError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::transact_write_items::TransactWriteItemsError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,16 +249,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.TransactWriteItems",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_transact_write_items::ser_transact_write_items_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_transact_write_items_input::ser_transact_write_items_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -300,16 +288,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
-            .set_resource_arn_list(get_resource_arn_list(_input).map(|v| v.into_iter().cloned().collect::<Vec<_>>()))
             .build()
             .map_err(|err| {
                 ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
@@ -323,33 +304,6 @@
 // The get_* functions below are generated from JMESPath expressions in the
 // operationContextParams trait. They target the operation's input shape.

-// Generated from JMESPath Expression: TransactItems[*].[ConditionCheck.TableName, Put.TableName, Delete.TableName, Update.TableName][]
-fn get_resource_arn_list(input: &crate::operation::transact_write_items::TransactWriteItemsInput) -> Option<::std::vec::Vec<&::std::string::String>> {
-    let _fld_1 = input.transact_items.as_ref()?;
-    let _prj_11 = _fld_1
-        .iter()
-        .flat_map(|v| {
-            #[allow(clippy::let_and_return)]
-            fn map(_v: &crate::types::TransactWriteItem) -> ::std::option::Option<::std::vec::Vec<::std::option::Option<&::std::string::String>>> {
-                let _fld_2 = _v.condition_check.as_ref();
-                let _fld_3 = _fld_2.map(|v| &v.table_name);
-                let _fld_4 = _v.put.as_ref();
-                let _fld_5 = _fld_4.map(|v| &v.table_name);
-                let _fld_6 = _v.delete.as_ref();
-                let _fld_7 = _fld_6.map(|v| &v.table_name);
-                let _fld_8 = _v.update.as_ref();
-                let _fld_9 = _fld_8.map(|v| &v.table_name);
-                let _msl_10 = vec![_fld_3, _fld_5, _fld_7, _fld_9];
-                ::std::option::Option::Some(_msl_10)
-            }
-            map(v)
-        })
-        .flatten()
-        .flatten()
-        .collect::<::std::vec::Vec<_>>();
-    Some(_prj_11)
-}
-
 /// Error type for the `TransactWriteItemsError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -358,7 +312,6 @@
     IdempotentParameterMismatchException(crate::types::error::IdempotentParameterMismatchException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The request was denied due to request throttling. For detailed information about why the request was throttled and the ARN of the impacted resource, find the <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ThrottlingReason.html">ThrottlingReason</a> field in the returned exception. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
     ProvisionedThroughputExceededException(crate::types::error::ProvisionedThroughputExceededException),
@@ -689,6 +642,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::transact_write_items::TransactWriteItemsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::transact_write_items::TransactWriteItemsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/untag_resource.rs`

```diff
--- reference/src/operation/untag_resource.rs
+++ generated/src/operation/untag_resource.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UntagResource", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::untag_resource::UntagResourceError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::untag_resource::UntagResourceError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::untag_resource::UntagResourceError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UntagResource",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_untag_resource::ser_untag_resource_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_untag_resource_input::ser_untag_resource_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,15 +290,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .resource_arn
@@ -324,7 +319,6 @@
 pub enum UntagResourceError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -470,6 +464,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::untag_resource::UntagResourceError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::untag_resource::UntagResourceError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_continuous_backups.rs`

```diff
--- reference/src/operation/update_continuous_backups.rs
+++ generated/src/operation/update_continuous_backups.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_continuous_backups::UpdateContinuousBackupsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_continuous_backups::UpdateContinuousBackupsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_continuous_backups::UpdateContinuousBackupsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,16 +258,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateContinuousBackups",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_continuous_backups::ser_update_continuous_backups_input(&input)?,
+            crate::protocol_serde::shape_update_continuous_backups_input::ser_update_continuous_backups_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -294,15 +297,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -331,7 +328,6 @@
     ContinuousBackupsUnavailableException(crate::types::error::ContinuousBackupsUnavailableException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>A source table with the name <code>TableName</code> does not currently exist within the subscriber's account or the subscriber is operating in the wrong Amazon Web Services Region.</p>
     TableNotFoundException(crate::types::error::TableNotFoundException),
@@ -450,6 +446,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_continuous_backups::UpdateContinuousBackupsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_continuous_backups::UpdateContinuousBackupsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_contributor_insights.rs`

```diff
--- reference/src/operation/update_contributor_insights.rs
+++ generated/src/operation/update_contributor_insights.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_contributor_insights::UpdateContributorInsightsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_contributor_insights::UpdateContributorInsightsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_contributor_insights::UpdateContributorInsightsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -261,16 +269,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateContributorInsights",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_contributor_insights::ser_update_contributor_insights_input(&input)?,
+            crate::protocol_serde::shape_update_contributor_insights_input::ser_update_contributor_insights_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -305,15 +308,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -441,6 +438,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_contributor_insights::UpdateContributorInsightsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_contributor_insights::UpdateContributorInsightsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_global_table.rs`

```diff
--- reference/src/operation/update_global_table.rs
+++ generated/src/operation/update_global_table.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateGlobalTable")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateGlobalTableTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateGlobalTableEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::update_global_table::UpdateGlobalTableError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::update_global_table::UpdateGlobalTableError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_global_table::UpdateGlobalTableError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateGlobalTable")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdateGlobalTableTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdateGlobalTableEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::update_global_table::UpdateGlobalTableError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::update_global_table::UpdateGlobalTableError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_global_table::UpdateGlobalTableError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +242,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateGlobalTable",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_global_table::ser_update_global_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_global_table_input::ser_update_global_table_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,15 +281,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .global_table_name
@@ -331,7 +314,6 @@
     GlobalTableNotFoundException(crate::types::error::GlobalTableNotFoundException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>The specified replica is already part of the global table.</p>
     ReplicaAlreadyExistsException(crate::types::error::ReplicaAlreadyExistsException),
@@ -470,6 +452,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_global_table::UpdateGlobalTableError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_global_table::UpdateGlobalTableError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_global_table_settings.rs`

```diff
--- reference/src/operation/update_global_table_settings.rs
+++ generated/src/operation/update_global_table_settings.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -256,16 +264,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateGlobalTableSettings",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_global_table_settings::ser_update_global_table_settings_input(&input)?,
+            crate::protocol_serde::shape_update_global_table_settings_input::ser_update_global_table_settings_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -300,15 +303,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .global_table_name
@@ -341,7 +338,6 @@
     IndexNotFoundException(crate::types::error::IndexNotFoundException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -503,6 +499,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_item.rs`

```diff
--- reference/src/operation/update_item.rs
+++ generated/src/operation/update_item.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UpdateItem", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_item::UpdateItemError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_item::UpdateItemError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_item::UpdateItemError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,15 +261,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateItem",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_item::ser_update_item_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_item_input::ser_update_item_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -297,15 +298,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -334,7 +329,6 @@
     ConditionalCheckFailedException(crate::types::error::ConditionalCheckFailedException),
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
     ItemCollectionSizeLimitExceededException(crate::types::error::ItemCollectionSizeLimitExceededException),
@@ -482,10 +476,7 @@
         ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
     }
     fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
-        match self {
-            Self::ReplicatedWriteConflictException(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
-            _ => ::std::option::Option::None,
-        }
+        ::std::option::Option::None
     }
 }
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for UpdateItemError {
@@ -516,6 +507,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_item::UpdateItemError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_item::UpdateItemError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_kinesis_streaming_destination.rs`

```diff
--- reference/src/operation/update_kinesis_streaming_destination.rs
+++ generated/src/operation/update_kinesis_streaming_destination.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -265,16 +273,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateKinesisStreamingDestination",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_kinesis_streaming_destination::ser_update_kinesis_streaming_destination_input(&input)?,
+            crate::protocol_serde::shape_update_kinesis_streaming_destination_input::ser_update_kinesis_streaming_destination_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -309,15 +312,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -344,7 +341,6 @@
 pub enum UpdateKinesisStreamingDestinationError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -490,6 +486,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_table.rs`

```diff
--- reference/src/operation/update_table.rs
+++ generated/src/operation/update_table.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UpdateTable", "DynamoDB"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_table::UpdateTableError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_table::UpdateTableError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_table::UpdateTableError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -247,15 +253,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateTable",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_table::ser_update_table_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_table_input::ser_update_table_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -289,15 +290,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -324,7 +319,6 @@
 pub enum UpdateTableError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -470,6 +464,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_table::UpdateTableError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_table::UpdateTableError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_table_replica_auto_scaling.rs`

```diff
--- reference/src/operation/update_table_replica_auto_scaling.rs
+++ generated/src/operation/update_table_replica_auto_scaling.rs
@@ -113,9 +113,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -256,16 +264,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateTableReplicaAutoScaling",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_table_replica_auto_scaling::ser_update_table_replica_auto_scaling_input(&input)?,
+            crate::protocol_serde::shape_update_table_replica_auto_scaling_input::ser_update_table_replica_auto_scaling_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -300,15 +303,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -471,6 +468,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_time_to_live.rs`

```diff
--- reference/src/operation/update_time_to_live.rs
+++ generated/src/operation/update_time_to_live.rs
@@ -107,9 +107,9 @@
             "DynamoDB",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,16 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_time_to_live::UpdateTimeToLiveError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_time_to_live::UpdateTimeToLiveError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_time_to_live::UpdateTimeToLiveError>::builder(
+                )
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,15 +257,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-amz-json-1.0");
-            builder = _header_serialization_settings.set_default_header(
-                builder,
-                ::http_1x::header::HeaderName::from_static("x-amz-target"),
-                "DynamoDB_20120810.UpdateTimeToLive",
-            );
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_time_to_live::ser_update_time_to_live_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_time_to_live_input::ser_update_time_to_live_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -292,15 +296,9 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
-            .set_account_id_endpoint_mode(::std::option::Option::Some(
-                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()
-                    .cloned()
-                    .unwrap_or_default()
-                    .to_string(),
-            ))
             .set_resource_arn(Some(
                 _input
                     .table_name
@@ -327,7 +325,6 @@
 pub enum UpdateTimeToLiveError {
     /// <p>An error occurred on the server side.</p>
     InternalServerError(crate::types::error::InternalServerError),
-    #[allow(missing_docs)] // documentation missing in model
     InvalidEndpointException(crate::types::error::InvalidEndpointException),
     /// <p>There is no limit to the number of daily on-demand backups that can be taken.</p>
     /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>.</p>
@@ -473,6 +470,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_time_to_live::UpdateTimeToLiveError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_time_to_live::UpdateTimeToLiveError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation.rs`

```diff
--- reference/src/operation.rs
+++ generated/src/operation.rs
@@ -1,176 +1,219 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use ::aws_types::request_id::RequestId;
-
-/// Types for the `BatchExecuteStatement` operation.
-pub mod batch_execute_statement;
-
-/// Types for the `BatchGetItem` operation.
-pub mod batch_get_item;
-
-/// Types for the `BatchWriteItem` operation.
-pub mod batch_write_item;
-
-/// Types for the `CreateBackup` operation.
-pub mod create_backup;
-
-/// Types for the `CreateGlobalTable` operation.
-pub mod create_global_table;
-
-/// Types for the `CreateTable` operation.
-pub mod create_table;
-
-/// Types for the `DeleteBackup` operation.
-pub mod delete_backup;
-
-/// Types for the `DeleteItem` operation.
-pub mod delete_item;
-
-/// Types for the `DeleteResourcePolicy` operation.
-pub mod delete_resource_policy;
-
-/// Types for the `DeleteTable` operation.
-pub mod delete_table;
-
-/// Types for the `DescribeBackup` operation.
-pub mod describe_backup;
-
-/// Types for the `DescribeContinuousBackups` operation.
-pub mod describe_continuous_backups;
-
-/// Types for the `DescribeContributorInsights` operation.
-pub mod describe_contributor_insights;
-
-/// Types for the `DescribeEndpoints` operation.
-pub mod describe_endpoints;
-
-/// Types for the `DescribeExport` operation.
-pub mod describe_export;
-
-/// Types for the `DescribeGlobalTable` operation.
-pub mod describe_global_table;

-/// Types for the `DescribeGlobalTableSettings` operation.
-pub mod describe_global_table_settings;
-
-/// Types for the `DescribeImport` operation.
-pub mod describe_import;
-
-/// Types for the `DescribeKinesisStreamingDestination` operation.
-pub mod describe_kinesis_streaming_destination;
-
-/// Types for the `DescribeLimits` operation.
-pub mod describe_limits;
-
-/// Types for the `DescribeTable` operation.
-pub mod describe_table;
-
-/// Types for the `DescribeTableReplicaAutoScaling` operation.
-pub mod describe_table_replica_auto_scaling;
-
-/// Types for the `DescribeTimeToLive` operation.
-pub mod describe_time_to_live;
-
-/// Types for the `DisableKinesisStreamingDestination` operation.
-pub mod disable_kinesis_streaming_destination;
-
-/// Types for the `EnableKinesisStreamingDestination` operation.
-pub mod enable_kinesis_streaming_destination;
-
-/// Types for the `ExecuteStatement` operation.
-pub mod execute_statement;
-
-/// Types for the `ExecuteTransaction` operation.
-pub mod execute_transaction;
-
-/// Types for the `ExportTableToPointInTime` operation.
-pub mod export_table_to_point_in_time;
-
-/// Types for the `GetItem` operation.
-pub mod get_item;
-
-/// Types for the `GetResourcePolicy` operation.
-pub mod get_resource_policy;
-
-/// Types for the `ImportTable` operation.
-pub mod import_table;
-
-/// Types for the `ListBackups` operation.
-pub mod list_backups;
-
-/// Types for the `ListContributorInsights` operation.
-pub mod list_contributor_insights;
-
-/// Types for the `ListExports` operation.
-pub mod list_exports;
-
-/// Types for the `ListGlobalTables` operation.
-pub mod list_global_tables;
-
-/// Types for the `ListImports` operation.
-pub mod list_imports;
-
-/// Types for the `ListTables` operation.
-pub mod list_tables;
-
-/// Types for the `ListTagsOfResource` operation.
-pub mod list_tags_of_resource;
-
-/// Types for the `PutItem` operation.
-pub mod put_item;
-
-/// Types for the `PutResourcePolicy` operation.
-pub mod put_resource_policy;
-
-/// Types for the `Query` operation.
-pub mod query;
-
-/// Types for the `RestoreTableFromBackup` operation.
-pub mod restore_table_from_backup;
-
-/// Types for the `RestoreTableToPointInTime` operation.
-pub mod restore_table_to_point_in_time;
-
-/// Types for the `Scan` operation.
-pub mod scan;
-
-/// Types for the `SearchVectors` operation.
-pub mod search_vectors;
-
-/// Types for the `TagResource` operation.
-pub mod tag_resource;
-
-/// Types for the `TransactGetItems` operation.
-pub mod transact_get_items;
-
-/// Types for the `TransactWriteItems` operation.
-pub mod transact_write_items;
-
-/// Types for the `UntagResource` operation.
-pub mod untag_resource;
+pub use ::aws_types::request_id::RequestId;

-/// Types for the `UpdateContinuousBackups` operation.
-pub mod update_continuous_backups;
-
-/// Types for the `UpdateContributorInsights` operation.
-pub mod update_contributor_insights;
-
-/// Types for the `UpdateGlobalTable` operation.
-pub mod update_global_table;
-
-/// Types for the `UpdateGlobalTableSettings` operation.
-pub mod update_global_table_settings;
-
-/// Types for the `UpdateItem` operation.
-pub mod update_item;
-
-/// Types for the `UpdateKinesisStreamingDestination` operation.
-pub mod update_kinesis_streaming_destination;
-
-/// Types for the `UpdateTable` operation.
-pub mod update_table;
-
-/// Types for the `UpdateTableReplicaAutoScaling` operation.
-pub mod update_table_replica_auto_scaling;
-
-/// Types for the `UpdateTimeToLive` operation.
-pub mod update_time_to_live;
+pub mod operation {
+    pub mod batch_execute_statement {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/batch_execute_statement.rs"));
+    }
+    pub mod batch_get_item {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/batch_get_item.rs"));
+    }
+    pub mod batch_write_item {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/batch_write_item.rs"));
+    }
+    pub mod create_backup {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/create_backup.rs"));
+    }
+    pub mod create_global_table {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/create_global_table.rs"));
+    }
+    pub mod create_table {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/create_table.rs"));
+    }
+    pub mod delete_backup {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/delete_backup.rs"));
+    }
+    pub mod delete_item {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/delete_item.rs"));
+    }
+    pub mod delete_resource_policy {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/delete_resource_policy.rs"));
+    }
+    pub mod delete_table {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/delete_table.rs"));
+    }
+    pub mod describe_backup {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/describe_backup.rs"));
+    }
+    pub mod describe_continuous_backups {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/describe_continuous_backups.rs"
+        ));
+    }
+    pub mod describe_contributor_insights {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/describe_contributor_insights.rs"
+        ));
+    }
+    pub mod describe_endpoints {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/describe_endpoints.rs"));
+    }
+    pub mod describe_export {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/describe_export.rs"));
+    }
+    pub mod describe_global_table {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/describe_global_table.rs"));
+    }
+    pub mod describe_global_table_settings {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/describe_global_table_settings.rs"
+        ));
+    }
+    pub mod describe_import {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/describe_import.rs"));
+    }
+    pub mod describe_kinesis_streaming_destination {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/describe_kinesis_streaming_destination.rs"
+        ));
+    }
+    pub mod describe_limits {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/describe_limits.rs"));
+    }
+    pub mod describe_table {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/describe_table.rs"));
+    }
+    pub mod describe_table_replica_auto_scaling {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/describe_table_replica_auto_scaling.rs"
+        ));
+    }
+    pub mod describe_time_to_live {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/describe_time_to_live.rs"));
+    }
+    pub mod disable_kinesis_streaming_destination {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/disable_kinesis_streaming_destination.rs"
+        ));
+    }
+    pub mod enable_kinesis_streaming_destination {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/enable_kinesis_streaming_destination.rs"
+        ));
+    }
+    pub mod execute_statement {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/execute_statement.rs"));
+    }
+    pub mod execute_transaction {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/execute_transaction.rs"));
+    }
+    pub mod export_table_to_point_in_time {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/export_table_to_point_in_time.rs"
+        ));
+    }
+    pub mod get_item {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/get_item.rs"));
+    }
+    pub mod get_resource_policy {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/get_resource_policy.rs"));
+    }
+    pub mod import_table {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/import_table.rs"));
+    }
+    pub mod list_backups {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/list_backups.rs"));
+    }
+    pub mod list_contributor_insights {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/list_contributor_insights.rs"));
+    }
+    pub mod list_exports {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/list_exports.rs"));
+    }
+    pub mod list_global_tables {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/list_global_tables.rs"));
+    }
+    pub mod list_imports {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/list_imports.rs"));
+    }
+    pub mod list_tables {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/list_tables.rs"));
+    }
+    pub mod list_tags_of_resource {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/list_tags_of_resource.rs"));
+    }
+    pub mod put_item {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/put_item.rs"));
+    }
+    pub mod put_resource_policy {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/put_resource_policy.rs"));
+    }
+    pub mod query {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/query.rs"));
+    }
+    pub mod restore_table_from_backup {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/restore_table_from_backup.rs"));
+    }
+    pub mod restore_table_to_point_in_time {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/restore_table_to_point_in_time.rs"
+        ));
+    }
+    pub mod scan {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/scan.rs"));
+    }
+    pub mod search_vectors {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/search_vectors.rs"));
+    }
+    pub mod tag_resource {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/tag_resource.rs"));
+    }
+    pub mod transact_get_items {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/transact_get_items.rs"));
+    }
+    pub mod transact_write_items {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/transact_write_items.rs"));
+    }
+    pub mod untag_resource {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/untag_resource.rs"));
+    }
+    pub mod update_continuous_backups {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/update_continuous_backups.rs"));
+    }
+    pub mod update_contributor_insights {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/update_contributor_insights.rs"
+        ));
+    }
+    pub mod update_global_table {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/update_global_table.rs"));
+    }
+    pub mod update_global_table_settings {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/update_global_table_settings.rs"
+        ));
+    }
+    pub mod update_item {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/update_item.rs"));
+    }
+    pub mod update_kinesis_streaming_destination {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/update_kinesis_streaming_destination.rs"
+        ));
+    }
+    pub mod update_table {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/update_table.rs"));
+    }
+    pub mod update_table_replica_auto_scaling {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/dynamodb/src/operation/update_table_replica_auto_scaling.rs"
+        ));
+    }
+    pub mod update_time_to_live {
+        include!(concat!(env!("OUT_DIR"), "/generated/dynamodb/src/operation/update_time_to_live.rs"));
+    }
+}
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -32,6 +32,31 @@
     builder
 }

+pub(crate) fn keys_and_attributes_correct_errors(
+    mut builder: crate::types::builders::KeysAndAttributesBuilder,
+) -> crate::types::builders::KeysAndAttributesBuilder {
+    if builder.keys.is_none() {
+        builder.keys = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn put_request_correct_errors(mut builder: crate::types::builders::PutRequestBuilder) -> crate::types::builders::PutRequestBuilder {
+    if builder.item.is_none() {
+        builder.item = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn delete_request_correct_errors(
+    mut builder: crate::types::builders::DeleteRequestBuilder,
+) -> crate::types::builders::DeleteRequestBuilder {
+    if builder.key.is_none() {
+        builder.key = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn backup_details_correct_errors(
     mut builder: crate::types::builders::BackupDetailsBuilder,
 ) -> crate::types::builders::BackupDetailsBuilder {
@@ -53,51 +78,35 @@
     builder
 }

-pub(crate) fn continuous_backups_description_correct_errors(
-    mut builder: crate::types::builders::ContinuousBackupsDescriptionBuilder,
-) -> crate::types::builders::ContinuousBackupsDescriptionBuilder {
-    if builder.continuous_backups_status.is_none() {
-        builder.continuous_backups_status = "no value was set".parse::<crate::types::ContinuousBackupsStatus>().ok()
-    }
-    builder
-}
-
-pub(crate) fn time_to_live_specification_correct_errors(
-    mut builder: crate::types::builders::TimeToLiveSpecificationBuilder,
-) -> crate::types::builders::TimeToLiveSpecificationBuilder {
-    if builder.enabled.is_none() {
-        builder.enabled = Some(Default::default())
-    }
+pub(crate) fn attribute_definition_correct_errors(
+    mut builder: crate::types::builders::AttributeDefinitionBuilder,
+) -> crate::types::builders::AttributeDefinitionBuilder {
     if builder.attribute_name.is_none() {
         builder.attribute_name = Some(Default::default())
     }
-    builder
-}
-
-pub(crate) fn endpoint_correct_errors(mut builder: crate::types::builders::EndpointBuilder) -> crate::types::builders::EndpointBuilder {
-    if builder.address.is_none() {
-        builder.address = Some(Default::default())
-    }
-    if builder.cache_period_in_minutes.is_none() {
-        builder.cache_period_in_minutes = Some(Default::default())
+    if builder.attribute_type.is_none() {
+        builder.attribute_type = "no value was set".parse::<crate::types::ScalarAttributeType>().ok()
     }
     builder
 }

-pub(crate) fn keys_and_attributes_correct_errors(
-    mut builder: crate::types::builders::KeysAndAttributesBuilder,
-) -> crate::types::builders::KeysAndAttributesBuilder {
-    if builder.keys.is_none() {
-        builder.keys = Some(Default::default())
+pub(crate) fn key_schema_element_correct_errors(
+    mut builder: crate::types::builders::KeySchemaElementBuilder,
+) -> crate::types::builders::KeySchemaElementBuilder {
+    if builder.attribute_name.is_none() {
+        builder.attribute_name = Some(Default::default())
+    }
+    if builder.key_type.is_none() {
+        builder.key_type = "no value was set".parse::<crate::types::KeyType>().ok()
     }
     builder
 }

-pub(crate) fn replica_settings_description_correct_errors(
-    mut builder: crate::types::builders::ReplicaSettingsDescriptionBuilder,
-) -> crate::types::builders::ReplicaSettingsDescriptionBuilder {
-    if builder.region_name.is_none() {
-        builder.region_name = Some(Default::default())
+pub(crate) fn stream_specification_correct_errors(
+    mut builder: crate::types::builders::StreamSpecificationBuilder,
+) -> crate::types::builders::StreamSpecificationBuilder {
+    if builder.stream_enabled.is_none() {
+        builder.stream_enabled = Some(Default::default())
     }
     builder
 }
@@ -114,11 +123,23 @@
     builder
 }

-pub(crate) fn s3_bucket_source_correct_errors(
-    mut builder: crate::types::builders::S3BucketSourceBuilder,
-) -> crate::types::builders::S3BucketSourceBuilder {
-    if builder.s3_bucket.is_none() {
-        builder.s3_bucket = Some(Default::default())
+pub(crate) fn search_schema_element_correct_errors(
+    mut builder: crate::types::builders::SearchSchemaElementBuilder,
+) -> crate::types::builders::SearchSchemaElementBuilder {
+    if builder.attribute_name.is_none() {
+        builder.attribute_name = Some(Default::default())
+    }
+    if builder.search_schema_element_type.is_none() {
+        builder.search_schema_element_type = "no value was set".parse::<crate::types::SearchSchemaElementType>().ok()
+    }
+    builder
+}
+
+pub(crate) fn vector_attribute_definition_correct_errors(
+    mut builder: crate::types::builders::VectorAttributeDefinitionBuilder,
+) -> crate::types::builders::VectorAttributeDefinitionBuilder {
+    if builder.attribute_name.is_none() {
+        builder.attribute_name = Some(Default::default())
     }
     builder
 }
@@ -147,81 +168,84 @@
     builder
 }

-pub(crate) fn stream_specification_correct_errors(
-    mut builder: crate::types::builders::StreamSpecificationBuilder,
-) -> crate::types::builders::StreamSpecificationBuilder {
-    if builder.stream_enabled.is_none() {
-        builder.stream_enabled = Some(Default::default())
+pub(crate) fn provisioned_throughput_correct_errors(
+    mut builder: crate::types::builders::ProvisionedThroughputBuilder,
+) -> crate::types::builders::ProvisionedThroughputBuilder {
+    if builder.read_capacity_units.is_none() {
+        builder.read_capacity_units = Some(Default::default())
     }
+    if builder.write_capacity_units.is_none() {
+        builder.write_capacity_units = Some(Default::default())
+    }
     builder
 }

-pub(crate) fn table_creation_parameters_correct_errors(
-    mut builder: crate::types::builders::TableCreationParametersBuilder,
-) -> crate::types::builders::TableCreationParametersBuilder {
-    if builder.table_name.is_none() {
-        builder.table_name = Some(Default::default())
+pub(crate) fn continuous_backups_description_correct_errors(
+    mut builder: crate::types::builders::ContinuousBackupsDescriptionBuilder,
+) -> crate::types::builders::ContinuousBackupsDescriptionBuilder {
+    if builder.continuous_backups_status.is_none() {
+        builder.continuous_backups_status = "no value was set".parse::<crate::types::ContinuousBackupsStatus>().ok()
     }
-    if builder.attribute_definitions.is_none() {
-        builder.attribute_definitions = Some(Default::default())
+    builder
+}
+
+pub(crate) fn endpoint_correct_errors(mut builder: crate::types::builders::EndpointBuilder) -> crate::types::builders::EndpointBuilder {
+    if builder.address.is_none() {
+        builder.address = Some(Default::default())
     }
-    if builder.key_schema.is_none() {
-        builder.key_schema = Some(Default::default())
+    if builder.cache_period_in_minutes.is_none() {
+        builder.cache_period_in_minutes = Some(Default::default())
     }
     builder
 }

-pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
-    if builder.key.is_none() {
-        builder.key = Some(Default::default())
-    }
-    if builder.value.is_none() {
-        builder.value = Some(Default::default())
+pub(crate) fn replica_settings_description_correct_errors(
+    mut builder: crate::types::builders::ReplicaSettingsDescriptionBuilder,
+) -> crate::types::builders::ReplicaSettingsDescriptionBuilder {
+    if builder.region_name.is_none() {
+        builder.region_name = Some(Default::default())
     }
     builder
 }

-pub(crate) fn attribute_definition_correct_errors(
-    mut builder: crate::types::builders::AttributeDefinitionBuilder,
-) -> crate::types::builders::AttributeDefinitionBuilder {
-    if builder.attribute_name.is_none() {
-        builder.attribute_name = Some(Default::default())
-    }
-    if builder.attribute_type.is_none() {
-        builder.attribute_type = "no value was set".parse::<crate::types::ScalarAttributeType>().ok()
+pub(crate) fn auto_scaling_target_tracking_scaling_policy_configuration_description_correct_errors(
+    mut builder: crate::types::builders::AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder,
+) -> crate::types::builders::AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder {
+    if builder.target_value.is_none() {
+        builder.target_value = Some(Default::default())
     }
     builder
 }

-pub(crate) fn key_schema_element_correct_errors(
-    mut builder: crate::types::builders::KeySchemaElementBuilder,
-) -> crate::types::builders::KeySchemaElementBuilder {
-    if builder.attribute_name.is_none() {
-        builder.attribute_name = Some(Default::default())
-    }
-    if builder.key_type.is_none() {
-        builder.key_type = "no value was set".parse::<crate::types::KeyType>().ok()
+pub(crate) fn replica_global_secondary_index_settings_description_correct_errors(
+    mut builder: crate::types::builders::ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder,
+) -> crate::types::builders::ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder {
+    if builder.index_name.is_none() {
+        builder.index_name = Some(Default::default())
     }
     builder
 }

-pub(crate) fn provisioned_throughput_correct_errors(
-    mut builder: crate::types::builders::ProvisionedThroughputBuilder,
-) -> crate::types::builders::ProvisionedThroughputBuilder {
-    if builder.read_capacity_units.is_none() {
-        builder.read_capacity_units = Some(Default::default())
-    }
-    if builder.write_capacity_units.is_none() {
-        builder.write_capacity_units = Some(Default::default())
+pub(crate) fn s3_bucket_source_correct_errors(
+    mut builder: crate::types::builders::S3BucketSourceBuilder,
+) -> crate::types::builders::S3BucketSourceBuilder {
+    if builder.s3_bucket.is_none() {
+        builder.s3_bucket = Some(Default::default())
     }
     builder
 }

-pub(crate) fn delete_request_correct_errors(
-    mut builder: crate::types::builders::DeleteRequestBuilder,
-) -> crate::types::builders::DeleteRequestBuilder {
-    if builder.key.is_none() {
-        builder.key = Some(Default::default())
+pub(crate) fn table_creation_parameters_correct_errors(
+    mut builder: crate::types::builders::TableCreationParametersBuilder,
+) -> crate::types::builders::TableCreationParametersBuilder {
+    if builder.table_name.is_none() {
+        builder.table_name = Some(Default::default())
+    }
+    if builder.attribute_definitions.is_none() {
+        builder.attribute_definitions = Some(Default::default())
+    }
+    if builder.key_schema.is_none() {
+        builder.key_schema = Some(Default::default())
     }
     builder
 }
@@ -244,31 +268,6 @@
     builder
 }

-pub(crate) fn put_request_correct_errors(mut builder: crate::types::builders::PutRequestBuilder) -> crate::types::builders::PutRequestBuilder {
-    if builder.item.is_none() {
-        builder.item = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn replica_global_secondary_index_settings_description_correct_errors(
-    mut builder: crate::types::builders::ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder,
-) -> crate::types::builders::ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder {
-    if builder.index_name.is_none() {
-        builder.index_name = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn vector_attribute_definition_correct_errors(
-    mut builder: crate::types::builders::VectorAttributeDefinitionBuilder,
-) -> crate::types::builders::VectorAttributeDefinitionBuilder {
-    if builder.attribute_name.is_none() {
-        builder.attribute_name = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn vector_index_correct_errors(mut builder: crate::types::builders::VectorIndexBuilder) -> crate::types::builders::VectorIndexBuilder {
     if builder.index_name.is_none() {
         builder.index_name = Some(Default::default())
@@ -294,23 +293,24 @@
     builder
 }

-pub(crate) fn search_schema_element_correct_errors(
-    mut builder: crate::types::builders::SearchSchemaElementBuilder,
-) -> crate::types::builders::SearchSchemaElementBuilder {
-    if builder.attribute_name.is_none() {
-        builder.attribute_name = Some(Default::default())
+pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
+    if builder.key.is_none() {
+        builder.key = Some(Default::default())
     }
-    if builder.search_schema_element_type.is_none() {
-        builder.search_schema_element_type = "no value was set".parse::<crate::types::SearchSchemaElementType>().ok()
+    if builder.value.is_none() {
+        builder.value = Some(Default::default())
     }
     builder
 }

-pub(crate) fn auto_scaling_target_tracking_scaling_policy_configuration_description_correct_errors(
-    mut builder: crate::types::builders::AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder,
-) -> crate::types::builders::AutoScalingTargetTrackingScalingPolicyConfigurationDescriptionBuilder {
-    if builder.target_value.is_none() {
-        builder.target_value = Some(Default::default())
+pub(crate) fn time_to_live_specification_correct_errors(
+    mut builder: crate::types::builders::TimeToLiveSpecificationBuilder,
+) -> crate::types::builders::TimeToLiveSpecificationBuilder {
+    if builder.enabled.is_none() {
+        builder.enabled = Some(Default::default())
+    }
+    if builder.attribute_name.is_none() {
+        builder.attribute_name = Some(Default::default())
     }
     builder
 }
```

### `src/types/_attribute_value.rs`

```diff
--- reference/src/types/_attribute_value.rs
+++ generated/src/types/_attribute_value.rs
@@ -8,13 +8,13 @@
 pub enum AttributeValue {
     /// <p>An attribute of type Binary. For example:</p>
     /// <p><code>"B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk"</code></p>
-    B(::aws_smithy_types::Blob),
+    B(::std::vec::Vec<u8>),
     /// <p>An attribute of type Boolean. For example:</p>
     /// <p><code>"BOOL": true</code></p>
     Bool(bool),
     /// <p>An attribute of type Binary Set. For example:</p>
     /// <p><code>"BS": \["U3Vubnk=", "UmFpbnk=", "U25vd3k="\]</code></p>
-    Bs(::std::vec::Vec<::aws_smithy_types::Blob>),
+    Bs(::std::vec::Vec<::std::vec::Vec<u8>>),
     /// <p>An attribute of type List. For example:</p>
     /// <p><code>"L": \[ {"S": "Cookies"} , {"S": "Coffee"}, {"N": "3.14159"}\]</code></p>
     L(::std::vec::Vec<crate::types::AttributeValue>),
@@ -49,9 +49,9 @@
     Unknown,
 }
 impl AttributeValue {
-    /// Tries to convert the enum instance into [`B`](crate::types::AttributeValue::B), extracting the inner [`Blob`](::aws_smithy_types::Blob).
+    /// Tries to convert the enum instance into [`B`](crate::types::AttributeValue::B), extracting the inner [`Blob`](::std::vec::Vec<u8>).
     /// Returns `Err(&Self)` if it can't be converted.
-    pub fn as_b(&self) -> ::std::result::Result<&::aws_smithy_types::Blob, &Self> {
+    pub fn as_b(&self) -> ::std::result::Result<&::std::vec::Vec<u8>, &Self> {
         if let AttributeValue::B(val) = &self {
             ::std::result::Result::Ok(val)
         } else {
@@ -75,9 +75,9 @@
     pub fn is_bool(&self) -> bool {
         self.as_bool().is_ok()
     }
-    /// Tries to convert the enum instance into [`Bs`](crate::types::AttributeValue::Bs), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`Bs`](crate::types::AttributeValue::Bs), extracting the inner [`Vec::<Blob>`](::std::vec::Vec<::std::vec::Vec<u8>>).
     /// Returns `Err(&Self)` if it can't be converted.
-    pub fn as_bs(&self) -> ::std::result::Result<&::std::vec::Vec<::aws_smithy_types::Blob>, &Self> {
+    pub fn as_bs(&self) -> ::std::result::Result<&::std::vec::Vec<::std::vec::Vec<u8>>, &Self> {
         if let AttributeValue::Bs(val) = &self {
             ::std::result::Result::Ok(val)
         } else {
@@ -88,7 +88,7 @@
     pub fn is_bs(&self) -> bool {
         self.as_bs().is_ok()
     }
-    /// Tries to convert the enum instance into [`L`](crate::types::AttributeValue::L), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`L`](crate::types::AttributeValue::L), extracting the inner [`Vec::<AttributeValue>`](::std::vec::Vec<crate::types::AttributeValue>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_l(&self) -> ::std::result::Result<&::std::vec::Vec<crate::types::AttributeValue>, &Self> {
         if let AttributeValue::L(val) = &self {
@@ -101,7 +101,7 @@
     pub fn is_l(&self) -> bool {
         self.as_l().is_ok()
     }
-    /// Tries to convert the enum instance into [`M`](crate::types::AttributeValue::M), extracting the inner [`HashMap`](::std::collections::HashMap).
+    /// Tries to convert the enum instance into [`M`](crate::types::AttributeValue::M), extracting the inner [`HashMap::<String, AttributeValue>`](::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_m(&self) -> ::std::result::Result<&::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>, &Self> {
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

### `src/types/_endpoint.rs`

```diff
--- reference/src/types/_endpoint.rs
+++ generated/src/types/_endpoint.rs
@@ -68,6 +68,7 @@
     /// Consumes the builder and constructs a [`Endpoint`](crate::types::Endpoint).
     /// This method will fail if any of the following fields are not set:
     /// - [`address`](crate::types::builders::EndpointBuilder::address)
+    /// - [`cache_period_in_minutes`](crate::types::builders::EndpointBuilder::cache_period_in_minutes)
     pub fn build(self) -> ::std::result::Result<crate::types::Endpoint, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::Endpoint {
             address: self.address.ok_or_else(|| {
@@ -76,7 +77,12 @@
                     "address was not specified but it is required when building Endpoint",
                 )
             })?,
-            cache_period_in_minutes: self.cache_period_in_minutes.unwrap_or_default(),
+            cache_period_in_minutes: self.cache_period_in_minutes.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "cache_period_in_minutes",
+                    "cache_period_in_minutes was not specified but it is required when building Endpoint",
+                )
+            })?,
         })
     }
 }
```

### `src/types/_import_table_description.rs`

```diff
--- reference/src/types/_import_table_description.rs
+++ generated/src/types/_import_table_description.rs
@@ -17,7 +17,7 @@
     /// <p>Values for the S3 bucket the source file is imported from. Includes bucket name (required), key prefix (optional) and bucket account owner ID (optional).</p>
     pub s3_bucket_source: ::std::option::Option<crate::types::S3BucketSource>,
     /// <p>The number of errors occurred on importing the source file into the target table.</p>
-    pub error_count: i64,
+    pub error_count: ::std::option::Option<i64>,
     /// <p>The Amazon Resource Number (ARN) of the Cloudwatch Log Group associated with the target table.</p>
     pub cloud_watch_log_group_arn: ::std::option::Option<::std::string::String>,
     /// <p>The format of the source data going into the target table.</p>
@@ -35,9 +35,9 @@
     /// <p>The total size of data processed from the source file, in Bytes.</p>
     pub processed_size_bytes: ::std::option::Option<i64>,
     /// <p>The total number of items processed from the source file.</p>
-    pub processed_item_count: i64,
+    pub processed_item_count: ::std::option::Option<i64>,
     /// <p>The number of items successfully imported into the new table.</p>
-    pub imported_item_count: i64,
+    pub imported_item_count: ::std::option::Option<i64>,
     /// <p>The error code corresponding to the failure that the import job ran into during execution.</p>
     pub failure_code: ::std::option::Option<::std::string::String>,
     /// <p>The error message corresponding to the failure that the import job ran into during execution.</p>
@@ -69,7 +69,7 @@
         self.s3_bucket_source.as_ref()
     }
     /// <p>The number of errors occurred on importing the source file into the target table.</p>
-    pub fn error_count(&self) -> i64 {
+    pub fn error_count(&self) -> ::std::option::Option<i64> {
         self.error_count
     }
     /// <p>The Amazon Resource Number (ARN) of the Cloudwatch Log Group associated with the target table.</p>
@@ -105,11 +105,11 @@
         self.processed_size_bytes
     }
     /// <p>The total number of items processed from the source file.</p>
-    pub fn processed_item_count(&self) -> i64 {
+    pub fn processed_item_count(&self) -> ::std::option::Option<i64> {
         self.processed_item_count
     }
     /// <p>The number of items successfully imported into the new table.</p>
-    pub fn imported_item_count(&self) -> i64 {
+    pub fn imported_item_count(&self) -> ::std::option::Option<i64> {
         self.imported_item_count
     }
     /// <p>The error code corresponding to the failure that the import job ran into during execution.</p>
@@ -428,7 +428,7 @@
             table_id: self.table_id,
             client_token: self.client_token,
             s3_bucket_source: self.s3_bucket_source,
-            error_count: self.error_count.unwrap_or_default(),
+            error_count: self.error_count,
             cloud_watch_log_group_arn: self.cloud_watch_log_group_arn,
             input_format: self.input_format,
             input_format_options: self.input_format_options,
@@ -437,8 +437,8 @@
             start_time: self.start_time,
             end_time: self.end_time,
             processed_size_bytes: self.processed_size_bytes,
-            processed_item_count: self.processed_item_count.unwrap_or_default(),
-            imported_item_count: self.imported_item_count.unwrap_or_default(),
+            processed_item_count: self.processed_item_count,
+            imported_item_count: self.imported_item_count,
             failure_code: self.failure_code,
             failure_message: self.failure_message,
         }
```

### `src/types/_return_consumed_capacity.rs`

```diff
--- reference/src/types/_return_consumed_capacity.rs
+++ generated/src/types/_return_consumed_capacity.rs
@@ -38,30 +38,30 @@
 /// - It might inadvertently shadow other intended match arms.
 ///
 /// <p>Determines the level of detail about either provisioned or on-demand throughput
-/// consumption that is returned in the response:</p>
-/// <ul>
-/// <li>
-/// <p>
-/// <code>INDEXES</code> - The response includes the aggregate
-/// <code>ConsumedCapacity</code> for the operation, together with
-/// <code>ConsumedCapacity</code> for each table and secondary index that was
-/// accessed.</p>
-/// <p>Note that some operations, such as <code>GetItem</code> and
-/// <code>BatchGetItem</code>, do not access any indexes at all. In these cases,
-/// specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code>
-/// information for table(s).</p>
-/// </li>
-/// <li>
-/// <p>
-/// <code>TOTAL</code> - The response includes only the aggregate
-/// <code>ConsumedCapacity</code> for the operation.</p>
-/// </li>
-/// <li>
-/// <p>
-/// <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the
-/// response.</p>
-/// </li>
-/// </ul>
+///             consumption that is returned in the response:</p>
+///          <ul>
+///             <li>
+///                <p>
+///                   <code>INDEXES</code> - The response includes the aggregate
+///                         <code>ConsumedCapacity</code> for the operation, together with
+///                         <code>ConsumedCapacity</code> for each table and secondary index that was
+///                     accessed.</p>
+///                <p>Note that some operations, such as <code>GetItem</code> and
+///                         <code>BatchGetItem</code>, do not access any indexes at all. In these cases,
+///                     specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code>
+///                     information for table(s).</p>
+///             </li>
+///             <li>
+///                <p>
+///                   <code>TOTAL</code> - The response includes only the aggregate
+///                         <code>ConsumedCapacity</code> for the operation.</p>
+///             </li>
+///             <li>
+///                <p>
+///                   <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the
+///                     response.</p>
+///             </li>
+///          </ul>
 #[non_exhaustive]
 #[derive(
     ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
```

### `src/types/_search_result_item.rs`

```diff
--- reference/src/types/_search_result_item.rs
+++ generated/src/types/_search_result_item.rs
@@ -7,7 +7,7 @@
     /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the projected attributes of the item returned by the vector search.</p>
     pub item: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
     /// <p>The similarity score for this item relative to the search vector. The interpretation depends on the distance function configured for the vector index.</p>
-    pub score: f64,
+    pub score: ::std::option::Option<f64>,
 }
 impl SearchResultItem {
     /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the projected attributes of the item returned by the vector search.</p>
@@ -15,7 +15,7 @@
         self.item.as_ref()
     }
     /// <p>The similarity score for this item relative to the search vector. The interpretation depends on the distance function configured for the vector index.</p>
-    pub fn score(&self) -> f64 {
+    pub fn score(&self) -> ::std::option::Option<f64> {
         self.score
     }
 }
@@ -75,7 +75,7 @@
     pub fn build(self) -> crate::types::SearchResultItem {
         crate::types::SearchResultItem {
             item: self.item,
-            score: self.score.unwrap_or_default(),
+            score: self.score,
         }
     }
 }
```

### `src/types/error/_invalid_endpoint_exception.rs`

```diff
--- reference/src/types/error/_invalid_endpoint_exception.rs
+++ generated/src/types/error/_invalid_endpoint_exception.rs
@@ -1,4 +1,5 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
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

### Missing reference files

- `Cargo.toml`
- `LICENSE`
- `README.md`
- `benches/deserialization_bench.rs`
- `benches/serialization_bench.rs`
- `src/account_id_endpoint.rs`
- `src/config/auth.rs`
- `src/config/endpoint.rs`
- `src/endpoint_lib/arn.rs`
- `src/endpoint_lib/bdd_interpreter.rs`
- `src/endpoint_lib/coalesce.rs`
- `src/endpoint_lib/diagnostic.rs`
- `src/endpoint_lib/host.rs`
- `src/endpoint_lib/parse_url.rs`
- `src/endpoint_lib/partition.rs`
- `src/endpoint_lib.rs`
- `src/error/sealed_unhandled.rs`
- `src/error_meta.rs`
- `src/json_errors.rs`
- `src/protocol_serde/shape_archival_summary.rs`
- `src/protocol_serde/shape_attribute_definition.rs`
- `src/protocol_serde/shape_attribute_definitions.rs`
- `src/protocol_serde/shape_attribute_map.rs`
- `src/protocol_serde/shape_attribute_name_list.rs`
- `src/protocol_serde/shape_attribute_value.rs`
- `src/protocol_serde/shape_attribute_value_update.rs`
- `src/protocol_serde/shape_auto_scaling_policy_description.rs`
- `src/protocol_serde/shape_auto_scaling_policy_description_list.rs`
- `src/protocol_serde/shape_auto_scaling_policy_update.rs`
- `src/protocol_serde/shape_auto_scaling_settings_description.rs`
- `src/protocol_serde/shape_auto_scaling_settings_update.rs`
- `src/protocol_serde/shape_auto_scaling_target_tracking_scaling_policy_configuration_description.rs`
- `src/protocol_serde/shape_auto_scaling_target_tracking_scaling_policy_configuration_update.rs`
- `src/protocol_serde/shape_backup_description.rs`
- `src/protocol_serde/shape_backup_details.rs`
- `src/protocol_serde/shape_backup_in_use_exception.rs`
- `src/protocol_serde/shape_backup_not_found_exception.rs`
- `src/protocol_serde/shape_backup_summaries.rs`
- `src/protocol_serde/shape_backup_summary.rs`
- `src/protocol_serde/shape_batch_execute_statement.rs`
- `src/protocol_serde/shape_batch_execute_statement_input.rs`
- `src/protocol_serde/shape_batch_get_item.rs`
- `src/protocol_serde/shape_batch_get_item_input.rs`
- `src/protocol_serde/shape_batch_get_request_map.rs`
- `src/protocol_serde/shape_batch_get_response_map.rs`
- `src/protocol_serde/shape_batch_statement_error.rs`
- `src/protocol_serde/shape_batch_statement_request.rs`
- `src/protocol_serde/shape_batch_statement_response.rs`
- `src/protocol_serde/shape_batch_write_item.rs`
- `src/protocol_serde/shape_batch_write_item_input.rs`
- `src/protocol_serde/shape_batch_write_item_request_map.rs`
- `src/protocol_serde/shape_billing_mode_summary.rs`
- `src/protocol_serde/shape_binary_set_attribute_value.rs`
- `src/protocol_serde/shape_cancellation_reason.rs`
- `src/protocol_serde/shape_cancellation_reason_list.rs`
- `src/protocol_serde/shape_capacity.rs`
- `src/protocol_serde/shape_condition.rs`
- `src/protocol_serde/shape_condition_check.rs`
- `src/protocol_serde/shape_conditional_check_failed_exception.rs`
- `src/protocol_serde/shape_consumed_capacity.rs`
- `src/protocol_serde/shape_consumed_capacity_multiple.rs`
- `src/protocol_serde/shape_continuous_backups_description.rs`
- `src/protocol_serde/shape_continuous_backups_unavailable_exception.rs`
- `src/protocol_serde/shape_contributor_insights_rule_list.rs`
- `src/protocol_serde/shape_contributor_insights_summaries.rs`
- `src/protocol_serde/shape_contributor_insights_summary.rs`
- `src/protocol_serde/shape_create_backup.rs`
- `src/protocol_serde/shape_create_backup_input.rs`
- `src/protocol_serde/shape_create_global_secondary_index_action.rs`
- `src/protocol_serde/shape_create_global_table.rs`
- `src/protocol_serde/shape_create_global_table_input.rs`
- `src/protocol_serde/shape_create_global_table_witness_group_member_action.rs`
- `src/protocol_serde/shape_create_replica_action.rs`
- `src/protocol_serde/shape_create_replication_group_member_action.rs`
- `src/protocol_serde/shape_create_table.rs`
- `src/protocol_serde/shape_create_table_input.rs`
- `src/protocol_serde/shape_create_vector_index_action.rs`
- `src/protocol_serde/shape_csv_header_list.rs`
- `src/protocol_serde/shape_csv_options.rs`
- `src/protocol_serde/shape_delete.rs`
- `src/protocol_serde/shape_delete_backup.rs`
- `src/protocol_serde/shape_delete_backup_input.rs`
- `src/protocol_serde/shape_delete_global_secondary_index_action.rs`
- `src/protocol_serde/shape_delete_global_table_witness_group_member_action.rs`
- `src/protocol_serde/shape_delete_item.rs`
- `src/protocol_serde/shape_delete_item_input.rs`
- `src/protocol_serde/shape_delete_replica_action.rs`
- `src/protocol_serde/shape_delete_replication_group_member_action.rs`
- `src/protocol_serde/shape_delete_request.rs`
- `src/protocol_serde/shape_delete_resource_policy.rs`
- `src/protocol_serde/shape_delete_resource_policy_input.rs`
- `src/protocol_serde/shape_delete_table.rs`
- `src/protocol_serde/shape_delete_table_input.rs`
- `src/protocol_serde/shape_delete_vector_index_action.rs`
- `src/protocol_serde/shape_describe_backup.rs`
- `src/protocol_serde/shape_describe_backup_input.rs`
- `src/protocol_serde/shape_describe_continuous_backups.rs`
- `src/protocol_serde/shape_describe_continuous_backups_input.rs`
- `src/protocol_serde/shape_describe_contributor_insights.rs`
- `src/protocol_serde/shape_describe_contributor_insights_input.rs`
- `src/protocol_serde/shape_describe_endpoints.rs`
- `src/protocol_serde/shape_describe_export.rs`
- `src/protocol_serde/shape_describe_export_input.rs`
- `src/protocol_serde/shape_describe_global_table.rs`
- `src/protocol_serde/shape_describe_global_table_input.rs`
- `src/protocol_serde/shape_describe_global_table_settings.rs`
- `src/protocol_serde/shape_describe_global_table_settings_input.rs`
- `src/protocol_serde/shape_describe_import.rs`
- `src/protocol_serde/shape_describe_import_input.rs`
- `src/protocol_serde/shape_describe_kinesis_streaming_destination.rs`
- `src/protocol_serde/shape_describe_kinesis_streaming_destination_input.rs`
- `src/protocol_serde/shape_describe_limits.rs`
- `src/protocol_serde/shape_describe_table.rs`
- `src/protocol_serde/shape_describe_table_input.rs`
- `src/protocol_serde/shape_describe_table_replica_auto_scaling.rs`
- `src/protocol_serde/shape_describe_table_replica_auto_scaling_input.rs`
- `src/protocol_serde/shape_describe_time_to_live.rs`
- `src/protocol_serde/shape_describe_time_to_live_input.rs`
- `src/protocol_serde/shape_disable_kinesis_streaming_destination.rs`
- `src/protocol_serde/shape_disable_kinesis_streaming_destination_input.rs`
- `src/protocol_serde/shape_duplicate_item_exception.rs`
- `src/protocol_serde/shape_enable_kinesis_streaming_configuration.rs`
- `src/protocol_serde/shape_enable_kinesis_streaming_destination.rs`
- `src/protocol_serde/shape_enable_kinesis_streaming_destination_input.rs`
- `src/protocol_serde/shape_endpoint.rs`
- `src/protocol_serde/shape_endpoints.rs`
- `src/protocol_serde/shape_execute_statement.rs`
- `src/protocol_serde/shape_execute_statement_input.rs`
- `src/protocol_serde/shape_execute_transaction.rs`
- `src/protocol_serde/shape_execute_transaction_input.rs`
- `src/protocol_serde/shape_expected_attribute_value.rs`
- `src/protocol_serde/shape_export_conflict_exception.rs`
- `src/protocol_serde/shape_export_description.rs`
- `src/protocol_serde/shape_export_not_found_exception.rs`
- `src/protocol_serde/shape_export_summaries.rs`
- `src/protocol_serde/shape_export_summary.rs`
- `src/protocol_serde/shape_export_table_to_point_in_time.rs`
- `src/protocol_serde/shape_export_table_to_point_in_time_input.rs`
- `src/protocol_serde/shape_expression_attribute_name_map.rs`
- `src/protocol_serde/shape_failure_exception.rs`
- `src/protocol_serde/shape_get.rs`
- `src/protocol_serde/shape_get_item.rs`
- `src/protocol_serde/shape_get_item_input.rs`
- `src/protocol_serde/shape_get_resource_policy.rs`
- `src/protocol_serde/shape_get_resource_policy_input.rs`
- `src/protocol_serde/shape_global_secondary_index.rs`
- `src/protocol_serde/shape_global_secondary_index_auto_scaling_update.rs`
- `src/protocol_serde/shape_global_secondary_index_description.rs`
- `src/protocol_serde/shape_global_secondary_index_description_list.rs`
- `src/protocol_serde/shape_global_secondary_index_info.rs`
- `src/protocol_serde/shape_global_secondary_index_list.rs`
- `src/protocol_serde/shape_global_secondary_index_update.rs`
- `src/protocol_serde/shape_global_secondary_index_warm_throughput_description.rs`
- `src/protocol_serde/shape_global_secondary_indexes.rs`
- `src/protocol_serde/shape_global_table.rs`
- `src/protocol_serde/shape_global_table_already_exists_exception.rs`
- `src/protocol_serde/shape_global_table_description.rs`
- `src/protocol_serde/shape_global_table_global_secondary_index_settings_update.rs`
- `src/protocol_serde/shape_global_table_list.rs`
- `src/protocol_serde/shape_global_table_not_found_exception.rs`
- `src/protocol_serde/shape_global_table_witness_description.rs`
- `src/protocol_serde/shape_global_table_witness_description_list.rs`
- `src/protocol_serde/shape_global_table_witness_group_update.rs`
- `src/protocol_serde/shape_idempotent_parameter_mismatch_exception.rs`
- `src/protocol_serde/shape_import_conflict_exception.rs`
- `src/protocol_serde/shape_import_not_found_exception.rs`
- `src/protocol_serde/shape_import_summary.rs`
- `src/protocol_serde/shape_import_summary_list.rs`
- `src/protocol_serde/shape_import_table.rs`
- `src/protocol_serde/shape_import_table_description.rs`
- `src/protocol_serde/shape_import_table_input.rs`
- `src/protocol_serde/shape_incremental_export_specification.rs`
- `src/protocol_serde/shape_index_not_found_exception.rs`
- `src/protocol_serde/shape_input_format_options.rs`
- `src/protocol_serde/shape_internal_server_error.rs`
- `src/protocol_serde/shape_invalid_endpoint_exception.rs`
- `src/protocol_serde/shape_invalid_export_time_exception.rs`
- `src/protocol_serde/shape_invalid_restore_time_exception.rs`
- `src/protocol_serde/shape_item_collection_key_attribute_map.rs`
- `src/protocol_serde/shape_item_collection_metrics.rs`
- `src/protocol_serde/shape_item_collection_metrics_multiple.rs`
- `src/protocol_serde/shape_item_collection_metrics_per_table.rs`
- `src/protocol_serde/shape_item_collection_size_estimate_range.rs`
- `src/protocol_serde/shape_item_collection_size_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_item_list.rs`
- `src/protocol_serde/shape_item_response.rs`
- `src/protocol_serde/shape_item_response_list.rs`
- `src/protocol_serde/shape_key.rs`
- `src/protocol_serde/shape_key_list.rs`
- `src/protocol_serde/shape_key_schema.rs`
- `src/protocol_serde/shape_key_schema_element.rs`
- `src/protocol_serde/shape_keys_and_attributes.rs`
- `src/protocol_serde/shape_kinesis_data_stream_destination.rs`
- `src/protocol_serde/shape_kinesis_data_stream_destinations.rs`
- `src/protocol_serde/shape_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_list_attribute_value.rs`
- `src/protocol_serde/shape_list_backups.rs`
- `src/protocol_serde/shape_list_backups_input.rs`
- `src/protocol_serde/shape_list_contributor_insights.rs`
- `src/protocol_serde/shape_list_contributor_insights_input.rs`
- `src/protocol_serde/shape_list_exports.rs`
- `src/protocol_serde/shape_list_exports_input.rs`
- `src/protocol_serde/shape_list_global_tables.rs`
- `src/protocol_serde/shape_list_global_tables_input.rs`
- `src/protocol_serde/shape_list_imports.rs`
- `src/protocol_serde/shape_list_imports_input.rs`
- `src/protocol_serde/shape_list_tables.rs`
- `src/protocol_serde/shape_list_tables_input.rs`
- `src/protocol_serde/shape_list_tags_of_resource.rs`
- `src/protocol_serde/shape_list_tags_of_resource_input.rs`
- `src/protocol_serde/shape_local_secondary_index.rs`
- `src/protocol_serde/shape_local_secondary_index_description.rs`
- `src/protocol_serde/shape_local_secondary_index_description_list.rs`
- `src/protocol_serde/shape_local_secondary_index_info.rs`
- `src/protocol_serde/shape_local_secondary_indexes.rs`
- `src/protocol_serde/shape_map_attribute_value.rs`
- `src/protocol_serde/shape_non_key_attribute_name_list.rs`
- `src/protocol_serde/shape_number_set_attribute_value.rs`
- `src/protocol_serde/shape_on_demand_throughput.rs`
- `src/protocol_serde/shape_on_demand_throughput_override.rs`
- `src/protocol_serde/shape_parameterized_statement.rs`
- `src/protocol_serde/shape_parti_ql_batch_response.rs`
- `src/protocol_serde/shape_point_in_time_recovery_description.rs`
- `src/protocol_serde/shape_point_in_time_recovery_specification.rs`
- `src/protocol_serde/shape_point_in_time_recovery_unavailable_exception.rs`
- `src/protocol_serde/shape_policy_not_found_exception.rs`
- `src/protocol_serde/shape_projection.rs`
- `src/protocol_serde/shape_provisioned_throughput.rs`
- `src/protocol_serde/shape_provisioned_throughput_description.rs`
- `src/protocol_serde/shape_provisioned_throughput_exceeded_exception.rs`
- `src/protocol_serde/shape_provisioned_throughput_override.rs`
- `src/protocol_serde/shape_put.rs`
- `src/protocol_serde/shape_put_item.rs`
- `src/protocol_serde/shape_put_item_input.rs`
- `src/protocol_serde/shape_put_item_input_attribute_map.rs`
- `src/protocol_serde/shape_put_request.rs`
- `src/protocol_serde/shape_put_resource_policy.rs`
- `src/protocol_serde/shape_put_resource_policy_input.rs`
- `src/protocol_serde/shape_query.rs`
- `src/protocol_serde/shape_query_input.rs`
- `src/protocol_serde/shape_replica.rs`
- `src/protocol_serde/shape_replica_already_exists_exception.rs`
- `src/protocol_serde/shape_replica_auto_scaling_description.rs`
- `src/protocol_serde/shape_replica_auto_scaling_description_list.rs`
- `src/protocol_serde/shape_replica_auto_scaling_update.rs`
- `src/protocol_serde/shape_replica_description.rs`
- `src/protocol_serde/shape_replica_description_list.rs`
- `src/protocol_serde/shape_replica_global_secondary_index.rs`
- `src/protocol_serde/shape_replica_global_secondary_index_auto_scaling_description.rs`
- `src/protocol_serde/shape_replica_global_secondary_index_auto_scaling_description_list.rs`
- `src/protocol_serde/shape_replica_global_secondary_index_auto_scaling_update.rs`
- `src/protocol_serde/shape_replica_global_secondary_index_description.rs`
- `src/protocol_serde/shape_replica_global_secondary_index_description_list.rs`
- `src/protocol_serde/shape_replica_global_secondary_index_settings_description.rs`
- `src/protocol_serde/shape_replica_global_secondary_index_settings_description_list.rs`
- `src/protocol_serde/shape_replica_global_secondary_index_settings_update.rs`
- `src/protocol_serde/shape_replica_list.rs`
- `src/protocol_serde/shape_replica_not_found_exception.rs`
- `src/protocol_serde/shape_replica_settings_description.rs`
- `src/protocol_serde/shape_replica_settings_description_list.rs`
- `src/protocol_serde/shape_replica_settings_update.rs`
- `src/protocol_serde/shape_replica_update.rs`
- `src/protocol_serde/shape_replicated_write_conflict_exception.rs`
- `src/protocol_serde/shape_replication_group_update.rs`
- `src/protocol_serde/shape_request_limit_exceeded.rs`
- `src/protocol_serde/shape_resource_in_use_exception.rs`
- `src/protocol_serde/shape_resource_not_found_exception.rs`
- `src/protocol_serde/shape_restore_summary.rs`
- `src/protocol_serde/shape_restore_table_from_backup.rs`
- `src/protocol_serde/shape_restore_table_from_backup_input.rs`
- `src/protocol_serde/shape_restore_table_to_point_in_time.rs`
- `src/protocol_serde/shape_restore_table_to_point_in_time_input.rs`
- `src/protocol_serde/shape_s3_bucket_source.rs`
- `src/protocol_serde/shape_scan.rs`
- `src/protocol_serde/shape_scan_input.rs`
- `src/protocol_serde/shape_search_result_item.rs`
- `src/protocol_serde/shape_search_result_list.rs`
- `src/protocol_serde/shape_search_schema.rs`
- `src/protocol_serde/shape_search_schema_element.rs`
- `src/protocol_serde/shape_search_vectors.rs`
- `src/protocol_serde/shape_search_vectors_input.rs`
- `src/protocol_serde/shape_secondary_indexes_capacity_map.rs`
- `src/protocol_serde/shape_source_table_details.rs`
- `src/protocol_serde/shape_source_table_feature_details.rs`
- `src/protocol_serde/shape_sse_description.rs`
- `src/protocol_serde/shape_sse_specification.rs`
- `src/protocol_serde/shape_stream_specification.rs`
- `src/protocol_serde/shape_string_set_attribute_value.rs`
- `src/protocol_serde/shape_table_already_exists_exception.rs`
- `src/protocol_serde/shape_table_auto_scaling_description.rs`
- `src/protocol_serde/shape_table_class_summary.rs`
- `src/protocol_serde/shape_table_creation_parameters.rs`
- `src/protocol_serde/shape_table_description.rs`
- `src/protocol_serde/shape_table_in_use_exception.rs`
- `src/protocol_serde/shape_table_name_list.rs`
- `src/protocol_serde/shape_table_not_found_exception.rs`
- `src/protocol_serde/shape_table_warm_throughput_description.rs`
- `src/protocol_serde/shape_tag.rs`
- `src/protocol_serde/shape_tag_list.rs`
- `src/protocol_serde/shape_tag_resource.rs`
- `src/protocol_serde/shape_tag_resource_input.rs`
- `src/protocol_serde/shape_throttling_exception.rs`
- `src/protocol_serde/shape_throttling_reason.rs`
- `src/protocol_serde/shape_throttling_reason_list.rs`
- `src/protocol_serde/shape_time_to_live_description.rs`
- `src/protocol_serde/shape_time_to_live_specification.rs`
- `src/protocol_serde/shape_transact_get_item.rs`
- `src/protocol_serde/shape_transact_get_items.rs`
- `src/protocol_serde/shape_transact_get_items_input.rs`
- `src/protocol_serde/shape_transact_write_item.rs`
- `src/protocol_serde/shape_transact_write_items.rs`
- `src/protocol_serde/shape_transact_write_items_input.rs`
- `src/protocol_serde/shape_transaction_canceled_exception.rs`
- `src/protocol_serde/shape_transaction_conflict_exception.rs`
- `src/protocol_serde/shape_transaction_in_progress_exception.rs`
- `src/protocol_serde/shape_untag_resource.rs`
- `src/protocol_serde/shape_untag_resource_input.rs`
- `src/protocol_serde/shape_update.rs`
- `src/protocol_serde/shape_update_continuous_backups.rs`
- `src/protocol_serde/shape_update_continuous_backups_input.rs`
- `src/protocol_serde/shape_update_contributor_insights.rs`
- `src/protocol_serde/shape_update_contributor_insights_input.rs`
- `src/protocol_serde/shape_update_global_secondary_index_action.rs`
- `src/protocol_serde/shape_update_global_table.rs`
- `src/protocol_serde/shape_update_global_table_input.rs`
- `src/protocol_serde/shape_update_global_table_settings.rs`
- `src/protocol_serde/shape_update_global_table_settings_input.rs`
- `src/protocol_serde/shape_update_item.rs`
- `src/protocol_serde/shape_update_item_input.rs`
- `src/protocol_serde/shape_update_kinesis_streaming_configuration.rs`
- `src/protocol_serde/shape_update_kinesis_streaming_destination.rs`
- `src/protocol_serde/shape_update_kinesis_streaming_destination_input.rs`
- `src/protocol_serde/shape_update_replication_group_member_action.rs`
- `src/protocol_serde/shape_update_table.rs`
- `src/protocol_serde/shape_update_table_input.rs`
- `src/protocol_serde/shape_update_table_replica_auto_scaling.rs`
- `src/protocol_serde/shape_update_table_replica_auto_scaling_input.rs`
- `src/protocol_serde/shape_update_time_to_live.rs`
- `src/protocol_serde/shape_update_time_to_live_input.rs`
- `src/protocol_serde/shape_vector_attribute_definition.rs`
- `src/protocol_serde/shape_vector_capacity.rs`
- `src/protocol_serde/shape_vector_index.rs`
- `src/protocol_serde/shape_vector_index_description.rs`
- `src/protocol_serde/shape_vector_index_description_list.rs`
- `src/protocol_serde/shape_vector_index_info.rs`
- `src/protocol_serde/shape_vector_index_list.rs`
- `src/protocol_serde/shape_vector_index_update.rs`
- `src/protocol_serde/shape_vector_indexes.rs`
- `src/protocol_serde/shape_vector_indexes_capacity_map.rs`
- `src/protocol_serde/shape_warm_throughput.rs`
- `src/protocol_serde/shape_write_request.rs`
- `src/protocol_serde/shape_write_requests.rs`
- `src/protocol_serde.rs`
- `src/serialization_settings.rs`
- `tests/account-based-endpoints.rs`
- `tests/auth_scheme_error.rs`
- `tests/build-errors.rs`
- `tests/cloning.rs`
- `tests/data.json`
- `tests/endpoint_tests.rs`
- `tests/endpoints.rs`
- `tests/ignore_configured_endpoint_urls.rs`
- `tests/movies.rs`
- `tests/paginators.rs`
- `tests/protocol-swap.rs`
- `tests/retries-with-client-rate-limiting.rs`
- `tests/retry-spec.rs`
- `tests/shared-config.rs`
- `tests/test-error-classification.rs`
- `tests/timeouts.rs`

### Rust token differences

- `src/client/batch_get_item.rs`
- `src/client/create_table.rs`
- `src/client/delete_item.rs`
- `src/client/get_item.rs`
- `src/client/put_item.rs`
- `src/client/query.rs`
- `src/client/scan.rs`
- `src/client/update_item.rs`
- `src/client.rs`
- `src/config.rs`
- `src/error.rs`
- `src/lib.rs`
- `src/operation/batch_execute_statement.rs`
- `src/operation/batch_get_item.rs`
- `src/operation/batch_write_item.rs`
- `src/operation/create_backup.rs`
- `src/operation/create_global_table.rs`
- `src/operation/create_table.rs`
- `src/operation/delete_backup.rs`
- `src/operation/delete_item.rs`
- `src/operation/delete_resource_policy.rs`
- `src/operation/delete_table.rs`
- `src/operation/describe_backup.rs`
- `src/operation/describe_continuous_backups.rs`
- `src/operation/describe_contributor_insights.rs`
- `src/operation/describe_endpoints.rs`
- `src/operation/describe_export.rs`
- `src/operation/describe_global_table.rs`
- `src/operation/describe_global_table_settings.rs`
- `src/operation/describe_import.rs`
- `src/operation/describe_kinesis_streaming_destination.rs`
- `src/operation/describe_limits.rs`
- `src/operation/describe_table.rs`
- `src/operation/describe_table_replica_auto_scaling.rs`
- `src/operation/describe_time_to_live.rs`
- `src/operation/disable_kinesis_streaming_destination.rs`
- `src/operation/enable_kinesis_streaming_destination.rs`
- `src/operation/execute_statement.rs`
- `src/operation/execute_transaction.rs`
- `src/operation/export_table_to_point_in_time.rs`
- `src/operation/get_item.rs`
- `src/operation/get_resource_policy.rs`
- `src/operation/import_table.rs`
- `src/operation/list_backups.rs`
- `src/operation/list_contributor_insights.rs`
- `src/operation/list_exports.rs`
- `src/operation/list_global_tables.rs`
- `src/operation/list_imports.rs`
- `src/operation/list_tables.rs`
- `src/operation/list_tags_of_resource.rs`
- `src/operation/put_item.rs`
- `src/operation/put_resource_policy.rs`
- `src/operation/query/_query_output.rs`
- `src/operation/query.rs`
- `src/operation/restore_table_from_backup.rs`
- `src/operation/restore_table_to_point_in_time.rs`
- `src/operation/scan/_scan_output.rs`
- `src/operation/scan.rs`
- `src/operation/search_vectors.rs`
- `src/operation/tag_resource.rs`
- `src/operation/transact_get_items.rs`
- `src/operation/transact_write_items/builders.rs`
- `src/operation/transact_write_items.rs`
- `src/operation/untag_resource.rs`
- `src/operation/update_continuous_backups.rs`
- `src/operation/update_contributor_insights.rs`
- `src/operation/update_global_table.rs`
- `src/operation/update_global_table_settings.rs`
- `src/operation/update_item.rs`
- `src/operation/update_kinesis_streaming_destination.rs`
- `src/operation/update_table.rs`
- `src/operation/update_table_replica_auto_scaling.rs`
- `src/operation/update_time_to_live.rs`
- `src/operation.rs`
- `src/serde_util.rs`
- `src/types/_attribute_value.rs`
- `src/types/_endpoint.rs`
- `src/types/_import_table_description.rs`
- `src/types/_return_consumed_capacity.rs`
- `src/types/_search_result_item.rs`
- `src/types/error/_replicated_write_conflict_exception.rs`
