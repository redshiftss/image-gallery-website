use axum::{
    routing::get,
    Router, Extension,
    response::{Html}
};

use handlebars::{
    Handlebars
};

use rust_embed::RustEmbed;
use serde_json::json;

#[derive(RustEmbed)]
#[folder = "src/templates"]
struct Assets;

#[tokio::main]
async fn main () {
    let mut reg = Handlebars::new();
    reg.register_embed_templates::<Assets>().unwrap();

    let app = Router::new()
                        .route("/gallery", get(gallery))
                        .route("/upload_image", get(upload))
                        .layer(&Extension(reg));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn upload (Extension(reg) : Extension<Handlebars<'_>>) -> Html<String>{
    let rendered = reg.render("upload_image.html", &json!({})).unwrap();
    Html(rendered)
}

async fn gallery (Extension(reg) : Extension<Handlebars<'_>>) -> Html<String>{
    let rendered = reg.render("gallery.html", &json!({})).unwrap();
    Html(rendered)
}

