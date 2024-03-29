use serde_json::Value;

use crate::schema::{BooleanType, DictType, LiteralType, NumberType, StringType, ListType, DataType};

pub trait Validator {
    fn validate_type(&self, node: &Value) -> bool;
    fn validate_meta(&self, node: &Value) -> bool;
    fn validate(&self, node: &Value) -> bool {
        self.validate_type(&node) && self.validate_meta(&node)
    }
}

impl Validator for DataType {
    fn validate_type(&self, node: &Value) -> bool {
        // todo nullable and optional
        match self {
            DataType::Number(inner) => inner.validate_type(&node),
            DataType::Dict(inner) => { inner.validate_type(&node) }
            DataType::List(inner) => { inner.validate_type(&node) }
            DataType::String(inner) => { inner.validate_type(&node) }
            DataType::Literal(inner) => { inner.validate_type(&node) }
            DataType::Boolean(inner) => { inner.validate_type(&node) }
        }
    }

    fn validate_meta(&self, node: &Value) -> bool {
        match self {
            DataType::Number(inner) => inner.validate_meta(&node),
            DataType::Dict(inner) => { inner.validate_meta(&node) }
            DataType::List(inner) => { inner.validate_meta(&node) }
            DataType::String(inner) => { inner.validate_meta(&node) }
            DataType::Literal(inner) => { inner.validate_meta(&node) }
            DataType::Boolean(inner) => { inner.validate_meta(&node) }
        }
    }
}

impl Validator for DictType {
    fn validate_type(&self, node: &Value) -> bool {
        matches!(node, Value::Object(..))
    }

    fn validate_meta(&self, node: &Value) -> bool {
        let object = match node {
            Value::Object(inner) => inner,
            _ => unreachable!()
        };
        
        for (key, value) in object.iter() {
            let contains_normal_field = self.fields.contains_key(key);
            if contains_normal_field {
                // todo DateType::Validate
            } else {
                return false;
            }
        };
        true
    }
}

impl Validator for ListType {
    fn validate_type(&self, node: &Value) -> bool {
        matches!(node, Value::Array(..))
    }

    fn validate_meta(&self, node: &Value) -> bool {
        let array = match node {
            Value::Array(inner) => inner,
            _ => unreachable!()
        };

        if let Some(limit) = self.limit {
            if array.len() as u64 > limit {
                return false;
            }
        }
        for item in array {
            if !self.element_type.validate(item) {
                return false;
            }
        }
        true
    }
}

impl Validator for LiteralType {
    fn validate_type(&self, node: &Value) -> bool {
        matches!(node, Value::String(..))
    }

    fn validate_meta(&self, node: &Value) -> bool {
        let inner = match node {
            Value::String(inner) => inner,
            _ => unreachable!()
        };
        self.candidate.contains(inner)
    }
}

impl Validator for StringType {
    fn validate_type(&self, node: &Value) -> bool {
        matches!(node, Value::String(..))
    }

    fn validate_meta(&self, node: &Value) -> bool {
        let inner = match node {
            Value::String(inner) => inner,
            _ => unreachable!()
        };
        if let Some(limit) = &self.length {
            if inner.len() as u64 > *limit { return false; }
        }
        if let Some(reg) = &self.regex {
            let result = regex::Regex::new(&format!("^{}$", reg)).unwrap();
            if !result.is_match(inner) {
                return false;
            }
        }
        true
    }
}

impl Validator for NumberType {
    fn validate_type(&self, node: &Value) -> bool {
        matches!(node, Value::Number(..))
    }

    fn validate_meta(&self, node: &Value) -> bool {
        true
    }
}


impl Validator for BooleanType {
    fn validate_type(&self, node: &Value) -> bool {
        matches!(node, Value::Bool(..))
    }

