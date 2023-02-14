use std::collections::HashMap;
use std::str::{from_utf8, FromStr};
use hex::ToHex;
use mysql_async::{Pool, Value};
use mysql_async::consts::ColumnType;
use mysql_async::prelude::{Queryable, ToValue};
use serde::{Serialize, Serializer};
use serde_json::{json, Number, Value as JsonValue};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{command, Manager, RunEvent, Runtime, State};
use tauri::utils::config::PluginConfig;
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error)]
pub enum Err {
    #[error(transparent)]
    Mysql(#[from] mysql_async::Error),
    #[error("error: {0}")]
    Other(String),
}

impl Serialize for Err {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T> = std::result::Result<T, Err>;

#[derive(Default)]
pub struct Pools(Mutex<HashMap<String, Pool>>);

#[command]
async fn load(
    pools: State<'_, Pools>,
    db: String,
) -> Result<()> {
    let mut pools = pools.0.lock().await;
    if pools.get(&db).is_some() {
        return Ok(());
    }
    let pool = Pool::new(db.as_str());
    pools.insert(db.clone(), pool);
    Ok(())
}

#[command]
async fn exec(
    pools: State<'_, Pools>,
    db: String,
    sql: String,
) -> Result<u64> {
    let pools = pools.0.lock().await;
    let pool = pools.get(&db).ok_or(Err::Other("pool load failed".to_string()))?;
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(sql, {}).await?;
    let rst = conn.affected_rows();
    Ok(rst)
}

#[command]
async fn query(
    pools: State<'_, Pools>,
    db: String,
    sql: String,
) -> Result<JsonValue> {
    let pools = pools.0.lock().await;
    let pool = pools.get(&db).ok_or(Err::Other("pool load failed".to_string()))?;
    let mut conn = pool.get_conn().await?;
    let mut qrst = conn.query_iter(sql).await?;
    let cols = qrst.columns().ok_or(Err::Other("get columns failed".to_string()))?;
    let types = cols.iter().map(|c| { get_type(c.column_type()) }).collect::<Vec<String>>();
    let cols = json!(cols.iter().map(|c| { json!(c.name_str()) }).collect::<Vec<JsonValue>>());
    let rows = qrst.map(|row| {
        let row = row.unwrap();
        let mut vals: Vec<JsonValue> = Vec::default();
        for (i, r) in row.iter().enumerate() {
            vals.push(get_val(r.to_value(), &types[i]));
        }
        json!(vals)
    }).await?;
    let mut map: HashMap<String, JsonValue> = HashMap::default();
    map.insert("cols".to_string(), cols);
    map.insert("rows".to_string(), json!(rows));
    drop(conn);
    Ok(json!(map))
}

fn get_type(ct: ColumnType) -> String {
    match ct {
        ColumnType::MYSQL_TYPE_NULL => "Null".to_string(),
        ColumnType::MYSQL_TYPE_TINY |
        ColumnType::MYSQL_TYPE_SHORT |
        ColumnType::MYSQL_TYPE_LONG |
        ColumnType::MYSQL_TYPE_FLOAT |
        ColumnType::MYSQL_TYPE_DOUBLE |
        ColumnType::MYSQL_TYPE_DECIMAL |
        ColumnType::MYSQL_TYPE_LONGLONG |
        ColumnType::MYSQL_TYPE_NEWDECIMAL |
        ColumnType::MYSQL_TYPE_TIMESTAMP |
        ColumnType::MYSQL_TYPE_TIMESTAMP2 |
        ColumnType::MYSQL_TYPE_INT24 => "Number".to_string(),
        ColumnType::MYSQL_TYPE_BIT => "Bool".to_string(),
        ColumnType::MYSQL_TYPE_TINY_BLOB |
        ColumnType::MYSQL_TYPE_MEDIUM_BLOB |
        ColumnType::MYSQL_TYPE_LONG_BLOB |
        ColumnType::MYSQL_TYPE_BLOB => "Blob".to_string(),
        _ => "String".to_string()
    }
}

fn get_val(val: Value, ct: &str) -> JsonValue {
    match val {
        Value::NULL => JsonValue::Null,
        Value::Int(x) => json!(x),
        Value::UInt(x) => json!(x),
        Value::Float(x) => json!(x),
        Value::Double(x) => json!(x),
        Value::Date(y, m, d, 0, 0, 0, 0) => json!(format!("{:04}-{:02}-{:02}", y, m, d)),
        Value::Date(year, month, day, hour, minute, second, 0) => JsonValue::String(format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month, day, hour, minute, second
        )),
        Value::Date(year, month, day, hour, minute, second, micros) => JsonValue::String(format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
            year, month, day, hour, minute, second, micros
        )),
        Value::Time(neg, d, h, i, s, 0) => {
            let t = format!("{:03}:{:02}:{:02}", d * 24 + u32::from(h), i, s);
            if neg { json!(String::from("-").push_str(&t)) } else { json!(t) }
        }
        Value::Time(neg, days, hours, minutes, seconds, micros) => {
            let t = format!("-{:03}:{:02}:{:02}.{:06}", days * 24 + u32::from(hours), minutes, seconds, micros);
            if neg { json!(String::from("-").push_str(&t)) } else { json!(t) }
        }
        Value::Bytes(ref bytes) => match ct {
            "Blob" => cvt_blob(bytes),
            _ => match from_utf8(&*bytes) {
                Ok(v) => match ct {
                    "Null" => JsonValue::Null,
                    "Number" => JsonValue::Number(Number::from_str(v).unwrap()),
                    "Bool" => JsonValue::Bool(v == "1"),
                    _ => json!(v)
                },
                Err(_) => cvt_blob(bytes)
            },
        }
    }
}

fn cvt_blob(bytes: &Vec<u8>) -> JsonValue {
    json!(bytes.encode_hex_upper::<String>())
}

pub fn build<R: Runtime>() -> TauriPlugin<R, Option<PluginConfig>> {
    Builder::new("mysql")
        .invoke_handler(tauri::generate_handler![load,query,exec])
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let pools = Pools::default();
                app.manage(pools);
                Ok(())
            })
        })
        .on_event(|app, event| {
            if let RunEvent::Exit = event {
                tauri::async_runtime::block_on(async move {
                    let pools = &*app.state::<Pools>();
                    let pools = pools.0.lock().await;
                    for value in pools.values() {
                        let _ = value.to_owned().disconnect().await;
                    }
                });
            }
        })
        .build()
}
