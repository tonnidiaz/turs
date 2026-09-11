// pub mod io;

use std::{collections::HashMap, env, time::Instant};

use actix_cors::Cors;
use actix_web::{
    App, Error, HttpServer,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{Next, from_fn},
};
use turs::log;

use crate::routes::{self};

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
        log!("{_method} {_status} {_url}  {:.2?}", _instant.elapsed());
        res
    })
}

pub struct Server {}

impl Server {
    pub async fn new(router: Scope, port: i64) -> std::io::Result<()> {
        crate::log!("Starting server");
        let bind = ("0.0.0.0", port);
        crate::log!("Listening on: {}:{}", bind.0, bind.1);

        HttpServer::new(|| {
            let cors = Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allow_any_header()
                // .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                .max_age(3600);
            App::new().wrap(cors).wrap(from_fn(logger)).service(router)
        })
        .bind(bind)?
        .run()
        .await
    }
}
 