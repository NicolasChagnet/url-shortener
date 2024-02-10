use actix_web::{get, post, HttpResponse, Responder, web};
use mongodb::Client;
use crate::{
    get_url_origin_db, html_body, set_url_db,
    model::UrlTo
};

#[get("/")]
pub async fn get_root() -> impl Responder {
    HttpResponse::Ok().body(html_body::IDX_BDY)
}

#[post("/add")]
pub async fn add_url(client: web::Data<Client>, form: web::Form<UrlTo>) -> impl Responder {
    let to = form.into_inner().to;
    println!("{}", &to);
    let result = set_url_db(&client, &to).await;
    match result {
        Some(from) => {
            println!("{}", &from);
            HttpResponse::Ok().body(html_body::add_bdy(&from))
        },
        None => HttpResponse::InternalServerError().body("Internal error.")
    }
}

#[get("/{from}")]
pub async fn get_url(client: web::Data<Client>, from: web::Path<String>) -> impl Responder {
    let from = from.into_inner();
    let url_req = get_url_origin_db(&client, &from).await;
    match url_req {
        Some(url_obj) => web::Redirect::to(url_obj.get_to().to_string()), 
        None => web::Redirect::to("/")
    }
    
}
