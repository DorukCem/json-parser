use ordered_float::OrderedFloat;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum JsonNumber {
    Integer(i64),
    Float(OrderedFloat<f64>),
}

#[derive(Debug, PartialEq, Eq)]
enum JsonToken {
    Object(JsonObject),
    Array(JsonArray),
    String(String),
    Number(JsonNumber),
    True,
    False,
    Null,
}
#[derive(Debug, PartialEq, Eq)]
struct JsonArray {
    items: Option<Vec<JsonToken>>,
}

#[derive(Debug, PartialEq, Eq)]
struct JsonObject {
    keys: Option<HashMap<String, JsonToken>>,
}

#[derive(Debug)]
struct SomeError {
    msg: String,
}

struct Parser {
    index: usize,
    chars: Vec<char>,
}

enum ParseState {
    Key,
    Value,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            index: 0,
            chars: input.chars().collect(),
        }
    }

    fn parse_json(&mut self) -> Result<JsonObject, SomeError> {
        let object = self.parse_object();

        return object;
    }

    fn parse_object(&mut self) -> Result<JsonObject, SomeError> {
        let first = self.pop();
        if Some(&'{') != first {
            return Err(SomeError {
                msg: format!("Expected object to start with: {{ instead got: {:?}  ", {
                    first.unwrap()
                }),
            });
        }
        self.skip_whitespace();

        let mut parse_state = ParseState::Key;
        let mut current_key = None;
        let mut keys: HashMap<String, JsonToken> = HashMap::new();
        let mut expect_one_more_kv_pair = false;

        // Read key : value pairs
        while !self.is_done() {
            self.skip_whitespace();
            match (
                &parse_state,
                *self
                    .peek()
                    .expect("I do not expect this to panic MATCH PARSE STATE"),
            ) {
                (ParseState::Key, '}') => break,

                (ParseState::Key, ':') => {
                    self.pop();
                    parse_state = ParseState::Value;
                }

                (ParseState::Key, '\"') => {
                    if current_key.is_some() {
                        return Err(SomeError {
                            msg: format!("Expected  :  after key got  \"  index: {}", self.index),
                        });
                    }
                    current_key = Some(self.parse_string()?);
                }

                (ParseState::Value, '}') => {
                    if expect_one_more_kv_pair {
                        return Err(SomeError {
                            msg: "Expexted one more value, got  }  instead".to_string(),
                        });
                    }
                    break;
                }
                (ParseState::Value, ',') => {
                    parse_state = ParseState::Key;
                    expect_one_more_kv_pair = true;
                    self.pop();
                }

                (ParseState::Value, _value) => {
                    let value = self.parse_json_value()?;
                    let key = current_key.take().unwrap();
                    keys.insert(key, value);
                    expect_one_more_kv_pair = false;
                }

                (_, unexpected) => {
                    return Err(SomeError {
                        msg: format!("Unexpected token {}  index: {}", unexpected, self.index),
                    })
                }
            }
        }

        if expect_one_more_kv_pair {
            return Err(SomeError {
                msg: "Expected one more key-value pair after comma".to_string(),
            });
        }

        self.pop();

        return Ok(JsonObject {
            keys: if keys.len() > 0 { Some(keys) } else { None },
        });
    }

    fn parse_string(&mut self) -> Result<String, SomeError> {
        let mut s = String::new();

        if Some(&'\"') != self.pop() {
            return Err(SomeError {
                msg: "Expected string to start with  \"  ".to_string(),
            });
        }

        while let Some(c) = self.pop() {
            if c == &'\"' {
                break;
            }
            s.push(*c);
        }

        if self.is_done() {
            return Err(SomeError {
                msg: "Unterminated string".to_string(),
            });
        }

        return Ok(s);
    }

    fn parse_array(&mut self) -> Result<JsonArray, SomeError> {
        todo!()
    }

    fn parse_number(&mut self) -> Result<JsonNumber, SomeError> {
        let mut number = String::new();
        while let Some(c) = self.peek() {
            match *c {
                '0'..'9' | '.' | '-' | 'E' | 'e' => {
                    number.push(*c);
                    self.pop();
                }
                _ => break,
            }
        }

        // Attempt to parse as an integer
        if let Ok(integer) = number.parse::<i64>() {
            return Ok(JsonNumber::Integer(integer));
        }

        // If that fails, attempt to parse as a float
        if let Ok(float) = number.parse::<f64>() {
            return Ok(JsonNumber::Float(OrderedFloat(float)));
        }

        Err(SomeError {
            msg: format!("Invalid Json number: {}", number),
        })
    }

    fn parse_expected_word(&mut self, word: &str) -> Result<(), SomeError> {
        for expected in word.chars() {
            if let Some(ch) = self.pop() {
                if expected != *ch {
                    return Err(SomeError {
                        msg: format!(
                            "Expected token {} for word {} found token {}",
                            expected, word, ch
                        ),
                    });
                }
            } else {
                return Err(SomeError {
                    msg: "Json mssing terminator".to_owned(),
                });
            }
        }
        Ok(())
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.index)
    }

    fn is_done(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn pop(&mut self) -> Option<&char> {
        let val = self.chars.get(self.index);
        self.index += 1;
        val
    }

    fn skip_whitespace(&mut self) -> bool {
        while !self.is_done() && (self.peek() == Some(&' ') || self.peek() == Some(&'\n')) {
            self.pop();
        }
        // Some error occured
        if self.is_done() {
            return false;
        }
        true
    }

    fn parse_json_value(&mut self) -> Result<JsonToken, SomeError> {
        match *self.peek().unwrap() {
            '\"' => Ok(JsonToken::String(self.parse_string()?)),
            '{' => Ok(JsonToken::Object(self.parse_object()?)),

            '[' => Ok(JsonToken::Array(self.parse_array()?)),

            '-' | '0'..='9' => Ok(JsonToken::Number(self.parse_number()?)),

            token @ ('t' | 'f' | 'n') => match token {
                't' => {
                    self.parse_expected_word("true")?;
                    Ok(JsonToken::True)
                }
                'f' => {
                    self.parse_expected_word("false")?;
                    Ok(JsonToken::False)
                }
                'n' => {
                    self.parse_expected_word("null")?;
                    Ok(JsonToken::Null)
                }
                _ => unreachable!(),
            },

            unexpected => {
                return Err(SomeError {
                    msg: format!("Unexpected token {}  index: {}", unexpected, self.index),
                })
            }
        }
    }
}

