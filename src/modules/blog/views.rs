use sailfish::TemplateOnce;


#[derive(TemplateOnce)]
#[template(path = "modules/blog/views/add_post.stpl")]
pub struct AddPostTemplate;