use comrak::{markdown_to_html, ComrakOptions};

pub fn render(md: &str) -> String {
    let mut opts = ComrakOptions::default();
    opts.extension.strikethrough = true;
    opts.extension.table = true;
    opts.extension.autolink = true;
    opts.extension.tasklist = true;
    opts.render.unsafe_ = true;
    markdown_to_html(md, &opts)
}
