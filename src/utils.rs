use anyhow::Result;
use std::{
    fs::File,
    io::{stdin, BufReader, Read},
};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // 用Box和syn还有Trait消除多种类型
    let reader: Box<dyn Read> = if input == "-" {
        println!("请输入内容 (按Ctrl+D或Ctrl+Z结束):"); // 更新提示信息
        Box::new(BufReader::new(stdin()))
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}
