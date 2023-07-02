mod core;
mod sandbox;
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_json_file(path: &Path) -> Value {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let json: Value = serde_json::from_str(&contents).unwrap();
    return json;
}

fn process_json_value(value: &Value, indent: usize) {
    match value {
        Value::Null => println!("{}null", "  ".repeat(indent)),
        Value::Bool(v) => println!("{}{}", "  ".repeat(indent), v),
        Value::Number(v) => println!("{}{}", "  ".repeat(indent), v),
        Value::String(v) => println!("{}\"{}\"", "  ".repeat(indent), v),
        Value::Array(arr) => {
            println!("{}[", "  ".repeat(indent));
            for item in arr {
                process_json_value(item, indent + 1);
            }
            println!("{},", "  ".repeat(indent));
            println!("{}]", "  ".repeat(indent));
        }
        Value::Object(obj) => {
            println!("{}{{", "  ".repeat(indent));
            for (key, value) in obj {
                println!("{}\"{}\":", "  ".repeat(indent + 1), key);
                process_json_value(value, indent + 2);
            }
            println!("{},", "  ".repeat(indent));
            println!("{}}}", "  ".repeat(indent));
        }
    }
}

fn main() {
    let file_path = Path::new("src/sandbox/cities.json");
    let json_result = read_json_file(&file_path);
    process_json_value(&json_result, 0)
}
