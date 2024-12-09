use crate::db::{get_once, DDDb, RawKey};
use crate::templates::{BadReqest, Index, Message, NotFound};
use askama_axum::Template;
use axum::{
    extract::{Form, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
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
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {
    let key = RawKey::new(key.as_bytes(), pwd.as_bytes());
    match crate::db::insert(&db, &key, text.as_bytes()) {
        Ok(_) => Ok((StatusCode::CREATED, Index.render().unwrap())),
        Err(_) => Err((StatusCode::BAD_REQUEST, BadReqest.render().unwrap())),
    }
}

pub async fn api_get(
    State(db): State<DDDb>,
    Query(g @ Get { .. }): Query<Get>,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
    if let Some(k) = g.to_key() {
        let res = get_once(&db, &k).unwrap();
        if let Some(val) = res {
            Ok(Message {
                message: std::str::from_utf8(&val).unwrap(),
            }
            .render()
            .unwrap())
        } else {
            Err((StatusCode::NOT_FOUND, NotFound.render().unwrap()))
        }
    } else {
        Ok(Index.render().unwrap())
    }
}
