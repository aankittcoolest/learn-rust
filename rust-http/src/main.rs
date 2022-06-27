use actix_web::{App, HttpServer};
mod views;
mod processes;
mod state;
mod to_do;
mod json_serialization;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}


// use processes::process_input;
// use serde_json::value::Value;
// use serde_json::Map;
// use state::read_file;
// use std::env;
// use to_do::to_to_factory;

// fn main() {
//     // let to_do_item: Result<ItemTypes, &'static str> = to_to_factory("pending", "washing");

//     // match to_do_item.unwrap() {
//     //     ItemTypes::Pending(item) => item.create(&item.super_struct.title),
//     //     ItemTypes::Done(item) => println!("It's a done item with the title: {}", item.super_struct.title)
//     // }

//     let args: Vec<String> = env::args().collect();
//     let command: &String = &args[1];
//     let title: &String = &args[2];

//     let state: Map<String, Value> = read_file("./state.json");

//     println!("{:?}", state);

//     let status: String;
//     match &state.get(*&title) {
//         Some(result) => {
//             status = result.to_string().replace('\"', "");
//         }
//         None => {
//             status = "pending".to_string();
//         }
//     }

//     let item = to_to_factory(&status, title).expect(&status);
//     process_input(item, command.to_string(), &state);
// }
