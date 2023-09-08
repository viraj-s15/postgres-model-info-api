use crate::note::Note;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json;

#[get("/models")]
async fn get_all() -> impl Responder {
    HttpResponse::Ok().json(vec![Model {
        id:123,
        project_name: "Stock Trend Prediction".to_string(),
        tast_type= "classification".to_string(),
        quant_size:fp16,
        based_on:"tabnet",
        is_llm:false,
        device_type:2
    },
    Model {
        id:231,
        project_name: "Llama Mcq Solver".to_string(),
        tast_type= "nlp".to_string(),
        quant_size:fp16,
        based_on:"llama2-7b",
        is_llm:true,
        device_type:2
    }
    ])
}

#[get("/models/{id}")]
pub fn get_by_index() -> impl Responder {
    HttpResponse::Ok().json(Model {
        id:123,
        project_name: "Stock Trend Prediction".to_string(),
        tast_type= "classification".to_string(),
        quant_size:fp16,
        based_on:"tabnet",
        is_llm:false,
        device_type:2
    })
}

pub fn inint_routes(config:&mut web::ServiceConfig){
    config.service(get_all);
    config.service(get_by_index);
}
