pub mod core {
    pub trait Converter {
        ///
        ///
        ///
        fn to_json(&self) -> String;
        fn to_yml(&self) -> String;
    }
}
