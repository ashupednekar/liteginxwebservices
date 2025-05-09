use askama::Template;

#[derive(Template)]
#[template(path = "buckets.html")]
pub struct Buckets<'a> {
    pub username: &'a str,
}

#[derive(Template)]
#[template(path = "containers.html")]
pub struct Containers<'a> {
    pub username: &'a str,
}

#[derive(Template)]
#[template(path = "functions.html")]
pub struct Functions<'a> {
    pub username: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home<'a> {
    pub username: &'a str,
}
