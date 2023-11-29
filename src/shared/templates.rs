use askama::Template;

#[derive(Template)]
#[template(path = "shared/templates/base.html")]
pub struct BaseTemplate;

#[derive(Template)]
#[template(path = "shared/templates/header.html")]
pub struct HeaderTemplate;

#[derive(Template)]
#[template(path = "shared/templates/footer.html")]
pub struct FooterTemplate;