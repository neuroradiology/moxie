use actix_web::{
    dev::{MessageBody, ResponseBody, ServiceResponse},
    error::Error as WebError,
    web,
};
use futures::stream::{Stream, StreamExt};
use lol_html::{element, html_content::ContentType, RewriteStrSettings};
use tracing::*;

const RELOAD_ON_CHANGES: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/reloadOnChanges.js"));

pub async fn reload_on_changes_into_html<B>(
    mut response: ServiceResponse<B>,
) -> Result<ServiceResponse<B>, actix_web::error::Error>
where
    B: MessageBody + Unpin,
{
    if let Some(content_type) = response.headers().get("content-type") {
        if content_type != "text/html" {
            return Ok(response);
        }
    }

    let path = response.request().path();
    if path == "/ch-ch-ch-changes" {
        return Ok(response);
    }
    info!({ %path }, "serving html page");

    let mut body = collect_body(response.take_body()).await;

    if let Ok(rewritten) = lol_html::rewrite_str(&body, RewriteStrSettings {
        element_content_handlers: vec![element!("head", |head| {
            info!("inserting script tag");
            head.append("<script>", ContentType::Html);
            head.append(RELOAD_ON_CHANGES, ContentType::Text);
            head.append("</script>", ContentType::Html);
            Ok(())
        })],
        ..Default::default()
    }) {
        body = rewritten;
    }

    Ok(response.map_body(|_, _| ResponseBody::Other(body.into())))
}

async fn collect_body(
    mut body: impl Stream<Item = Result<web::Bytes, WebError>> + Unpin,
) -> String {
    let mut buf = Vec::new();

    while let Some(Ok(bytes)) = body.next().await {
        buf.extend(bytes);
    }

    String::from_utf8(buf).unwrap()
}
