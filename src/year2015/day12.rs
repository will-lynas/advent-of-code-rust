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
        Value::Null | Value::Bool(_) => unreachable!(),
    }
}

pub fn solve2(value: &Value) -> Option<i64> {
    match value {
        Value::Object(obj) => Some(obj.values().map(solve2).sum::<Option<i64>>().unwrap_or(0)),
        Value::Array(arr) => Some(arr.iter().map(|x| solve2(x).unwrap_or(0)).sum()),
        Value::Number(num) => Some(num.as_i64().unwrap()),
        Value::String(s) if s == "red" => None,
        Value::String(_) => Some(0),
        Value::Null | Value::Bool(_) => unreachable!(),
    }
}

pub fn part2(input: &Value) -> i64 {
    solve2(input).unwrap_or(0)
}
