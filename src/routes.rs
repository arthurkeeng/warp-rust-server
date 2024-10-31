use warp::{reject::Rejection, reply::Reply, Filter};
use rusqlite::Connection;
use crate::db::get_db_conn;
use std::{clone, convert::Infallible};
use crate::handlers::{
    create_order_handler, 
    list_table_handler,
    create_table_handler,
    list_menu_handler,
    create_menu_handler,
    list_order_handler,
    delete_order_item_handler,
    list_order_items_for_table_handler, 
    get_order_item_for_table_handler
}
async fn handle_rejection(err:Rejection)-> Result<impl Reply , Rejection>{
    if err.is_not_found(){
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: {:?}" , err)),
            warp::http::StatusCode::NOT_FOUND
        ))
    }
    else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>(){
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: failed to deserialize request body")),
            warp::http::StatusCode::BAD_REQUEST
        ))
    }else{
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: {:?}", err)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR
        ))
    }
}

fn with_db()-> impl Filter<Extract = Connection , Error = Infallible> + Clone{
    warp::any().map(||get_db_conn())
}
pub fn list_all_orders_route(){}
pub fn create_order_route(){}
pub fn delete_item_from_order_route(){}
pub fn list_tables_route(){}
pub fn create_table_route(){}
pub fn list_order_items_for_table_route(){}
pub fn get_item_from_order_route(){}
pub fn list_menu_routes(){}
pub fn create_menu_route(){}


pub fn restaurant_routes() -> impl Filter<Extract = impl Reply , Error = Rejection> + Clone{
    let routes = create_order_route()
    .or(create_table_route())
    .or(create_menu_route())
    .or(list_tables_route())
    .or(list_menu_routes())
    .or(list_all_orders_route())
    .or(delete_item_from_order_route())
    .or(list_order_items_for_table_route())
    .or(get_item_from_order_route());

    routes.recover(handle_rejection)
}