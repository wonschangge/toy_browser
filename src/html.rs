//! 简易HTML解析器
//！
//！ TODO：
//！ * <!DOCTYPE>和预处理命令
//！ * 错误格式内容

use dom;
use std::collections::HashMap;

/// 解析一份HTML文档字符串，返回根节点
pub fn parse(source: String) -> dom::Node {
    let nodes = Parser { pos: 0, input: source }.parse_nodes();
    
    dom::elem("html".to_string(), HashMap::new(), nodes)
}

struct Parser {
    pos: usize,
    input: String,
}

#[derive(Debug)]
pub enum SingleTag {
    Link,
    Meta,
}

impl Parser {
    /// 解析一个相邻节点队列
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = vec!();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// 解析单个节点
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => {
                if self.input[self.pos+1..].chars().next().unwrap() == '!' {
                    self.parse_text()
                } else {
                    self.parse_element()
                }
            },
            _   => self.parse_text(),
        }
    }

    /// 解析单个元素，包括它的开标签、内容和闭标签
    fn parse_element(&mut self) -> dom::Node {
        // 开标签
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        // 单标签检测并返回
        if self.if_single_tag(&tag_name) {
            return dom::elem(tag_name, attrs, vec![])
        }
        // 内容
        let children = self.parse_nodes();

        // 闭标签
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        dom::elem(tag_name, attrs, children)
    }

    fn if_single_tag(&self, tag_name: &String) -> bool {
        match tag_name.as_str() {
            "link" => true,
            "meta" => true,
            "img" => true,
            "hr" => true,
            "input" => true,
            _ => false
        }
    }

    /// 解析标签名或属性名
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' => true,
            _ => false
        })
    }

    /// 解析一个形如name="value"的键值对列表，用空格符分隔
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop  {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    /// 解析单个形如name="value"的键值对
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    /// 解析带引号的属性值
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        value
    }

    /// 解析文本节点
    fn parse_text(&mut self) -> dom::Node {
        self.parse_comments();
        dom::text(self.consume_while(|c| c != '<'))
    }

    /// 解析HTML和Style中的注释
    fn parse_comments(&mut self) -> String {
        let mut charvec = vec![];
        if self.input[self.pos..].chars().next().unwrap() == '/' && 
           self.input[self.pos+1..].chars().next().unwrap() == '*' {
               while !(self.next_char() == '*' && self.input[self.pos+1..].chars().next().unwrap() == '/') {
                   charvec.push(self.consume_char());
                }
                charvec.push(self.consume_char());
                charvec.push(self.consume_char());
            }
        if self.input[self.pos..].chars().next().unwrap() == '<' && 
           self.input[self.pos+1..].chars().next().unwrap() == '!' &&
           self.input[self.pos+2..].chars().next().unwrap() == '-' &&
           self.input[self.pos+3..].chars().next().unwrap() == '-' {
               while !(self.next_char() == '-' && self.input[self.pos+1..].chars().next().unwrap() == '>') {
                    charvec.push(self.consume_char());
                }
                charvec.push(self.consume_char());
                charvec.push(self.consume_char());
                charvec.push(self.consume_char());
            }
        charvec.iter().collect()
    }

    /// 使用并丢弃零个或多个空白字符
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// 处理当前字符直到字符串终止（期间会移动self.pos)
    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            match self.input[self.pos..].chars().next() {
                Some('"') => {
                    self.consume_char();
                    result.push('\\');
                    result.push('"');
                },
                Some('\t') => {
                    self.consume_char();
                    result.push('\\');
                    result.push('t');
                },
                Some('\n') => {
                    self.consume_char();
                    result.push('\\');
                    result.push('n');
                },
                _ => {
                    result.push(self.consume_char());
                },
            };
        }
        result
    }

    /// 返回当前字符并将self.pos后移一位
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// 只读取当前self.pos所指字符
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// 判断当前位置字符输入是否以给定的字符串为开头
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// 判断字符串终止
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}