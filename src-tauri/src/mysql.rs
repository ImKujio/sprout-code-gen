use std::collections::HashMap;
use std::ops::Index;
use std::str::from_utf8;
use mysql_async::{Column, Pool, Value};
use mysql_async::prelude::Queryable;
use serde::{Serialize, Serializer};
use serde_json::{Number, Value as JsonValue};
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
    let mut pool_map = pools.0.lock().await;
    if pool_map.get(&db).is_some() {
        return Ok(());
    }
    let pool = Pool::new(db.as_str());
    pool_map.insert(db.clone(), pool);
    Ok(())
}

#[command]
async fn query(
    pools: State<'_, Pools>,
    db: String,
    query: String,
) -> Result<Vec<HashMap<String, JsonValue>>> {
    let mut pool_map = pools.0.lock().await;
    let pool = pool_map.get(&db).ok_or(Err::Other("pool load failed".to_string()))?;
    let mut conn = pool.get_conn().await?;
    let mut rst_set = conn.query_iter(query).await?;
    let mut rst = Vec::default();
    rst_set.for_each(|mut row| {
        let mut map = HashMap::default();
        let cols = row.columns();
        let row = row.unwrap();
        for i in 0..cols.len() {
            let col = match cols.get(i) {
                None => { i.to_string() }
                Some(c) => { c.name_str().to_string() }
            };
            let val = match row.get(i).unwrap() {
                Value::NULL => JsonValue::Null,
                Value::Int(x) => JsonValue::from(x),
                Value::UInt(x) => JsonValue::from(x),
                Value::Float(x) => JsonValue::from(x),
                Value::Double(x) => JsonValue::from(x),
                Value::Date(y, m, d, 0, 0, 0, 0) => JsonValue::String(format!("{:04}-{:02}-{:02}", y, m, d)),
                Value::Date(year, month, day, hour, minute, second, 0) => JsonValue::String(format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    year, month, day, hour, minute, second
                )),
                Value::Date(year, month, day, hour, minute, second, micros) => JsonValue::String(format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                    year, month, day, hour, minute, second, micros
                )),
                Value::Time(neg, d, h, i, s, 0) => {
                    if neg {
                        JsonValue::String(format!("-{:03}:{:02}:{:02}", d * 24 + u32::from(h), i, s))
                    } else {
                        JsonValue::String(format!("{:03}:{:02}:{:02}", d * 24 + u32::from(h), i, s))
                    }
                }
                Value::Time(neg, days, hours, minutes, seconds, micros) => {
                    if neg {
                        JsonValue::String(format!(
                            "-{:03}:{:02}:{:02}.{:06}",
                            days * 24 + u32::from(hours),
                            minutes,
                            seconds,
                            micros
                        ))
                    } else {
                        JsonValue::String(format!(
                            "{:03}:{:02}:{:02}.{:06}",
                            days * 24 + u32::from(hours),
                            minutes,
                            seconds,
                            micros
                        ))
                    }
                }
                Value::Bytes(ref bytes) => match from_utf8(&*bytes) {
                    Ok(string) => JsonValue::String(string.to_string()),
                    Err(_) => {
                        let mut s = String::from("0x");
                        for c in bytes.iter() {
                            s.extend(format!("{:02X}", *c).chars())
                        }
                        JsonValue::String(s)
                    }
                },
            };
            map.insert(col,val);
        }
        rst.push(map)
    }).await?;
    drop(conn);
    Ok(rst)
}

pub fn build<R: Runtime>() -> TauriPlugin<R, Option<PluginConfig>> {
    Builder::new("mysql")
        .invoke_handler(tauri::generate_handler![load,query])
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
