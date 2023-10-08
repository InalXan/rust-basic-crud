use actix_web::{web, HttpResponse};
use serde::Serialize;
// chrono time pack
//use chrono::prelude::*;

#[derive(Serialize, Debug)]

pub struct Message {
    pub title: String,
    pub message: String,
}

pub async fn index() -> HttpResponse {
    let TITLE: &str = "Page";
    let MSG: &str = "Hello Customer You are at Home Page";
    let res_json = &Message {
        message: String::from(MSG),
        title: String::from(TITLE),
    };
    HttpResponse::Ok().json(res_json)
}

pub fn cfg_route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
}
