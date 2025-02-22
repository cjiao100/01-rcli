use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::cli::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
// 这里的字段名要和csv文件的header一致
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    // 这里的字段名要和csv文件的header一致
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // unwrap 和 ? 都是用来处理Result的，如果是Ok，unwrap会返回Ok中的值，如果是Err，unwrap会panic
    // 如果是Ok，?会返回Ok中的值，如果是Err，?会将错误传播到调用该函数的地方，而不是立即崩溃。
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // let records = reader
    //     .deserialize::<Player>()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();

    // clone() 的用处
    // reader.headers() 和 reader.records() 都会声明一个 &mut self 的引用，这两个方法不能同时调用，否则会报错。
    // 通过 clone() 方法，可以将 reader.headers() 的引用克隆一份，这样就不会出现同时调用两个方法的情况。
    let headers = reader.headers()?.clone();

    // 读取csv文件的内容
    for result in reader.records() {
        let record = result?;
        // headers.iter() 使用headers的迭代器
        // record.iter() 使用record的迭代器
        // zip 将两个迭代器合并为一个元组的迭代器 [(headers, record), ...]
        // collect::<Value>() 将元组的迭代器转换为Value类型
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        println!("{:?}", json_value);
        // 将读取到的内容存入Vec<Player>中
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    // 通过serde_json将Vec<Player>序列化为json字符串
    // let json = serde_json::to_string_pretty(&ret)?;
    // 将json内容写入文件
    fs::write(output, content)?;

    Ok(())
}
