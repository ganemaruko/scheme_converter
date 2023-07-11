mod leaf;
mod list;

pub mod core {
    use serde_json::{Result, Value};
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use super::leaf::tree::Leaf;

    /// - 順序が必要な場合があるため、HashMapではなくBTreeを用いています.
    /// - LeafTypeはDebugを実装している必要があります.
    #[derive(Debug)]
    pub enum Tree<LeafType: std::fmt::Debug> {
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

    impl<LeafType : std::fmt::Debug> Tree<LeafType> {
        pub fn append(&mut self, key: String, value: Tree<LeafType>) {
            println!("{}", key);
            match self {
                Tree::ListBranch { children } => {
                    children.push(value);
                }
                Tree::MapBranch { children } => {
                    println!("MapBranch {}, {:?}", key, value);
                    children.insert(key, value);
                }
                Tree::Leaf { value } => {
                    panic!("Leafにはappendできません");
                }
            }
        }
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
    fn array_branch_can_append() {
        use super::core::Tree;

        let mut tree = Tree::MapBranch {
            children: BTreeMap::from([
                ("key1".to_string(), Tree::Leaf { value: 1 }),
                ("key2".to_string(), Tree::Leaf { value: 2 }),
            ]),
        };
        tree.append("sample".to_string(), Tree::Leaf { value: 1 });
    }
}
