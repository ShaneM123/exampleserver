use crate::users::{User, Users};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::Pool;

/* #[get("/users")]

async fn find_all(    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    let conn = &pool.get().unwrap();
    let users = Users::find_all(conn)?;
    Ok(HttpResponse::Ok().json(users))
}
*/

#[get("/users/{id}")]
async fn find(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let conn = &pool.get().unwrap();
    let user = Users::find(conn,id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(pool: web::Data<Pool>, user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let conn = &pool.get().unwrap();

    let user = Users::create_user(conn, user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    user: web::Json<User>,
) -> Result<HttpResponse, CustomError> {
    let conn = &pool.get().unwrap();

    let user = Users::update(conn,id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(pool: web::Data<Pool>,id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let conn = &pool.get().unwrap();

    let deleted_user = Users::delete(conn,id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user
    })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    //config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}