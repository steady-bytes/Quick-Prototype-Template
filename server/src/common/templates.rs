use::std::sync::Arc;
use std::include_str;

use tera::{Tera};

pub type Templates = Arc<Tera>;
pub type Context = tera::Context;

pub fn new() -> Templates {
    let mut tera = Tera::default();

    tera.
        add_raw_templates(vec![
            ("hello", include_str!("../../templates/hello.html")),
            ("signup_page", include_str!("../../templates/signup.html")),
            ("app", include_str!("../../templates/app.html"))
        ]).unwrap();

    Arc::new(tera)
}

pub fn new_template_context() -> Context {
    tera::Context::new()
}