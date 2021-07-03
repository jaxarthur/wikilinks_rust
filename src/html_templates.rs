use askama::Template;

#[derive(Template)]
#[template(path = "outline.html")]
struct OutlineTemplate {
    body: String
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    num: u16,
    text: String
}

pub fn error_template(num: u16, text: String) -> String {
    let rendered_error = ErrorTemplate{num, text}.render().unwrap();
    let rendered_page = OutlineTemplate{body: rendered_error}.render().unwrap();
    return rendered_page;
}
