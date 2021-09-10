use rocket::serde::json::serde_json::json;
use rocket::{State, catch, catchers, delete, get, post, put, routes};
use std::error::Error;
use rocket::serde::json::{self, Json, Value};
use rocket::serde::{Serialize,Deserialize};
use rocket::tokio::sync::Mutex;
use std::collections::HashMap;

//map -> mutex ->state
type PersonItem = Mutex<HashMap<usize,Person>>;
type Message<'r> = &'r State<PersonItem>;




//请求体的结构体
//model database 的结构体
// response 结构体
#[derive(Serialize,Deserialize,Clone)]
//下面这里用于指定Serialize库用哪一个
#[serde(crate = "rocket::serde")]
struct Task{
    id : usize,
    name: String
}

#[derive(Serialize,Deserialize,Clone)]
//下面这里用于指定Serialize库用哪一个
#[serde(crate = "rocket::serde")]
struct Person{
    id : usize,
    name: String
}

#[get("/person/<id>")]
async fn get_person(id: usize,message:Message<'_>) -> Json<Person>{
    let person_map = message.lock().await;
    if id ==0{
        return Json(Person{
                id:0,
                name:"None".to_string(),
            });
    }
    match person_map.get(&id){
        None => Json(Person{
            id:0, 
            name:"".to_string(),
        }),
        Some(p)=>Json(p.to_owned()),
    }
}


#[post("/person",format="json",data="<person>")]
async fn create_person(person:Json<Person>,message:Message<'_>) -> Value{ 
    let mut person_map = message.lock().await;
    let new_person = person.into_inner();
    if person_map.contains_key(&new_person.id){
        json!({"res":"err"})
    }else {
        person_map.insert(new_person.id,new_person);
        json!({"res":"ok"})
    }
}

#[put("/person/<id>",format = "json",data ="<person>")]
async fn put_person(id:usize,person:Json<Person>,message:Message<'_>)->Value {
    let mut person_map = message.lock().await;
    let new_person = person.into_inner();
    if id != new_person.id{
        return(json!({"res":"err"}))
    }
    if person_map.contains_key(&new_person.id){
        person_map.insert(new_person.id,new_person);
        json!({"res":"ok"})
    }else{
        json!({"res":"err"})
    }

}

#[delete("/person/<id>")]
async fn delete_person(id:usize,message:Message<'_>) -> Value{
    let mut person_map = message.lock().await;

    if person_map.contains_key(&id){
        person_map.remove(&id);
        json!({"res":"ok"})
    }else {
        json!({"res":"dont exist"})
    }
}

#[get("/")]
async fn hello()->Option<Json<Task>> {
    Some(
        Json(Task {
            id: 0,
            name: "Hello".to_string(),
        })
    )
}

//restful
//get
#[get("/ex")]
async fn get_list_ex()->Value {
    json!({"res":"ex"})
}

#[get("/ex/<_id>")]
async fn get_id_ex(_id:usize)->Value {
    json!({"res":"ex"})
}

//post
// #[post("/ex")]
// async fn post_ex()->Value {
//     json!({"res":"ex"})
// }

//下面是通过请求体里获取json
//format请求体格式
//data
#[post("/ex",format="json",data = "<task>")]
async fn post_ex(task:Json<Task>)->Value {
    let task = task.into_inner();
    json!({"res":format!("{} {}",task.id,task.name)})
}


//put
#[put("/ex/<id>")]
async fn put_ex(id:usize)->Value {
    json!({"res":"ex"})
}

#[delete("/ex/<id>")]
async fn delete_ex(id:usize)->Value {
    json!({"res":"ex"})
}

#[catch(404)]
async fn not_rust()->Value {
    json!("404")
}

#[catch(404)]
async fn not_base()->Value {
    json!("404")
}

#[rocket::main]
async fn main() -> Result<(),Box<dyn Error>> {
    rocket::build()
        //route
        .mount("/hello",routes![hello])
        .mount("/base",routes![get_list_ex,get_id_ex,post_ex,put_ex,delete_ex])
        .mount("/second",routes![get_list_ex,get_id_ex,post_ex,put_ex,delete_ex])
        //route person
        .manage(PersonItem::new(HashMap::new()))
        .mount("/person_things",routes![get_person,create_person,delete_person])

        //catch
        .register("/", catchers!(not_rust))
        .register("/base", catchers!(not_base))
        //
        .launch().await?;
    Ok(())
}