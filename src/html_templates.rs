use askama::Template;

#[derive(Template)]
#[template(path = "outline.html")]
pub struct OutlineTemplate {
    body: String
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    num: u8,
    text: String
}


