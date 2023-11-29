use askama::Template;

#[derive(Template)]
#[template(path = "modules/index/views/index.html")]
pub struct IndexTemplate;