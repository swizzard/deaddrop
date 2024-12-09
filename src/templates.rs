use askama_axum::Template;

#[derive(Debug, Template)]
#[template(path = "message.html")]
pub struct Message<'a> {
    pub message: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFound;

#[derive(Template)]
#[template(path = "bad_request.html")]
pub struct BadReqest;
