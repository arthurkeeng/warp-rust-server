use serde_json::{json, Value};
use warp::{reply::Json, Filter};

use crate::security::{check_auth, UserCtx};

pub fn todos_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todo_base_url = warp::path("todos");
    let list = todo_base_url
        .and(warp::get())
        .and(warp::path::end())
        .and(check_auth())
        .and_then(todo_list);
    // .and_then(|| async{Ok::<&str , warp::Rejection>("from the todos")})

    let get = todo_base_url
        .and(warp::get())
        .and(check_auth())
        .and(warp::path::param())
        .and_then(todo_get);
    let create = todo_base_url
        .and(warp::post())
        .and(check_auth())
        .and(warp::body::json())
        .and_then(todo_create);
    list.or(get).or(create)
}

async fn todo_list(_user_ctx : UserCtx ) -> Result<Json, warp::Rejection> {
    let todos = json!([
        {"id" : "1", "title" : "todo 1"},
        {"id" : "2" , "title" : "todo 2"}
    ]);
    let todos = warp::reply::json(&todos);
    Ok(todos)
}

async fn todo_get(_user_ctx : UserCtx , id: i64) -> Result<Json, warp::Rejection> {
    // get the todo with the id
    let todo = json!({
        "id" : id ,"user_id" : _user_ctx.user_id, "title" : format!("todo {}" , id)
    });
    let todo = warp::reply::json(&todo);
    Ok(todo)
}

async fn todo_create(_user_ctx : UserCtx , data: Value) -> Result<Json, warp::Rejection> {
    //    writing todo
    let todo = data;
    let todo = warp::reply::json(&todo);
    Ok(todo)
}
