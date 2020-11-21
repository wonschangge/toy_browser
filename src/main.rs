use std::io::Read;
use std::fs::File;

pub mod dom;
pub mod html;

// 读取文本文件，输出字符串
fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}

fn main() {
    // 第一步---读取本地文件
    let html = read_source("examples/w3c_html_standard_parse.html".to_string());
    // let html = read_source("examples/hello.html".to_string());
    // println!("{}", html);
    
    // 第二步---解析HTML文档字符串并构建本地DOM树
    let root_node = html::parse(html);
    // println!("{:?}", root_node);            // 打印查看构建的DOM树结构
    println!("{}", root_node.tojson());        // 打印查看转换为JSON格式的DOM树
}
