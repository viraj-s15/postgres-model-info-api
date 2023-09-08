use crate::db;
use crate::error_handler::CustomError;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::ml_models;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "ml_models"]
pub struct Model {
    pub project_name: String,
    pub task_type: String,
    pub quant_size: String,
    pub based_on: String,
    // which other model is it based on
    pub is_llm: bool,
    pub device_type: i8,
    // Which device is the model to be loaded on (cpu,gpu,tpu = 1,2,3)
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "ml_models"]
pub struct Models {
    pub id: i16,
    pub project_name: String,
    pub task_type: String,
    pub quant_size: String,
    pub based_on: String,
    // which other model is it based on
    pub is_llm: bool,
    pub device_type: i8,
    // Which device is the model to be loaded on (cpu,gpu,tpu = 1,2,3)
}

impl Models{
    pub fn find_all() -> Result<Vec, CustomError> {
        let conn = db::connection()?;
        let models = models::table.load::(&conn)?;
        Ok(models)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let model = models::table.filter(models::id.eq(id)).first(&conn)?;
        Ok(model)
    }
    pub fn create(model:Model) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let model = Models::from(model);
        let model = diesel::insert_into(models::table)
            .values(model)
            .get_result(&conn)?;
        Ok(model)
    }
    pub fn update(id: i32, model:Model) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let model = diesel::update(models::table)
            .filter(models::id.eq(id))
            .set(model)
            .get_result(&conn)?;
        Ok(model)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(models::table.filter(models::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}


impl Model{
    fn from(model: Model) -> Model{
        Model{
            project_name : model.project_name,
            task_type : model.task_type,
            quant_size : model.quant_size,
            based_on : model.based_on,
            is_llm : model.is_llm,
            device_type : model.device_type
        }
    }
}

