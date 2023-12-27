use serde_json::Value;
use chrono::NaiveDate;
use chrono::NaiveTime;
use base64::decode;

#[allow(dead_code)] // for future usage
pub struct DatabaseValue {
  data_type: DataType,
  value: Option<Value>,
}

#[allow(dead_code)] // for future usage
enum DataType {
  Int64(i64),
  Int32(i32),
  Float32(f32),
  Double(f64),
  Byte(u8),
  Text(String),
  SmallText(String),
  LargeText(String),
  Json(Value),
  Blob(Vec<u8>),
  Bool(bool),
  Date(String),
  Time(String),
  None,
}

#[allow(dead_code)] // for from_string future usage 
impl DatabaseValue {
  pub fn from_string(data_type: &str, value: Option<&str>) -> Result<DatabaseValue,i32> {
    let data_type = match data_type {
      "int64" => DataType::Int64(value.unwrap_or_default().parse().map_err(|_| 606)?),
      "int32" => DataType::Int32(value.unwrap_or_default().parse().map_err(|_| 607)?),
      "float32" => DataType::Float32(value.unwrap_or_default().parse().map_err(|_| 608)?),
      "double" | "float64" => DataType::Double(value.unwrap_or_default().parse().map_err(|_| 609)?),
      "byte" => DataType::Byte(value.unwrap_or_default().parse().map_err(|_| 610)?),
      "text" => {
        if value.unwrap_or_default().len() <= 255 {
          DataType::Text(value.unwrap_or_default().to_string())
        } else {
          return Err(611);
        }
      }
      "smallText" => {
        if value.unwrap_or_default().len() <= 155 {
          DataType::SmallText(value.unwrap_or_default().to_string())
        } else {
          return Err(612);
        }
      }
      "largeText" => {
        if value.unwrap_or_default().len() <= 65536 {
          DataType::LargeText(value.unwrap_or_default().to_string())
        } else {
          return Err(613);
        }
      }
      "json" => DataType::Json(serde_json::from_str(value.unwrap_or_default()).map_err(|_| 614)?),
      "blob" => DataType::Blob(decode(value.unwrap_or_default()).map_err(|_| 614)?),
      "bool" => DataType::Bool(value.unwrap_or_default().parse().map_err(|_| 616)?),
      "date" => NaiveDate::parse_from_str(value.unwrap_or_default(), "%d/%m/%Y")
      .map(|_| DataType::Date(value.unwrap_or_default().to_string()))
      .map_err(|_| 617)?,
      "time" => NaiveTime::parse_from_str(value.unwrap_or_default(), "%H:%M:%S%.f")
      .map(|_| DataType::Time(value.unwrap_or_default().to_string()))
      .map_err(|_| 618)?,
      "undefined" => DataType::None,
      _ => return Err(619),
    };

    let database_value = DatabaseValue {
      data_type,
      value: Some(match value {
        Some(v) => Value::String(v.to_string()),
        None => Value::Null,
      }),
    };

    Ok(database_value)
  }

  pub fn check(data_type: String) -> Result<(), i32> {
    let dt: Vec<&str> = vec![
      "int64",
      "int32",
      "float32",
      "float64",
      "double",
      "byte",
      "text",
      "smallText",
      "largeText",
      "!null",
      "json",
      "bool",
      "date",
      "time",
      "undefined",
      "incremental",
    ];

    if dt.contains(&data_type.to_lowercase().as_str()) {
      Ok(())
    } else {
      Err(605)
    }
  }
}