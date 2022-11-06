
    use std::borrow::{Cow, Borrow};

    use actix_web::{get, HttpResponse};

    pub struct GetCow<'a> {
        pub name: Cow<'a, str>
    }

fn do_nothing(){
    let x = GetCow{name: std::borrow::Cow::Borrowed("Hello")};
}

fn do_1thing(x: Cow<str>){
    let value = x;
    println!("{:?}", value);
}

    #[get("/name")]
    pub async fn get_user_name() -> HttpResponse{
        HttpResponse::Ok().body(format!("dummy value"))
    }


