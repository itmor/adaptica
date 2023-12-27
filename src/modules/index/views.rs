use sailfish::TemplateOnce;


#[derive(TemplateOnce)]
#[template(path = "modules/index/views/index.stpl")]
pub struct IndexTemplate;

#[derive(TemplateOnce)]
#[template(path = "modules/index/views/login_form.stpl")]
pub struct LoginTemplate;