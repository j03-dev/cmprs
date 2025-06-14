use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub data: Option<String>,
    pub occurrence: i32,
}

impl Node {
    pub fn add(left: Node, right: Node) -> Self {
        Self {
            left: Some(Box::new(left.clone())),
            right: Some(Box::new(right.clone())),
            data: None,
            occurrence: left.occurrence + right.occurrence,
        }
    }

    pub fn search(&self, target: &str, path: &mut Vec<u8>) -> Option<Vec<u8>> {
        if let Some(data) = &self.data {
            if data == target {
                return Some(path.clone());
            }
        }

        if let Some(left) = &self.left {
            path.push(0);
            if let Some(left_path) = left.search(target, path) {
                return Some(left_path);
            }
            path.pop();
        }

        if let Some(right) = &self.right {
            path.push(1);
            if let Some(right_path) = right.search(target, path) {
                return Some(right_path);
            }
            path.pop();
        }

        None
    }
}