fn parse_json(input: &str) -> Result<JsonObject, SomeError> {
    if input == "" {
        return Err(SomeError {
            msg: "No json object".to_string(),
        });
    }
    return Parser::new(input).parse_json();
}

fn main() {
    let example = "";
    parse_json(example).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let result = parse_json("");
        assert!(result.is_err())
    }

    #[test]
    fn empty_object() {
        let result = parse_json("{}");
        assert!(result.unwrap() == JsonObject { keys: None })
    }

    #[test]
    fn kv_after_comma() {
        let result = parse_json(r#"{"key": "value",}"#);
        assert!(result.is_err())
    }

    #[test]
    fn non_double_quoted_property() {
        // key2 should have double quotes around it here
        let result = parse_json(
            r#"{
  "key": "value",
  key2: "value"
}"#,
        );
        assert!(result.is_err())
    }

    #[test]
    fn simple_key_value() {
        let result = parse_json(r#"{"key": "value"}"#).unwrap();
        let keys: HashMap<String, JsonToken> =
            HashMap::from([("key".to_string(), JsonToken::String("value".to_string()))]);

        let compare = JsonObject { keys: Some(keys) };

        assert_eq!(result, compare)
    }

    #[test]
    fn simple_key_value_with_comma() {
        let result = parse_json(
            r#"{
  "key": "value",
  "key2": "value"
}"#,
        )
        .unwrap();
        let keys: HashMap<String, JsonToken> = HashMap::from([
            ("key".to_string(), JsonToken::String("value".to_string())),
            ("key2".to_string(), JsonToken::String("value".to_string())),
        ]);

        let compare = JsonObject { keys: Some(keys) };

        assert_eq!(result, compare)
    }

    #[test]
    fn non_valid_boolean() {
        let result = parse_json(
            r#"{
  "key1": true,
  "key2": False,
  "key3": null,
  "key4": "value",
  "key5": 101
}"#,
        );

        assert!(result.is_err())
    }

    #[test]
    fn valid_bool_and_number() {
        let result = parse_json(
            r#"{
  "key1": true,
  "key2": false,
  "key3": null,
  "key4": "value",
  "key5": 101
}"#,
        )
        .unwrap();

        let keys: HashMap<String, JsonToken> = HashMap::from([
            ("key1".to_string(), JsonToken::True),
            ("key2".to_string(), JsonToken::False),
            ("key3".to_string(), JsonToken::Null),
            ("key4".to_string(), JsonToken::String("value".to_string())),
            (
                "key5".to_string(),
                JsonToken::Number(JsonNumber::Integer(101)),
            ),
        ]);

        let compare = JsonObject { keys: Some(keys) };

        assert_eq!(result, compare)
    }
}
