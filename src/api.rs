use crate::db::{get_once, DDDb, RawKey};
use axum::extract::{Form, Query, State};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Insert {
    key: String,
    pwd: String,
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Get {
    key: Option<String>,
    pwd: Option<String>,
}

impl Get {
    fn to_key(&self) -> Option<RawKey> {
        if let Some(k) = &self.key {
            if let Some(p) = &self.pwd {
                return Some(RawKey::new(k.as_bytes(), p.as_bytes()));
            }
        }
        None
    }
}

pub async fn api_insert(
    State(db): State<DDDb>,
    Form(Insert { key, pwd, text }): Form<Insert>,
) -> String {
    let key = RawKey::new(key.as_bytes(), pwd.as_bytes());
    crate::db::insert(&db, &key, text.as_bytes()).unwrap();
    "ok".to_string()
}

pub async fn api_get(State(db): State<DDDb>, Query(g @ Get { .. }): Query<Get>) -> String {
    if let Some(k) = g.to_key() {
        let res = get_once(&db, &k).unwrap();
        if let Some(val) = res {
            String::from_utf8(val).unwrap()
        } else {
            "not found".to_string()
        }
    } else {
        String::from("index")
    }
}
