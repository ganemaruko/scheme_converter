#[cfg(test)]
mod sandbox {
    use serde::{Deserialize, Serialize};

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;

    #[derive(Serialize, Deserialize, Debug)]

    struct Cities {
        cities: HashMap<String, HashMap<String, String>>,
    }
    #[test]
    fn main() {
        eprintln!("*** 開始 ***");
        let file_name = "src/sandbox/cities.json";

        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        let deserialized: Cities = serde_json::from_reader(reader).unwrap();

        for (key, value) in deserialized.cities.iter() {
            print!("{}", key);
            print!("\t");
            print!("{}", value["name"]);
            print!("\t");
            print!("{}", value["population"]);
            print!("\t");
            println!("{}", value["date_mod"]);
        }

        eprintln!("*** 終了 ***");
    }

    // --------------------------------------------------------------------
}
