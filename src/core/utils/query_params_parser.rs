/// Converts query parameters into a MongoDB filter document.
///
/// This function supports various types of filters, including equality, comparison operators,
/// regex matching, and nested fields. Additionally, it handles the `$set` operator for update
/// operations.
///
/// # `$set` Operator
///
/// The `$set` operator is expected to follow the format:
/// `?set=field1:value1,field2:value2`
///
/// This will generate a MongoDB document like:
/// ```json
/// {
///   "$set": {
///     "field1": value1,
///     "field2": value2
///   }
/// }
/// ```
///
/// This format aligns with MongoDB's update operation syntax.
///
/// # Examples
///
/// - `/items?name=John`
///   - Generates: `{ "name": "John" }`
///
/// - `/items?age.gt=25`
///   - Generates: `{ "age": { "$gt": 25 } }`
///
/// - `/items?name.regex=smith`
///   - Generates: `{ "name": { "$regex": "smith", "$options": "i" } }`
///
/// - `/items?name.regex=^A`
///   - Generates: `{ "name": { "$regex": "^A", "$options": "i" } }`
///
/// - `/items?number.regex=^123`
///   - Generates: `{ "number": { "$regex": "^123", "$options": "i" } }`
///
/// - `/items?status=active&price.lt=100`
///   - Generates: `{ "status": "active", "price": { "$lt": 100 } }`
///
/// - `/items?set=field1:value1,field2:value2`
///   - Generates: `{ "$set": { "field1": value1, "field2": value2 } }`
///
/// - `/items?name=John%20Doe`
///   - Generates: `{ "name": "John Doe" }`
///
/// # New: Nested Field Support
///
/// - `/items?address.street=Main`
///   - Generates: `{ "address": { "street": "Main" } }`
///
/// - `/items?address.city=NYC`
///   - Generates: `{ "address": { "city": "NYC" } }`
///
/// - `/items?metadata.tags.in=rust,programming`
///   - Generates: `{ "metadata": { "tags": { "$in": ["rust", "programming"] } } }`
///
/// - `/items?address.location.lat.gt=40.0`
///   - Generates: `{ "address": { "location": { "lat": { "$gt": 40.0 } } } }`
///
/// # `sort` Operator
///
/// The `sort` operator is expected to follow the format:
/// `?sort=field1:1,field2:-1`
///
/// This will generate a MongoDB document like:
/// ```json
/// {
///   "$sort": {
///     "field1": 1,
///     "field2": -1
///   }
/// }
/// ```
///
/// This format aligns with MongoDB's sort operation syntax.
///
/// # Examples
///
/// - `/items?sort=name:1`
///   - Generates: `{ "$sort": { "name": 1 } }`
///
/// - `/items?sort=age:-1,name:1`
///   - Generates: `{ "$sort": { "age": -1, "name": 1 } }`
///
/// # Notes
///
/// - Supported comparison operators: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `regex`.
/// - Handles nested fields using dot notation (e.g., `address.city`).
/// - Automatically parses values into appropriate BSON types (e.g., integers, floats, booleans, ObjectId, DateTime).
use std::collections::HashMap;

use bson::{doc, Bson, Document};

/// Helper function to insert a value into a nested document path
fn insert_nested_document(doc: &mut Document, path: &str, value: Bson) {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.len() == 1 {
        // Base case: insert at current level
        doc.insert(parts[0], value);
    } else {
        // Recursive case: navigate/create nested documents
        let current_key = parts[0];
        let remaining_path = parts[1..].join(".");

        // Check if we already have a document at this key
        if let Some(existing) = doc.get_mut(current_key) {
            if let Bson::Document(existing_doc) = existing {
                // Recurse into existing document
                insert_nested_document(existing_doc, &remaining_path, value);
            } else {
                // Overwrite with new document if not a document
                let mut new_doc = Document::new();
                insert_nested_document(&mut new_doc, &remaining_path, value);
                doc.insert(current_key, Bson::Document(new_doc));
            }
        } else {
            // Create new nested document
            let mut new_doc = Document::new();
            insert_nested_document(&mut new_doc, &remaining_path, value);
            doc.insert(current_key, Bson::Document(new_doc));
        }
    }
}

