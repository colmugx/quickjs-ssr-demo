use std::{collections::HashMap, fs::read_to_string};

use actix_files::Files;
use actix_web::{get, http::StatusCode, App, HttpResponse, HttpServer, Responder};
use rquickjs::{Context, Object, Runtime};

fn make_html() -> String {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

    let result = context.with(|ctx| {
        let _a: String = ctx.eval_file("ssr/dist/server/entry-server.js").unwrap();
        let result: Object = ctx.eval(r#"render()"#).unwrap();

        let head: String = result.get("head").unwrap();
        let body: String = result.get("body").unwrap();
        let mut map = HashMap::<String, String>::new();

        map.insert("body".to_string(), body);
        map.insert("head".to_string(), head);

        map
    });

    let template = read_to_string("ssr/dist/client/index.html").unwrap();

    let html = template
        .replace("<!--app-head-->", result.get("head").unwrap())
        .replace("<!--app-body-->", result.get("body").unwrap());

    html
}

#[get("/")]
async fn hello() -> impl Responder {
    let html = make_html();
    HttpResponse::Ok()
        .status(StatusCode::OK)
        .content_type("text/html")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/assets", "ssr/dist/client/assets"))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
