# AWS SDK Conformance Report: dynamodb

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## dynamodb
**Progress:** `882/882` files compared · `859` matched · `23` mismatches · `0` missing · `0` extra · `97.39%` match (100.00% means fully matched)

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

### `src/config/endpoint.rs`

```diff
--- reference/src/config/endpoint.rs
+++ generated/src/config/endpoint.rs
@@ -153,422 +153,396 @@
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
-                        },
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
+                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url({
+                                        let mut out = String::new();
+                                        #[allow(clippy::needless_borrow)]
+                                        out.push_str(&endpoint.as_ref());
+                                        out
+                                    }).build())
                         },
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
-11 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS and DualStack are enabled, but this partition does not support one or both"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
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
-23 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Credentials-sourced account ID parameter is invalid"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
-24 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("AccountIdEndpointMode is required but no AccountID was provided or able to be loaded"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
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
-28 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("DualStack is enabled but this partition does not support DualStack"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
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
-35 => {
-
-                            ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid Configuration: Missing Region"
-.to_string())) as ::aws_smithy_runtime_api::box_error::BoxError)
-                        },
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
@@ -626,10 +600,8 @@
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
@@ -658,10 +630,8 @@
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
@@ -704,7 +674,7 @@
                         16 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_2 = &context.parsed_arn_ssa_2;
                             let partition_resolver = &self.partition_resolver;
-                            &mut Some(
+                            (&mut Some(
                                 (if let Some(inner) = parsed_arn_ssa_2 {
                                     inner.region()
                                 } else {
@@ -711,7 +681,7 @@
                                     return false;
                                 }
                                 .into()),
-                            ) == (region)
+                            )) == region
                         })(&mut _diagnostic_collector),
                         17 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_2 = &context.parsed_arn_ssa_2;
@@ -754,10 +724,11 @@
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
@@ -777,7 +748,7 @@
                         23 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_1 = &context.parsed_arn_ssa_1;
                             let partition_resolver = &self.partition_resolver;
-                            &mut Some(
+                            (&mut Some(
                                 (if let Some(inner) = parsed_arn_ssa_1 {
                                     inner.region()
                                 } else {
@@ -784,7 +755,7 @@
                                     return false;
                                 }
                                 .into()),
-                            ) == (region)
+                            )) == region
                         })(&mut _diagnostic_collector),
                         24 => (|_diagnostic_collector: &mut super::super::endpoint_lib::diagnostic::DiagnosticCollector| -> bool {
                             let parsed_arn_ssa_1 = &context.parsed_arn_ssa_1;
```

### `src/operation/describe_endpoints.rs`

```diff
--- reference/src/operation/describe_endpoints.rs
+++ generated/src/operation/describe_endpoints.rs
@@ -204,15 +204,9 @@
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
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_describe_endpoints::ser_describe_endpoints_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
```

### `src/operation/describe_limits.rs`

```diff
--- reference/src/operation/describe_limits.rs
+++ generated/src/operation/describe_limits.rs
@@ -204,15 +204,9 @@
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
-        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_describe_limits::ser_describe_limits_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from("");

         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
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

### `src/operation/put_resource_policy/_put_resource_policy_input.rs`

```diff
--- reference/src/operation/put_resource_policy/_put_resource_policy_input.rs
+++ generated/src/operation/put_resource_policy/_put_resource_policy_input.rs
@@ -168,7 +168,7 @@
             resource_arn: self.resource_arn,
             policy: self.policy,
             expected_revision_id: self.expected_revision_id,
-            confirm_remove_self_resource_access: self.confirm_remove_self_resource_access,
+            confirm_remove_self_resource_access: self.confirm_remove_self_resource_access.unwrap_or_default(),
         })
     }
 }
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

### `src/protocol_serde/shape_describe_endpoints.rs`

```diff
--- reference/src/protocol_serde/shape_describe_endpoints.rs
+++ generated/src/protocol_serde/shape_describe_endpoints.rs
@@ -33,12 +33,6 @@
     })
 }

-pub fn ser_describe_endpoints_input(
-    _input: &super::super::operation::describe_endpoints::DescribeEndpointsInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
-    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
-}
-
 pub(crate) fn de_describe_endpoints(
     _value: &[u8],
     mut builder: super::super::operation::describe_endpoints::builders::DescribeEndpointsOutputBuilder,
```

### `src/protocol_serde/shape_describe_limits.rs`

```diff
--- reference/src/protocol_serde/shape_describe_limits.rs
+++ generated/src/protocol_serde/shape_describe_limits.rs
@@ -67,12 +67,6 @@
     })
 }

-pub fn ser_describe_limits_input(
-    _input: &super::super::operation::describe_limits::DescribeLimitsInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
-    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
-}
-
 pub(crate) fn de_describe_limits(
     _value: &[u8],
     mut builder: super::super::operation::describe_limits::builders::DescribeLimitsOutputBuilder,
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

### `src/protocol_serde/shape_put_resource_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_resource_policy_input.rs
+++ generated/src/protocol_serde/shape_put_resource_policy_input.rs
@@ -12,8 +12,5 @@
     if let Some(var_3) = &input.expected_revision_id {
         object.key("ExpectedRevisionId").string(var_3.as_str());
     }
-    if let Some(var_4) = &input.confirm_remove_self_resource_access {
-        object.key("ConfirmRemoveSelfResourceAccess").boolean(*var_4);
-    }
     Ok(())
 }
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

### `src/types/error/_replicated_write_conflict_exception.rs`

```diff
--- reference/src/types/error/_replicated_write_conflict_exception.rs
+++ generated/src/types/error/_replicated_write_conflict_exception.rs
@@ -11,7 +11,7 @@
 impl ReplicatedWriteConflictException {
     /// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
     pub fn retryable_error_kind(&self) -> ::aws_smithy_types::retry::ErrorKind {
-        ::aws_smithy_types::retry::ErrorKind::ClientError
+        ::aws_smithy_types::retry::ErrorKind::ServerError
     }
     /// Returns the error message.
     pub fn message(&self) -> ::std::option::Option<&str> {
```
