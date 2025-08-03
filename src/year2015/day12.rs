use serde_json::Value;

pub fn parse(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

pub fn part1(value: &Value) -> i64 {
    match value {
        Value::Object(obj) => obj.values().map(part1).sum(),
        Value::Array(arr) => arr.iter().map(part1).sum(),
        Value::Number(num) => num.as_i64().unwrap(),
        Value::String(_) => 0,
        _ => unreachable!(),
    }
}

pub fn part2(_input: &Value) -> usize {
    0
}
