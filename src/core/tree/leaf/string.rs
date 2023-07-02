mod leaf {
    use crate::core::converter::core::Converter;
    pub struct StringLeaf {
        pub value: String,
    }

    impl Converter for StringLeaf {
        fn to_json(&self) -> String {
            return format!("\"{}\"", self.value);
        }
        fn to_yml(&self) -> String {
            return self.value.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::leaf::StringLeaf;
    // TODO why this trait should be imported?
    use crate::core::converter::core::Converter;

    #[test]
    fn string_leaf_to_json_shold_return_with_double_quatation() {
        let leaf = StringLeaf {
            value: String::from("sample value"),
        };
        assert_eq!(leaf.to_json(), "\"sample value\"")
    }
    #[test]
    fn string_leaf_to_yml_shold_return_raw_string_format() {
        let leaf = StringLeaf {
            value: String::from("sample value"),
        };
        assert_eq!(leaf.to_yml(), "sample value")
    }
}
