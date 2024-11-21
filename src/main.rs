use std::{collections::HashMap, fmt::format};

#[derive(Debug, PartialEq, Eq)]
enum JsonToken {
    Object(JsonObject),
    String(String),
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
                msg: format!("Expected object to start with: {{ instead got: {:?}", {
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
            match (&parse_state, self.peek()) {
                (ParseState::Key, Some(&'}')) => break,
                (ParseState::Key, Some(&'\"')) => {
                    if current_key.is_some() {
                        return Err(SomeError {
                            msg: format!("Expected  :  after key got  \"  "),
                        });
                    }
                    current_key = Some(self.parse_string()?);
                }

                (ParseState::Key, Some(&':')) => {
                    self.pop();
                    parse_state = ParseState::Value;
                }

                (ParseState::Value, Some(&'"')) => {
                    let value = self.parse_string()?;
                    let key = current_key.take().unwrap();
                    keys.insert(key, JsonToken::String(value));
                }

                (ParseState::Value, Some(&',')) => {
                    expect_one_more_kv_pair = true;
                    self.pop();
                }
                (_, Some(unexpected)) => {
                    return Err(SomeError {
                        msg: format!("Unexpected token {}", unexpected),
                    })
                }
                (_, None) => unreachable!(),
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
            if c == &'"' {
                self.pop();
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
        let keys: HashMap<String, JsonToken> =
            HashMap::from([("key".to_string(), JsonToken::String("value".to_string())), ("key2".to_string(), JsonToken::String("value".to_string()))]);

        let compare = JsonObject { keys: Some(keys) };

        assert_eq!(result, compare)
    }
}
