use actix_web::{post, web, HttpRequest, HttpResponse, Result};
use jsonwebtoken::{encode, get_current_timestamp, EncodingKey, Header};
use mongodb::{bson::doc, Client, Collection};

use crate::models;
use crate::utils;
use models::claim::Claims;
use models::user::LoginUser;
use models::user::LoginUserResponse;
use models::user::User;
use models::user::UserDate;
use utils::hash;

macro_rules! trace {
    ($($args: expr),*) => {
        print!("TRACE: file: {}, line: {}", file!(), line!());
        $(
            print!(", {}: {}", stringify!($args), $args);
        )*
        println!(""); // to get a new line at the end
    }
}

#[post("/login")]
async fn login(req: HttpRequest, client: web::Data<Client>, data: web::Json<User>) -> HttpResponse {
    let users: Collection<User> = client.database("rust").collection("users");
    let user_data = users.find_one(doc! { "email": &data.email }, None).await;

    let result: Result<_, &str> = match user_data {
        Ok(v) => {
            match v {
                Some(user_data) => {
                    if hash::verify_hash(
                        String::from(&user_data.password),
                        String::from(&data.password),
                    ) {
                        let my_claims = Claims {
                            aud: std::env::var("AUDIENCE")
                                .unwrap_or_else(|_| "some_default_idk".into()),
                            exp: get_current_timestamp() + 3600 * 24 * 7, //expires in a week
                            iat: get_current_timestamp(),
                            sub: user_data.email.clone(),
                            user_id: user_data.id.to_string().chars().take(24).collect(),
                        };
                        let key = std::env::var("JWT_SECRET")
                            .unwrap_or_else(|_| "random_bullshit_go".into());
                        let token = encode(
                            &Header::default(),
                            &my_claims,
                            &EncodingKey::from_secret(key.as_bytes()),
                        )
                        .unwrap();
                        let user = UserDate {
                            email: user_data.email,
                            username: user_data.username,
                        };
                        Ok(LoginUserResponse {
                            user_data: user,
                            token: token,
                        })
                    } else {
                        Err("bad password error")
                    }
                }
                None => {
                    Err("email doesnt exist")
                }
            }
        }
        Err(_e) => {
            Err("database error")
        }
    };

    match result {
        Ok(value) => {
            HttpResponse::Ok().json(value)
        }
        Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
    }
}

#[post("/register")]
async fn register(client: web::Data<Client>, data: web::Json<User>) -> HttpResponse {
    let users: Collection<User> = client.database("rust").collection("users");

    let user = data.clone();
    let mut user_data = data;
    let user = users.find_one(doc! { "email": user.email }, None).await;

    let result: Result<_, &str> = match user {
        Ok(v) => match v {
            Some(..) => Err("email taken"),
            None => {
                user_data.password = hash::hash(user_data.clone().password);
                Ok(users.insert_one(user_data.clone(), None).await)
            }
        },
        Err(..) => {
            Err("database error")
        }
    };

    match result {
        Ok(value) => {
            HttpResponse::Ok().body("accound created")
        }
        Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
    }
}
