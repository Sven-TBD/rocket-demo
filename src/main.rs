use rocket::{catch,catcher,delete,get,put,post,routes};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
    .launch().await?;
    Ok(())
}