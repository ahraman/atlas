use askama::Template;

#[derive(Debug, Clone, Copy, Default)]
pub struct SiteInfo<'a> {
    pub name: &'a str,
}

impl<'a> SiteInfo<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PageInfo<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

impl<'a> PageInfo<'a> {
    pub fn new(title: &'a str, content: &'a str) -> Self {
        Self { title, content }
    }
}

#[derive(Debug, Template)]
#[template(path = "page-view.html")]
pub struct PageViewTemplate<'a> {
    pub site: SiteInfo<'a>,
    pub page: PageInfo<'a>,
}

impl<'a> PageViewTemplate<'a> {
    pub fn new(site: SiteInfo<'a>, page: PageInfo<'a>) -> Self {
        Self { site, page }
    }
}
