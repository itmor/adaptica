use sailfish::TemplateOnce;
 
#[derive(TemplateOnce)]
#[template(path = "shared/templates/base.stpl")]
pub struct BaseTemplate {
   pub content: String,
   pub css_paths: Vec<String>
}

#[derive(TemplateOnce)]
#[template(path = "shared/templates/header.stpl")]
pub struct HeaderTemplate;

#[derive(TemplateOnce)]
#[template(path = "shared/templates/footer.stpl")]
pub struct FooterTemplate;