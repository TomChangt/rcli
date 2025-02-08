use crate::cli::OutputFormat;
use csv::Reader;
use std::fs;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut records = Vec::with_capacity(128);

    // 将 CSV 记录转换为更适合序列化的格式
    for result in reader.records() {
        let record = result?;
        let mut map = serde_json::Map::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            map.insert(
                header.to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
        records.push(serde_json::Value::Object(map));
    }

    // 根据不同格式进行序列化
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
        OutputFormat::Toml => {
            // 将记录包装在一个对象中，使其符合 TOML 格式要求
            let wrapper = serde_json::json!({ "records": records });
            toml::to_string_pretty(&wrapper)?
        }
    };

    fs::write(output, content)?;
    Ok(())
}
