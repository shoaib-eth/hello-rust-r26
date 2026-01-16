use rocket::*;

#[get("/home/<name>")]
fn hello_user(name: String) -> String {
    format!("Hello 👋 {}, Have a Nice Day 🙋🏻", name)
}
#[launch]

fn rocket() ->_ {
    rocket::build().mount("/", routes![hello_user])
}

