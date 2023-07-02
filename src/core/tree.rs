mod leaf;
mod list;

pub mod core {
    use serde_json::{Result, Value};
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use super::leaf::tree::Leaf;

    /// - 順序が必要な場合があるため、BTreeを用いています.
    ///
    pub enum Tree<LeafType> {
        ListBranch {
            children: Vec<Tree<LeafType>>,
        },
        MapBranch {
            children: BTreeMap<String, Tree<LeafType>>,
        },
        Leaf {
            value: LeafType,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn minimum_tree_can_generate() {
        use super::core::Tree;

        let tree = Tree::Leaf { value: 1 };
        let tree = Tree::Leaf { value: "aaa" };
        let tree = Tree::ListBranch {
            children: vec![Tree::Leaf { value: 1 }],
        };
        // assert_eq!(tree, Tree::Leaf { value: 1 });
    }

    #[test]
    fn tree_can_override() {
        use super::core::Tree;

        let tree = Tree::MapBranch {
            children: BTreeMap::from([
                ("key1".to_string(), Tree::Leaf { value: 1 }),
                ("key2".to_string(), Tree::Leaf { value: 2 }),
            ]),
        };
        
    }
}
