use askama::Template;

#[derive(Template)]
#[template(path = "outline.html", escape = "none")]
struct OutlineTemplate {
    body: String
}

#[derive(Template)]
#[template(path = "entry.html")]
struct EntryTemplate {
    hide_error: bool,
    error_message: String
}

#[derive(Template)]
#[template(path = "submit.html")]
struct SubmitTemplate {
    redirect_url: String,
    message: String
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    num: u16,
    text: String
}

pub fn entry_template(hide_error: bool, error_message: String) -> String {
    let rendered_entry = EntryTemplate{hide_error, error_message}.render().unwrap();
    let rendered_page = OutlineTemplate{body: rendered_entry}.render().unwrap();
    return rendered_page;
}

pub fn submit_template(redirect_url: String, message: String) -> String {
    let rendered_submit = SubmitTemplate{redirect_url, message}.render().unwrap();
    let rendered_page = OutlineTemplate{body: rendered_submit}.render().unwrap();
    return rendered_page;
}

pub fn error_template(num: u16, text: String) -> String {
    let rendered_error = ErrorTemplate{num, text}.render().unwrap();
    let rendered_page = OutlineTemplate{body: rendered_error}.render().unwrap();
    return rendered_page;
}
