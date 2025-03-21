use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};

use tera::{Context, Tera};

#[get("/")]
pub async fn index(_req: HttpRequest, tera: Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    HttpResponse::Ok().body(tera.render("base.html", &ctx).unwrap())
}