/// Converts query parameters into a MongoDB filter document using dot notation for nested fields.
pub fn query_to_document(
    query: HashMap<String, String>,
) -> (Document, Option<Document>, Option<Document>) {
    let mut filter = Document::new();
    let mut sort_doc = None;
    let mut project_doc = None;

    // Check if there's a $set operator in the query
    if let Some(set_value) = query.get("set") {
        let set_doc = parse_set_document(set_value);
        filter.insert("$set", set_doc);
    }

    // check if there is a $project operator in the query
    if let Some(project_value) = query.get("project") {
        project_doc = Some(parse_project_document(project_value));
    }

    // Handle sort separately from the filter
    if let Some(sort_value) = query.get("sort") {
        sort_doc = Some(parse_sort_document(sort_value));
    }

    // Check for $or operator - collect all fields that start with "or."
    let mut or_conditions: Vec<Document> = Vec::new();
    let mut regular_conditions: HashMap<String, String> = HashMap::new();

    // Separate or conditions from regular conditions
    for (key, value) in query
        .iter()
        .filter(|(k, _)| !["page", "limit", "sort", "set", "project"].contains(&k.as_str()))
    {
        if key.starts_with("or.") {
            // This is an or condition
            let condition_key = key.trim_start_matches("or.");
            let condition_doc = create_single_condition_document(condition_key, value);
            or_conditions.push(condition_doc);
        } else {
            // Regular condition
            regular_conditions.insert(key.clone(), value.clone());
        }
    }

    // If we have or conditions, create $or document
    if !or_conditions.is_empty() {
        if regular_conditions.is_empty() {
            // Only or conditions - use $or directly
            filter.insert(
                "$or",
                Bson::Array(or_conditions.into_iter().map(Bson::Document).collect()),
            );
        } else {
            // Combine regular conditions with or conditions using $and
            let mut and_conditions = Vec::new();

            // Add regular conditions as a single document
            let regular_doc = convert_conditions_to_document(regular_conditions);
            if !regular_doc.is_empty() {
                and_conditions.push(Bson::Document(regular_doc));
            }

            // Add or conditions
            if !or_conditions.is_empty() {
                let or_doc = doc! {
                    "$or": or_conditions
                };
                and_conditions.push(Bson::Document(or_doc));
            }

            if !and_conditions.is_empty() {
                filter.insert("$and", Bson::Array(and_conditions));
            }
        }
    } else {
        // No or conditions, process regular conditions as before
        for (key, value) in regular_conditions {
            // [Rest of the existing logic for processing regular conditions]
            // This is the same as your existing code, just using the extracted map
            if key.contains('.') {
                let parts: Vec<&str> = key.split('.').collect();

                // Check if last part is an operator (gt, lt, regex, etc.)
                let last_part = parts.last().unwrap();
                let is_operator = matches!(
                    *last_part,
                    "eq" | "ne" | "gt" | "gte" | "lt" | "lte" | "regex" | "in"
                );

                if is_operator && parts.len() >= 2 {
                    // Handle operator on nested field (e.g., address.location.lat.gt)
                    let field_parts = &parts[..parts.len() - 1];
                    let field_path = field_parts.join("."); // Use dot notation for MongoDB
                    let operator = *last_part;

                    match operator {
                        "eq" => {
                            // Simple equality with dot notation
                            filter.insert(&field_path, parse_value(&value));
                        }
                        "ne" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$ne", parse_value(&value));
                            filter.insert(&field_path, Bson::Document(op_doc));
                        }
                        "gt" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$gt", parse_value(&value));
                            filter.insert(&field_path, Bson::Document(op_doc));
                        }
                        "gte" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$gte", parse_value(&value));
                            filter.insert(&field_path, Bson::Document(op_doc));
                        }
                        "lt" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$lt", parse_value(&value));
                            filter.insert(&field_path, Bson::Document(op_doc));
                        }
                        "lte" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$lte", parse_value(&value));
                            filter.insert(&field_path, Bson::Document(op_doc));
                        }
                        "in" => {
                            let values: Vec<Bson> =
                                value.split(',').map(|v| parse_value(v.trim())).collect();
                            let mut op_doc = Document::new();
                            op_doc.insert("$in", Bson::Array(values));
                            filter.insert(&field_path, Bson::Document(op_doc));
                        }
                        "regex" => {
                            if let Bson::String(s) = parse_value(&value) {
                                if s.starts_with("NUMBER_REGEX:") {
                                    let pattern = s.trim_start_matches("NUMBER_REGEX:");
                                    let regex_doc = doc! {
                                        "$expr": {
                                            "$regexMatch": {
                                                "input": { "$toString": format!("${}", field_path) },
                                                "regex": pattern,
                                                "options": "i"
                                            }
                                        }
                                    };
                                    filter.extend(regex_doc);
                                } else {
                                    let mut regex_doc = Document::new();
                                    regex_doc.insert("$regex", s);
                                    regex_doc.insert("$options", "i");
                                    filter.insert(&field_path, Bson::Document(regex_doc));
                                }
                            }
                        }
                        _ => {
                            // fallback: treat as equality
                            filter.insert(&key, parse_value(&value));
                        }
                    }
                } else {
                    // Simple nested equality with dot notation (e.g., address.street=Main)
                    // Use the key directly with dot notation
                    filter.insert(&key, parse_value(&value));
                }
            } else {
                // Check if simple field has operator
                let parts: Vec<&str> = key.split('.').collect();
                if parts.len() == 2 {
                    let field = parts[0];
                    let operator = parts[1];

                    match operator {
                        "eq" => {
                            filter.insert(field, parse_value(&value));
                        }
                        "ne" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$ne", parse_value(&value));
                            filter.insert(field, Bson::Document(op_doc));
                        }
                        "gt" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$gt", parse_value(&value));
                            filter.insert(field, Bson::Document(op_doc));
                        }
                        "gte" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$gte", parse_value(&value));
                            filter.insert(field, Bson::Document(op_doc));
                        }
                        "lt" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$lt", parse_value(&value));
                            filter.insert(field, Bson::Document(op_doc));
                        }
                        "lte" => {
                            let mut op_doc = Document::new();
                            op_doc.insert("$lte", parse_value(&value));
                            filter.insert(field, Bson::Document(op_doc));
                        }
                        "in" => {
                            let values: Vec<Bson> =
                                value.split(',').map(|v| parse_value(v.trim())).collect();
                            let mut op_doc = Document::new();
                            op_doc.insert("$in", Bson::Array(values));
                            filter.insert(field, Bson::Document(op_doc));
                        }
                        "regex" => {
                            if let Bson::String(s) = parse_value(&value) {
                                if s.starts_with("NUMBER_REGEX:") {
                                    let pattern = s.trim_start_matches("NUMBER_REGEX:");
                                    let regex_doc = doc! {
                                        "$expr": {
                                            "$regexMatch": {
                                                "input": { "$toString": format!("${}", field) },
                                                "regex": pattern,
                                                "options": "i"
                                            }
                                        }
                                    };
                                    filter.extend(regex_doc);
                                } else {
                                    let mut regex_doc = Document::new();
                                    regex_doc.insert("$regex", s);
                                    regex_doc.insert("$options", "i");
                                    filter.insert(field, Bson::Document(regex_doc));
                                }
                            }
                        }
                        _ => {
                            // fallback: treat as equality
                            filter.insert(&key, parse_value(&value));
                        }
                    }
                } else {
                    // Simple equality filter
                    filter.insert(&key, parse_value(&value));
                }
            }
        }
    }

    (
        if filter.is_empty() {
            doc! {}
        } else {
            filter
        },
        sort_doc,
        project_doc,
    )
}

