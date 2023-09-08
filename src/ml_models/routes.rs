use crate::ml_models::Model;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json;

#[get("/models")]
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

#[get("/models/{id}")]
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

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(dummy_get_all);
    config.service(dummy_get_by_index);
}
