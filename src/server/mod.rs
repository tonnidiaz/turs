// pub mod io;

use std::{sync::Arc, time::Instant};

use actix_cors::Cors;
use actix_web::{
    App, Error, HttpResponse, HttpServer, Scope,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::{Next, from_fn},
};
use serde::Serialize;
use serde_json::json;

use crate::log;

async fn logger(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let _instant = Instant::now();
    let _method = req.method().as_str().to_uppercase();
    let uri = req.uri().clone();
    let _url = uri
        .path_and_query()
        .and_then(|q| Some(q.as_str()))
        .unwrap_or_else(|| "");
    next.call(req).await.map(|res| {
        let _status = res.status().as_u16();
        // log!("{_method} {_status} {_url}  {:.2?}", _instant.elapsed());
        res
    })
}
pub struct Server {}

impl Server {
    pub async fn new(router: fn() -> Scope, port: u16) -> std::io::Result<()> {
        log!("Starting server");
        let bind = ("0.0.0.0", port);
        log!("Listening on: {}:{}", bind.0, bind.1);

        HttpServer::new(move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allow_any_header()
                // .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                .max_age(3600);
            App::new()
                .wrap(cors)
                .wrap(from_fn(logger))
                .service(router())
        })
        .bind(bind)?
        .run()
        .await
    }
}

pub fn tapi_err(status: u16, msg: &str) -> HttpResponse {
    HttpResponse::build(StatusCode::from_u16(status).unwrap())
        .json(json!({"message": format!("{}", msg)}))
}

pub fn tapi_json<T: Serialize>(body: T) -> HttpResponse {
    HttpResponse::Ok().json(body)
}
