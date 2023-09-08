table! {
    ml_models (id) {
        id -> Int8,
        project_name -> Varchar,
        task_type -> Varchar,
        quant_size -> Varchar,
        based_on -> Varchar,
        is_llm -> Boolean,
        device_type -> Int4,
    }
}
