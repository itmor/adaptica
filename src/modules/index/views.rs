use sailfish::TemplateOnce;


#[derive(TemplateOnce)]
#[template(path = "modules/index/views/index.stpl")]
pub struct IndexTemplate;