// Helper function to create a single condition document
fn create_single_condition_document(key: &str, value: &str) -> Document {
    let mut doc = Document::new();

    // This replicates the logic from your main function for a single condition
    if key.contains('.') {
        let parts: Vec<&str> = key.split('.').collect();

        // Check if last part is an operator
        let last_part = parts.last().unwrap();
        let is_operator = matches!(
            *last_part,
            "eq" | "ne" | "gt" | "gte" | "lt" | "lte" | "regex" | "in"
        );

        if is_operator && parts.len() >= 2 {
            let field_parts = &parts[..parts.len() - 1];
            let field_path = field_parts.join(".");
            let operator = *last_part;

            match operator {
                "eq" => {
                    doc.insert(&field_path, parse_value(value));
                }
                "ne" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$ne", parse_value(value));
                    doc.insert(&field_path, Bson::Document(op_doc));
                }
                "gt" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$gt", parse_value(value));
                    doc.insert(&field_path, Bson::Document(op_doc));
                }
                "gte" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$gte", parse_value(value));
                    doc.insert(&field_path, Bson::Document(op_doc));
                }
                "lt" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$lt", parse_value(value));
                    doc.insert(&field_path, Bson::Document(op_doc));
                }
                "lte" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$lte", parse_value(value));
                    doc.insert(&field_path, Bson::Document(op_doc));
                }
                "in" => {
                    let values: Vec<Bson> =
                        value.split(',').map(|v| parse_value(v.trim())).collect();
                    let mut op_doc = Document::new();
                    op_doc.insert("$in", Bson::Array(values));
                    doc.insert(&field_path, Bson::Document(op_doc));
                }
                "regex" => {
                    if let Bson::String(s) = parse_value(value) {
                        if s.starts_with("NUMBER_REGEX:") {
                            let pattern = s.trim_start_matches("NUMBER_REGEX:");
                            let regex_doc = doc! {
                                "$expr": {
                                    "$regexMatch": {
                                        "input": { "$toString": format!("${}", field_path) },
                                        "regex": pattern,
                                        "options": "i"
                                    }
                                }
                            };
                            doc.extend(regex_doc);
                        } else {
                            let mut regex_doc = Document::new();
                            regex_doc.insert("$regex", s);
                            regex_doc.insert("$options", "i");
                            doc.insert(&field_path, Bson::Document(regex_doc));
                        }
                    }
                }
                _ => {
                    doc.insert(key, parse_value(value));
                }
            }
        } else {
            doc.insert(key, parse_value(value));
        }
    } else {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() == 2 {
            let field = parts[0];
            let operator = parts[1];

            match operator {
                "eq" => {
                    doc.insert(field, parse_value(value));
                }
                "ne" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$ne", parse_value(value));
                    doc.insert(field, Bson::Document(op_doc));
                }
                "gt" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$gt", parse_value(value));
                    doc.insert(field, Bson::Document(op_doc));
                }
                "gte" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$gte", parse_value(value));
                    doc.insert(field, Bson::Document(op_doc));
                }
                "lt" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$lt", parse_value(value));
                    doc.insert(field, Bson::Document(op_doc));
                }
                "lte" => {
                    let mut op_doc = Document::new();
                    op_doc.insert("$lte", parse_value(value));
                    doc.insert(field, Bson::Document(op_doc));
                }
                "in" => {
                    let values: Vec<Bson> =
                        value.split(',').map(|v| parse_value(v.trim())).collect();
                    let mut op_doc = Document::new();
                    op_doc.insert("$in", Bson::Array(values));
                    doc.insert(field, Bson::Document(op_doc));
                }
                "regex" => {
                    if let Bson::String(s) = parse_value(value) {
                        if s.starts_with("NUMBER_REGEX:") {
                            let pattern = s.trim_start_matches("NUMBER_REGEX:");
                            let regex_doc = doc! {
                                "$expr": {
                                    "$regexMatch": {
                                        "input": { "$toString": format!("${}", field) },
                                        "regex": pattern,
                                        "options": "i"
                                    }
                                }
                            };
                            doc.extend(regex_doc);
                        } else {
                            let mut regex_doc = Document::new();
                            regex_doc.insert("$regex", s);
                            regex_doc.insert("$options", "i");
                            doc.insert(field, Bson::Document(regex_doc));
                        }
                    }
                }
                _ => {
                    doc.insert(key, parse_value(value));
                }
            }
        } else {
            doc.insert(key, parse_value(value));
        }
    }

    doc
}

