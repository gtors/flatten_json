extern crate serde_json;

use serde_json::{Error, Map, Value};

static DEFAULT_FLATTENER: &Flattener = &Flattener {
    flat_array: false,
    flat_key_cb: |vec| vec.join("."),
};

type FlatMap = Map<String, Value>;

type Keys<'a> = Vec<&'a str>;

#[derive(Clone)]
pub struct Flattener {
    pub flat_array: bool,
    pub flat_key_cb: fn(&Vec<&str>) -> String,
}

impl Default for Flattener {
    fn default() -> Self {
        Self::new()
    }
}

impl Flattener {
    pub fn new() -> Flattener {
        DEFAULT_FLATTENER.clone()
    }

    pub fn flatten_from_str(&self, json: &str) -> Result<String, Error> {
        let value: Value = serde_json::from_str(json)?;
        self.flatten_from_value(&value)
    }

    pub fn flatten_from_value(&self, value: &Value) -> Result<String, Error> {
        let flat_value = self.flatten_value(value, None);
        serde_json::to_string(&flat_value)
    }

    fn flatten_value(&self, value: &Value, maybe_keys: Option<Keys>) -> Value {
        match value {
            &Value::Object(ref map) => self.flatten_object(map, maybe_keys),
            &Value::Array(ref arr) => self.flatten_array(arr, maybe_keys),
            v_scalar @ _ => v_scalar.clone(),
        }
    }

    fn flatten_object(&self, obj: &Map<String, Value>, maybe_keys: Option<Keys>) -> Value {
        let mut keys = maybe_keys.unwrap_or_else(|| vec![]);
        let mut flat_map = FlatMap::new();

        for (k, v) in obj {
            keys.push(k);
            let flat_key = (self.flat_key_cb)(&keys);
            let flat_val = self.flatten_value(v, Some(keys.clone()));

            match flat_val {
                Value::Object(map) => for (k, v) in map {
                    flat_map.insert(k, v);
                },
                _ => {
                    flat_map.insert(flat_key, flat_val);
                }
            }

            keys.pop();
        }

        Value::Object(flat_map)
    }

    fn flatten_array(&self, arr: &[Value], maybe_keys: Option<Keys>) -> Value {
        if self.flat_array {
            let keys: Keys = maybe_keys.unwrap_or_else(|| vec![]);
            let mut flat_map = FlatMap::new();

            for (idx, val) in arr.iter().enumerate() {
                let str_idx = idx.to_string();
                let mut _keys = keys.clone();
                _keys.push(&*str_idx);
                let flat_key = (self.flat_key_cb)(&_keys);
                let flat_val = self.flatten_value(val, Some(_keys.clone()));

                match flat_val {
                    Value::Object(map) => for (k, v) in map {
                        flat_map.insert(k, v);
                    },
                    _ => {
                        flat_map.insert(flat_key, flat_val);
                    }
                }

                _keys.pop();
            }

            Value::Object(flat_map)
        } else {
            let mut flat_values: Vec<Value> = Vec::with_capacity(arr.len());
            for val in arr {
                flat_values.push(self.flatten_value(val, None));
            }

            Value::Array(flat_values)
        }
    }
}

pub fn flatten_from_str(json: &str) -> Result<String, Error> {
    DEFAULT_FLATTENER.flatten_from_str(json)
}

pub fn flatten_from_value(value: &Value) -> Result<String, Error> {
    DEFAULT_FLATTENER.flatten_from_value(value)
}

#[cfg(test)]
mod tests {
    use Flattener;
    use flatten_from_str;

    #[test]
    fn test_general_case() {
        let json = r#"[{"a": "a", "b": {"c": { "d": 1 }}}]"#; 
        let expected = r#"[{"a":"a","b.c.d":1}]"#;
        assert_eq!(flatten_from_str(json).unwrap(), expected);
    }

    #[test]
    fn test_flat_array() {
        let f = Flattener { flat_array: true , ..Flattener::default() };
        let json = r#"[{"a": "a", "b": {"c": { "d": 1 }}}]"#; 
        let expected = r#"{"0.a":"a","0.b.c.d":1}"#;
        assert_eq!(f.flatten_from_str(json).unwrap(), expected);
    }

    #[test]
    fn test_custom_key() {
        let f = Flattener { flat_key_cb: |vec| vec.join("_"), ..Flattener::default() };
        let json = r#"[{"a": "a", "b": {"c": { "d": 1 }}}]"#; 
        let expected = r#"[{"a":"a","b_c_d":1}]"#;
        assert_eq!(f.flatten_from_str(json).unwrap(), expected);
    }
}
