use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    // unwrap 和 ? 都是用来处理Result的，如果是Ok，unwrap会返回Ok中的值，如果是Err，unwrap会panic
    // 如果是Ok，?会返回Ok中的值，如果是Err，?会将错误传播到调用该函数的地方，而不是立即崩溃。
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // let records = reader
    //     .deserialize::<Player>()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();

    // 读取csv文件的内容
    for result in reader.deserialize() {
        let record: Player = result?;
        // 将读取到的内容存入Vec<Player>中
        ret.push(record);
    }

    // 通过serde_json将Vec<Player>序列化为json字符串
    let json = serde_json::to_string_pretty(&ret)?;
    // 将json内容写入文件
    fs::write(output, json)?;

    Ok(())
}
