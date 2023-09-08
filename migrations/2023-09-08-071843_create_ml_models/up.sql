-- Your SQL goes here
CREATE TABLE "ml-models"
(
  id SERIAL PRIMARY KEY,
  project_name VARCHAR NOT NULL,
  task_type VARCHAR NOT NULL,
  quant_size VARCHAR NOT NULL,
  based_on VARCHAR NOT NULL,
  is_llm BOOLEAN NOT NULL,
  device_type NUMBER NOT NULL
)
