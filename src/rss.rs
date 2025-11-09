use crate::{db::Db, config::Config};
pub async fn build_rss(db: &Db, cfg: &Config) -> anyhow::Result<String> {
    let rows = sqlx::query!(
      "SELECT slug,title,excerpt,published_at FROM posts WHERE status='published' ORDER BY published_at DESC LIMIT 50"
    ).fetch_all(db).await?;
    let mut items = String::new();
    for r in rows {
        let link = format!("{}/posts/{}", cfg.site_base, r.slug);
        let pubdate = r.published_at.unwrap_or_default();
        items.push_str(&format!(
          "<item><title><![CDATA[{t}]]></title><link>{l}</link><guid>{l}</guid><pubDate>{p}</pubDate><description><![CDATA[{d}]]></description></item>",
          t=r.title, l=link, p=pubdate, d=r.excerpt.unwrap_or_default()
        ));
    }
    Ok(format!(r#"<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0"><channel>
<title>Blog RSS</title><link>{}</link><description>Feed</description>{}
</channel></rss>"#, cfg.site_base, items))
}

pub async fn build_sitemap(db: &Db, cfg: &Config) -> anyhow::Result<String> {
    let rows = sqlx::query!("SELECT slug, updated_at FROM posts WHERE status='published'")
        .fetch_all(db).await?;
    let mut urls = String::new();
    for r in rows {
        urls.push_str(&format!(
          "<url><loc>{}/posts/{}</loc><lastmod>{}</lastmod><changefreq>weekly</changefreq></url>",
          cfg.site_base, r.slug, r.updated_at
        ));
    }
    Ok(format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{}</urlset>"#, urls))
}