    fn validate_meta(&self, node: &Value) -> bool {
        self.validate_type(&node)
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{Number, Value};
    use serde_json::json;

    use crate::schema::{BooleanType, DataType, DictType, LiteralType, NumberType, StringType, ListType};
    use crate::validator::Validator;

    fn basic_validate(validator: &dyn Validator, content: impl Into<String>) -> bool {
        let node: Value = serde_json::from_str(content.into().as_str()).unwrap();
        validator.validate(&node)
    }

    #[test]
    fn test_bool_type() {
        let validator = BooleanType { optional: false, nullable: false };
        assert_eq!(true, validator.validate_type(&Value::Bool(true)));
        assert_eq!(true, validator.validate_type(&Value::Bool(false)));
        assert_eq!(false, validator.validate_type(&Value::Null));
        assert_eq!(false, validator.validate_type(&Value::String("it".to_owned())));
        assert_eq!(false, validator.validate_type(&json!([])));
        assert_eq!(false, validator.validate_type(&Value::Number(Number::from(1i8))));
        assert_eq!(false, validator.validate_type(&json!({ "an": "object" })));
    }

    #[test]
    fn test_dict_type() {
        let validator = DictType {
            optional: false,
            nullable: false,
            fields: Default::default(),
            any_fields: None,
            others: None,
        };
        assert_eq!(false, validator.validate_type(&Value::Bool(true)));
        assert_eq!(false, validator.validate_type(&Value::Bool(false)));
        assert_eq!(false, validator.validate_type(&Value::Null));
        assert_eq!(false, validator.validate_type(&Value::String("it".to_owned())));
        assert_eq!(false, validator.validate_type(&json!([])));
        assert_eq!(false, validator.validate_type(&Value::Number(Number::from(1i8))));
        assert_eq!(true, validator.validate_type(&json!({ "an": "object" })));
    }

    #[test]
    fn test_literal_type() {
        let validator = LiteralType {
            optional: false,
            nullable: false,
            candidate: vec![],
        };
        assert_eq!(false, validator.validate_type(&Value::Bool(true)));
        assert_eq!(false, validator.validate_type(&Value::Bool(false)));
        assert_eq!(false, validator.validate_type(&Value::Null));
        assert_eq!(true, validator.validate_type(&Value::String("it".to_owned())));
        assert_eq!(false, validator.validate_type(&json!([])));
        assert_eq!(false, validator.validate_type(&Value::Number(Number::from(1i8))));
        assert_eq!(false, validator.validate_type(&json!({ "an": "object" })));
    }

    #[test]
    fn test_string_type() {
        let validator = StringType {
            optional: false,
            nullable: false,
            length: None,
            regex: None,
        };
        assert_eq!(false, validator.validate_type(&Value::Bool(true)));
        assert_eq!(false, validator.validate_type(&Value::Bool(false)));
        assert_eq!(false, validator.validate_type(&Value::Null));
        assert_eq!(true, validator.validate_type(&Value::String("it".to_owned())));
        assert_eq!(false, validator.validate_type(&json!([])));
        assert_eq!(false, validator.validate_type(&Value::Number(Number::from(1i8))));
        assert_eq!(false, validator.validate_type(&json!({ "an": "object" })));
    }

    #[test]
    fn test_number_type() {
        let validator = NumberType {
            optional: false,
            nullable: false,
        };
        assert_eq!(false, validator.validate_type(&Value::Bool(true)));
        assert_eq!(false, validator.validate_type(&Value::Bool(false)));
        assert_eq!(false, validator.validate_type(&Value::Null));
        assert_eq!(false, validator.validate_type(&Value::String("it".to_owned())));
        assert_eq!(false, validator.validate_type(&json!([])));
        assert_eq!(true, validator.validate_type(&Value::Number(Number::from(1i8))));
        assert_eq!(false, validator.validate_type(&json!({ "an": "object" })));
    }

    #[test]
    fn test_list_type() {
        let validator = ListType {
            optional: false,
            nullable: false,
            element_type: DataType::Boolean(Box::new(BooleanType { optional: false, nullable: false })),
            limit: None,
        };
        assert_eq!(false, validator.validate_type(&Value::Bool(true)));
        assert_eq!(false, validator.validate_type(&Value::Bool(false)));
        assert_eq!(false, validator.validate_type(&Value::Null));
        assert_eq!(false, validator.validate_type(&Value::String("it".to_owned())));
        assert_eq!(true, validator.validate_type(&json!([])));
        assert_eq!(false, validator.validate_type(&Value::Number(Number::from(1i8))));
        assert_eq!(false, validator.validate_type(&json!({ "an": "object" })));
    }

    #[test]
    fn dict_type_should_have_one_field() {
        let mut map = HashMap::new();
        map.insert("a".to_owned(), DataType::Boolean(Box::new(BooleanType { optional: false, nullable: false })));
        let validator = DictType {
            optional: false,
            nullable: false,
            fields: map,
            any_fields: None,
            others: None,
        };

        assert_eq!(true, basic_validate(&validator, r#" {"a": true} "#));
        assert_eq!(false, basic_validate(&validator, r#" {"b": true} "#));
    }

    #[test]
    fn literal_type_should_be_in_candidate() {
        let validator = LiteralType {
            optional: false,
            nullable: false,
            candidate: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        };

        assert_eq!(true, validator.validate(&Value::String("a".to_owned())));
        assert_eq!(true, validator.validate(&Value::String("b".to_owned())));
        assert_eq!(true, validator.validate(&Value::String("c".to_owned())));
        assert_eq!(false, validator.validate(&Value::String("d".to_owned())));
    }

    #[test]
    fn string_type_should_limit_with_length() {
        let string_type = StringType {
            optional: false,
            nullable: false,
            length: Some(10),
            regex: None,
        };
        assert_eq!(true, string_type.validate(&Value::String("1".to_owned())));
        assert_eq!(true, string_type.validate(&Value::String("".to_owned())));
        assert_eq!(true, string_type.validate(&Value::String("1234567890".to_owned())));
        assert_eq!(true, string_type.validate(&Value::String("emoji👍".to_owned())));
        assert_eq!(true, string_type.validate(&Value::String("utf8中文".to_owned())));
        assert_eq!(false, string_type.validate(&Value::String("12345678901".to_owned())));
    }

    #[test]
    fn string_type_should_match_by_regex() {
        let string_type = StringType {
            optional: false,
            nullable: false,
            length: None,
            regex: Some("[0-9]+".to_owned()),
        };
        assert_eq!(true, string_type.validate(&Value::String("1".to_owned())));
        assert_eq!(false, string_type.validate(&Value::String("".to_owned())));
        assert_eq!(true, string_type.validate(&Value::String("1234567890".to_owned())));
        assert_eq!(false, string_type.validate(&Value::String("emoji👍123".to_owned())));
        assert_eq!(false, string_type.validate(&Value::String("utf8中文".to_owned())));
        assert_eq!(true, string_type.validate(&Value::String("12345678901".to_owned())));
    }

    #[test]
    fn list_type_should_validate_element_type() {
        let validator = ListType {
            optional: false,
            nullable: false,
            element_type: DataType::Boolean(Box::new(BooleanType { optional: false, nullable: false })),
            limit: None,
        };
        assert_eq!(true, validator.validate(&json!([true])));
        assert_eq!(true, validator.validate(&json!([true, true])));
        assert_eq!(true, validator.validate(&json!([true, false])));
        assert_eq!(false, validator.validate(&json!([true, false, 1])));
        assert_eq!(false, validator.validate(&json!([true, false, "123"])));
        assert_eq!(false, validator.validate(&json!([true, false, null])));
        assert_eq!(false, validator.validate(&json!([{}])));
    }

    #[test]
    fn list_type_should_limit_by_length() {
        let validator = ListType {
            optional: false,
            nullable: false,
            element_type: DataType::Boolean(Box::new(BooleanType { optional: false, nullable: false })),
            limit: Some(3),
        };
        assert_eq!(true, validator.validate(&json!([true, true, true])));
        assert_eq!(false, validator.validate(&json!([true, true, true, true])));
    }
}