// Helper function to convert regular conditions to document
fn convert_conditions_to_document(conditions: HashMap<String, String>) -> Document {
    let mut doc = Document::new();

    for (key, value) in conditions {
        let condition_doc = create_single_condition_document(&key, &value);
        doc.extend(condition_doc);
    }

    doc
}

// Helper function to parse $set value as a document
fn parse_set_document(value: &str) -> Document {
    let mut doc = Document::new();

    // Split the value by commas to get key-value pairs
    for pair in value.split(',') {
        // Split each pair by colon to get key and value
        let parts: Vec<&str> = pair.split(':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();

            // Parse the value and insert it into the document
            insert_nested_document(&mut doc, key, parse_value(value));
        }
    }

    doc
}

fn parse_project_document(value: &str) -> Document {
    let mut project_fields = Document::new();

    // Split the value by commas to get field:include pairs
    for pair in value.split(',') {
        let parts: Vec<&str> = pair.split(':').collect();
        if parts.len() == 2 {
            let field = parts[0].trim();
            let include = match parts[1].trim() {
                "1" => 1,
                "0" => 0,
                _ => 1, // default to include
            };
            project_fields.insert(field, include);
        }
    }

    doc! {"$project": project_fields}
}

// Add new helper function to parse sort parameters
fn parse_sort_document(value: &str) -> Document {
    let mut sort_fields = Document::new();

    // Split the value by commas to get field:direction pairs
    for pair in value.split(',') {
        // Split each pair by colon to get field and direction
        let parts: Vec<&str> = pair.split(':').collect();
        if parts.len() == 2 {
            let field = parts[0].trim();
            let direction = match parts[1].trim() {
                "1" | "asc" => 1,
                "-1" | "desc" => -1,
                _ => 1, // default to ascending
            };
            sort_fields.insert(field, direction);
        }
    }

    // Wrap the sort fields in a $sort operator
    doc! { "$sort": sort_fields }
}

