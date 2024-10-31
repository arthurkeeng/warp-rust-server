
use warp::Filter;

const HEADER_XAUTH : &str = "x-Auth-Token";

pub fn check_auth() -> impl Filter<Extract = ((UserCtx),) , Error = warp::Rejection> + Clone{
    warp::any()
    .and(warp::header::<String>(HEADER_XAUTH))
    .and_then(|xauth : String| async move{
        if !xauth.ends_with(".expsignature"){
            return Err(warp::reject::custom(FailedAuth))
        }
        if let Some(user_id) = xauth.splitn(2, ".")
        .next().
        and_then(|v|v.parse::<i64>().ok()){
            Ok::<UserCtx , warp::Rejection>(UserCtx{user_id})
        }else{
            Err(warp::reject::custom(FailedAuth))
        }
      
        })
}
pub struct UserCtx{
    pub user_id : i64
}
#[derive(Debug)]
pub struct FailedAuth;
impl warp::reject::Reject for FailedAuth{

}