use serde::{Deseralize, Serialize};

#[derive(Debug)]
enum quantSizeType {
    int4,
    int8,
    fp16,
    int16,
}

#[derive(Deseralize, Serialize)]
pub struct Model {
    pub id: i16,
    pub project_name: String,
    pub task_type: String,
    pub quant_size: quantSizeType,
    pub based_on: String,
    // which other model is it based on
    pub is_llm: bool,
    pub device_type: i8,
    // Which device is the model to be loaded on (cpu,gpu,tpu = 1,2,3)
}
