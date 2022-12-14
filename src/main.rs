use actix_web::{
    delete, error::Error, get, post, web::Path, App, HttpResponse,
    HttpServer, Responder
};
use backend_rs::{DataHandler, JSONHandler, TaskInput, BackendError};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("welcome!")
}

#[get("/tasks")]
async fn get_all_tasks() -> Result<HttpResponse, BackendError> {
    let json_filepath = "/Users/lcrown/temp/tasklist.json";
    let loader = JSONHandler::new(&json_filepath);
    let data = loader.load().unwrap();
    let resp_text = match serde_json::to_string(&data) {
        Ok(t) => t,
        Err(_) => "failed to parse json".to_owned(),
    };
    Ok(HttpResponse::Ok().body(resp_text))
}

#[get("/tasks/{id}")]
async fn get_task(path: Path<(u32,)>) -> Result<HttpResponse, BackendError> {
    let task_id = path.into_inner().0;
    let json_filepath = "/Users/lcrown/temp/tasklist.json";
    let loader = JSONHandler::new(&json_filepath);
    let task_list = loader.load().unwrap();
    let task = match task_list.get_by_id(task_id) {
        Some(t) => t,
        None => return Err(BackendError::BadClientData)
    };
    let resp_text = task.to_json().unwrap();
    Ok(HttpResponse::Ok().body(resp_text))
}

#[post("/tasks")]
async fn add_task(req_body: String) -> Result<HttpResponse, Error> {
    let json_filepath = "/Users/lcrown/temp/tasklist.json";
    let handler = JSONHandler::new(&json_filepath);
    let mut task_list = handler.load().unwrap();
    let new_task = serde_json::from_str::<TaskInput>(&req_body)?;
    task_list.add(new_task);
    handler.save(&task_list)?;
    Ok(HttpResponse::Ok().body(task_list.to_string().unwrap()))
}

#[delete("/tasks/{id}")]
async fn delete_task(path: Path<(u32,)>) -> Result<HttpResponse, Error> {
    let json_filepath = "/Users/lcrown/temp/tasklist.json";
    let handler = JSONHandler::new(&json_filepath);
    let mut task_list = handler.load().unwrap();
    let task_id = path.into_inner().0;
    println!("removing id: {}", task_id);
    task_list.remove_by_id(task_id);
    handler.save(&task_list).unwrap();
    Ok(HttpResponse::Ok().body(task_list.to_string().unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_all_tasks)
            .service(get_task)
            .service(add_task)
            .service(delete_task)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
