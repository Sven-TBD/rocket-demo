use rocket::serde::json::{Value};
use rocket::serde::json::serde_json::json;
use rocket::{catch, catchers, delete, get, post, put, routes};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn get_products() -> Value{
    json!("list")
}

#[get("/<id>")]
async fn view_products(id:i32) -> Value {
    json!("get it!")
}

#[post("/")]
async fn create_products() -> Value {
    json!("create")
}

#[post("/<id>")]
async fn put_products(id:i32) -> Value {
    json!("put")
}

#[delete("/<id>")]
async fn delete_products(id:i32) -> Value {
    json!("delete")
}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("Not Found")
}

#[rocket::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    rocket::build().mount("/", routes![index])
        //产品方面需求
        .mount("/product", routes![
            get_products,
            view_products,
            create_products,
            put_products,
            delete_products])
        .register("/", catchers!(not_found_url))
    .launch().await?;
    Ok(())
}