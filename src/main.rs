use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// Struct to represent a user
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Dummy user data
fn get_dummy_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Alice".into(), email: "alice@example.com".into() },
        User { id: 2, name: "Bob".into(), email: "bob@example.com".into() },
    ]
}

// GET /users - Returns all users
async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(get_dummy_users())
}

// GET /users/{id} - Fetch user by ID
async fn get_user_by_id(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    let users = get_dummy_users();

    if let Some(user) = users.into_iter().find(|u| u.id == user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

// POST /users - Create a new user (dummy response)
#[derive(Deserialize)]
struct NewUser {
    name: String,
    email: String,
}

async fn create_user(user: web::Json<NewUser>) -> impl Responder {
    let new_user = User {
        id: 3, // Dummy ID
        name: user.name.clone(),
        email: user.email.clone(),
    };

    HttpResponse::Created().json(new_user)
}

// DELETE /users/{id} - Delete a user (dummy response)
async fn delete_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("User with ID {} deleted", user_id))
}

// Main function to set up the Actix Web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}