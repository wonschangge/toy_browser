///! 基础版的DOM数据结构体

use std::collections::{HashMap, HashSet};

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,        // 所有节点的共有数据
    pub node_type: NodeType,        // 每种节点类型的特有数据
}

impl Node {
    pub fn tojson(&self) -> String {
        let mut str = String::from("");
        match &self.node_type {
            NodeType::Element(ElementData { tag_name, attributes }) => {
                str += "{\"node_type\": \"element\", \"tag_name\": \"";
                str += tag_name;
                str += "\", \"attributes\": {";
                str += &self.get_attributes_string(attributes);
                str += "},\"children\": [";
                str += &self.get_children_nodes();
                str += "]}";
            },
            NodeType::Text(data) => {
                str += "{\"node_type\": \"text\", \"data\": \"";
                str += data.trim();
                str += "\"}"
            },
        }
        str
    }
    fn get_attributes_string(&self, attrmap: &AttrMap) -> String {
        let mut str = String::from("");
        for (name, value) in attrmap {
            if str.len() > 0 {
                str += ","
            }
            str += "\"";
            str += name;
            str += "\": \"";
            str += value;
            str += "\"";
        }
        str
    }
    fn get_children_nodes(&self) -> String {
        let mut str = String::from("");
        for i in 0..self.children.len() {
            if i > 0 {
                str += ",";
            }
            str += &self.children[i].tojson();
        }
        str
    }
}

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

// 元素节点数据块
impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}

/// 文本节点构造体
pub fn text(data: String) -> Node {
    Node {
        children: vec![],
        node_type: NodeType::Text(data)
    }
}

/// 元素节点构造体
pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}