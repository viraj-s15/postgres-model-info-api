use crate::error_handler::CustomError;
use crate::ml_models::Model;
use crate::ml_models::Models;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json;

#[get("/dummy_models")]
async fn dummy_get_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Model {
            id: 123,
            project_name: "Stock Trend Prediction".to_string(),
            task_type: "classification".to_string(),
            quant_size: "fp16".to_string(),
            based_on: "tabnet".to_string(),
            is_llm: false,
            device_type: 2,
        },
        Model {
            id: 231,
            project_name: "Llama Mcq Solver".to_string(),
            task_type: "nlp".to_string(),
            quant_size: "fp16".to_string(),
            based_on: "llama2-7b".to_string(),
            is_llm: true,
            device_type: 2,
        },
    ])
}

#[get("/dummy_models/{id}")]
async fn dummy_get_by_index() -> impl Responder {
    HttpResponse::Ok().json(Model {
        id: 123,
        project_name: "Stock Trend Prediction".to_string(),
        task_type: "classification".to_string(),
        quant_size: "fp16".to_string(),
        based_on: "tabnet".to_string(),
        is_llm: false,
        device_type: 2,
    })
}

#[get("/models")]
async fn get_all() -> Result<HttpResponse, CustomError> {
    let employees = Models::find_all()?;
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/model/{id}")]
async fn get_model_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee = Models::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/models")]
async fn create(model: web::Json<Model>) -> Result<HttpResponse, CustomError> {
    let employee = Models::create(model.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/models/{id}")]
async fn update(
    id: web::Path<i32>,
    employee: web::Json<Models>,
) -> Result<HttpResponse, CustomError> {
    let employee = Models::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/models/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_employee = Models::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(dummy_get_all);
    config.service(dummy_get_by_index);
    config.service(get_all);
    config.service(get_model_by_id);
    config.service(create);
    config.service(update);
    config.service(delete);
}
