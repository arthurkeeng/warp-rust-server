
// mod db;
// mod  handlers;
// mod routes;
// mod models;
// use warp::Filter;

// #[tokio::main]
// async fn main() {
//     db::initialize_db();
//     let routes = routes::restaurant_routes();

//     println!("running the server");

//     warp::serve(routes.with(warp::trace::request()))
//     .run(([127 , 0 , 0,1] , 5000))
//     .await;

// }
// // fn main(){}
mod security;
mod todo_rest;
use warp::{Filter};
const WEB_FOLDER :&str= "web/";
#[tokio::main]

async fn main(){
    // this is to create a basic route
    let hi = warp::path("hi").and(warp::get()).and(warp::path::end()).map(||"hello from the hi page");
    let apis = hi;
    // this is to serve static files
    let content = warp::fs::dir("web/");
    let root = warp::get().and(warp::path::end()).and(warp::fs::file(format!("{}/index.html",WEB_FOLDER)));

    let static_site = content.or(root);
    
    let routes = apis.or(static_site).or(todo_rest::todos_filter());
    // this helps to compose routes together. It filters through each declaration.

    println!("server started");
    warp::serve(routes).run(([127, 0 ,0 ,1] , 5000)).await;
}