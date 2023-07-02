mod core;
fn main() {
    println!("Hello, world!");
    let words = vec!["hello", "world"];
    let hello_world = words.join("");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
