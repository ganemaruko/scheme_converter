mod core;
mod sandbox;
use crate::core::tree;
use crate::core::tree::core::Tree;
use serde_json::{Result, Value};
use std::collections::BTreeMap;
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

#[derive(Debug)]
enum Leaf {
    String(String),
    Number(i64),
    Boolean(bool),
}

// fn process_json_value<'a>(
//     key: Option<&str>,
//     value: &'a Value,
//     indent: usize,
//     tree: &'a mut Tree<Leaf>,
// ) -> &'a mut Tree<Leaf> {
//     match value {
//         Value::Null => {
//             println!("{}null", "  ".repeat(indent));
//             return tree;
//         }
//         Value::Bool(v) => {
//             println!("{}{}", "  ".repeat(indent), v);
//             return tree;
//         }
//         Value::Number(v) => {
//             println!("{}{}", "  ".repeat(indent), v);
//             return tree;
//         }
//         Value::String(v) => {
//             println!("{}\"{}\"", "  ".repeat(indent), v);
//             if let Some(key) = key {
//                 tree.append(
//                     key,
//                     Tree::Leaf {
//                         value: Leaf::String(v.to_string()),
//                     },
//                 );
//             }

//             return tree;
//         }
//         Value::Array(arr) => {
//             println!("{}[", "  ".repeat(indent));
//             for item in arr {
//                 process_json_value(key, item, indent + 1, tree);
//             }
//             println!("{},", "  ".repeat(indent));
//             println!("{}]", "  ".repeat(indent));
//             return tree;
//         }
//         Value::Object(obj) => {
//             println!("{}{{", "  ".repeat(indent));
//             for (key, value) in obj {
//                 println!("{}\"{}\":", "  ".repeat(indent + 1), key);
//                 process_json_value(key, value, indent + 2, tree);
//             }
//             println!("{},", "  ".repeat(indent));
//             println!("{}}}", "  ".repeat(indent));
//             return tree;
//         }
//     }
// }

fn make_tree_from_json(json: &Value, key: Option<&String>, tree: Option<Tree<Leaf>>) -> Tree<Leaf> {
    let mut current_tree;
    if (tree.is_none()) {
        println!("tree is none");
        // TODO i don't know the way that BTreeMap create by from is correct.
        current_tree = Tree::MapBranch {
            children: BTreeMap::new(),
        };
    } else {
        println!("tree exists");
        current_tree = tree.unwrap();
    }
    match json {
        Value::Null => {
            println!("null");
            if let Some(key) = key {
                current_tree.append(
                    key.to_string(),
                    Tree::Leaf {
                        value: Leaf::String("null".to_string()),
                    },
                );
            }
        }
        Value::Bool(v) => {
            println!("{}", v);
            if let Some(key) = key {
                current_tree.append(
                    key.to_string(),
                    Tree::Leaf {
                        value: Leaf::Boolean(v.to_owned()),
                    },
                );
            }
        }
        Value::Number(v) => {
            println!("{}", v);
            if let Some(key) = key {
                current_tree.append(
                    key.to_string(),
                    Tree::Leaf {
                        value: Leaf::Number(v.as_i64().unwrap()),
                    },
                );
            }
        }
        Value::String(v) => {
            println!("{}", v);
            if let Some(key) = key {
                current_tree.append(
                    key.to_string(),
                    Tree::Leaf {
                        value: Leaf::String(v.to_string()),
                    },
                );
            }
        }
        Value::Array(arr) => {
            println!("[");
            for item in arr {
                current_tree = make_tree_from_json(item, None, Some(current_tree));
            }
            println!("],");
        }
        Value::Object(obj) => {
            println!("{{");
            let mut new_tree = Tree::MapBranch {
                children: BTreeMap::new(),
            };
            for (key, value) in obj {
                println!("\"{}\":", key);

                new_tree = make_tree_from_json(value, Some(key), Some(new_tree));
            }
            if let Some(key) = key {
                current_tree.append(key.to_string(), new_tree);
            }
            println!("}}");
        }
    }
    return current_tree;
}

fn calc_fibonach_recursively(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return calc_fibonach_recursively(n - 1) + calc_fibonach_recursively(n - 2);
    }
}

fn main() {
    let file_path = Path::new("src/sandbox/cities.json");
    let json_result = read_json_file(&file_path);
    // let mut tree = Tree::MapBranch {
    //     children: BTreeMap::from([
    //         ("key1".to_string(), Tree::Leaf { value: 1 }),
    //         ("key2".to_string(), Tree::Leaf { value: 2 }),
    //     ]),
    // };
    let mut tree: Tree<Leaf> = Tree::MapBranch {
        children: BTreeMap::from([
            // ("key1".to_string(), Tree::Leaf { value: 1 }),
            // ("key2".to_string(), Tree::Leaf { value: 2 }),
        ]),
    };
    let tree = make_tree_from_json(&json_result, None, None);
    println!("{:?}", tree);
}
