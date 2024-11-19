use std::collections::HashMap;

enum JsonToken {
    Object(JsonObject),
}

struct JsonObject {
    keys: HashMap<String, JsonToken>,
}

#[derive(Debug)]
struct SomeError {}

struct Parser {
    index: usize,
    chars: Vec<char>,
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
        if Some(&'{') != self.pop() {
            return Err(SomeError {});
        }

        return Ok(JsonObject {
            keys: HashMap::new(),
        });
    }

    fn parse_string(&mut self) -> Result<String, SomeError> {
        todo!()
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.index)
    }

    fn is_done(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn pop(&mut self) -> Option<&char> {
        self.index += 1;
        self.chars.get(self.index)
    }
}

fn parse_json(input: &str) -> Result<JsonObject, SomeError> {
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
        assert!(result.is_ok())
    }
}
