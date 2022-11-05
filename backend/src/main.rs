use db::*;
use std::sync::Arc;
use web::filters::*;
use dotenv::dotenv;
use warp::{
    http::{header, Method},
    Filter,
};

mod db;
mod web;
mod security;
mod error;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    let db = init_db().await?;
    let db = Arc::new(db);
    
    dotenv().ok();
    
    println!("App is running ...");
    
    let apis = user_filters("api", db)
            .with(warp::cors()
            .allow_credentials(true)
            .allow_methods(&[
                Method::OPTIONS,
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::PUT,
            ])
            .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .expose_headers(vec![header::LINK])
            .allow_any_origin())
        .recover(error::handler::error_handler);

    warp::serve(apis).run(([127,0,0,1], 7000)).await;
    Ok(())
}
