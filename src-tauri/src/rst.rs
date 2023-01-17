use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Rst {
    ok: bool,
    msg: String,
    data: Option<String>,
}

impl Rst {
    pub fn ok(data: Option<String>) -> Rst {
        return Rst { ok: true, msg: "succeed".to_string(), data };
    }
    pub fn err(msg: String) -> Rst {
        return Rst { ok: false, msg, data: None };
    }
}