// Helper function to parse string values to appropriate BSON types
fn parse_value(value: &str) -> Bson {
    // Support explicit type annotation: value~type
    // e.g., "123~int", "true~bool", "2024-06-01T12:00:00Z~datetime"
    // You can change the delimiter if needed
    let (val, ty) = if let Some(idx) = value.rfind('~') {
        let (v, t) = value.split_at(idx);
        (v, Some(&t[1..]))
    } else {
        (value, None)
    };

    match ty {
        Some("string") => Bson::String(val.to_string()),
        Some("int") | Some("i32") => val
            .parse::<i32>()
            .map(Bson::Int32)
            .unwrap_or(Bson::String(val.to_string())),
        Some("long") | Some("i64") => val
            .parse::<i64>()
            .map(Bson::Int64)
            .unwrap_or(Bson::String(val.to_string())),
        Some("double") | Some("float") | Some("f64") => val
            .parse::<f64>()
            .map(Bson::Double)
            .unwrap_or(Bson::String(val.to_string())),
        Some("bool") | Some("boolean") => match val.to_lowercase().as_str() {
            "true" => Bson::Boolean(true),
            "false" => Bson::Boolean(false),
            _ => Bson::String(val.to_string()),
        },
        Some("objectid") => bson::oid::ObjectId::parse_str(val)
            .map(Bson::ObjectId)
            .unwrap_or(Bson::String(val.to_string())),
        Some("datetime") | Some("date") => {
            // Try RFC3339 first
            if let Ok(datetime) = bson::DateTime::parse_rfc3339_str(val) {
                return Bson::DateTime(datetime);
            }

            // Fallback to string
            Bson::String(val.to_string())
        }
        Some("number-regex") => {
            // Special marker value that will be handled in query construction
            Bson::String(format!("NUMBER_REGEX:{}", val))
        }
        _ => {
            // Fallback to auto-detection as before
            if let Ok(int_val) = val.parse::<i32>() {
                return Bson::Int32(int_val);
            }
            if let Ok(float_val) = val.parse::<f64>() {
                return Bson::Double(float_val);
            }
            match val.to_lowercase().as_str() {
                "true" => return Bson::Boolean(true),
                "false" => return Bson::Boolean(false),
                _ => {}
            }
            if let Ok(object_id) = bson::oid::ObjectId::parse_str(val) {
                return Bson::ObjectId(object_id);
            }
            if let Ok(datetime) = bson::DateTime::parse_rfc3339_str(val) {
                return Bson::DateTime(datetime);
            }

            Bson::String(val.to_string())
        }
    }
}
