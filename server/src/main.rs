mod model;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::{ Client, options::IndexOptions, IndexModel, options::ServerApi, options::ServerApiVersion, options::ClientOptions};
use actix_cors::Cors;
use model::{User, Restaurant, Review, default, default_user, default_restaurant};

//const MONGO_URL: &str = "mongodb+srv://manaspatil:Manas2003@rustcrud.o7kvyly.mongodb.net/?retryWrites=true&w=majority";
const DATABASE_NAME: &str = "rustcrud";
const USER_COLLECTION: &str = "users";
const RESTAURANT_COLLECTION: &str = "restaurant";
const REVIEW_COLLECTION: &str = "review";

#[post("/add_user")]
async fn add_user(user: web::Json<User>) -> HttpResponse{
    let mut client_options =
        ClientOptions::parse("mongodb+srv://manaspatil:Manas2003@rustcrud.o7kvyly.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DATABASE_NAME);
    let collection = db.collection(USER_COLLECTION);
    let result = collection.insert_one(user.into_inner(), None).await.unwrap();
    HttpResponse::Ok().body(format!("Inserted {} documents", result.inserted_id))
}

#[post("/add_restaurant")]
async fn add_restaurant(restaurant: web::Json<Restaurant>) -> HttpResponse {
    let mut client_options =
        ClientOptions::parse("mongodb+srv://manaspatil:Manas2003@rustcrud.o7kvyly.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DATABASE_NAME);
    let collection = db.collection(RESTAURANT_COLLECTION);
    let result = collection.insert_one(restaurant.into_inner(), None).await.unwrap();
    HttpResponse::Ok().body(format!("Inserted {} documents", result.inserted_id))
}

#[post("/add_review")]
async fn add_review(review: web::Json<Review>) -> HttpResponse {
    let mut client_options =
        ClientOptions::parse("mongodb+srv://manaspatil:Manas2003@rustcrud.o7kvyly.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DATABASE_NAME);
    let collection = db.collection(REVIEW_COLLECTION);
    let result = collection.insert_one(review.into_inner(), None).await.unwrap();
    HttpResponse::Ok().body(format!("Inserted {} documents", result.inserted_id))
}

#[get("/get_users/{username}")]
async fn get_users(username: web::Path<String>) -> HttpResponse {
    let mut client_options =
        ClientOptions::parse("mongodb+srv://manaspatil:Manas2003@rustcrud.o7kvyly.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DATABASE_NAME);
    let collection = db.collection(USER_COLLECTION);
    let username = username.into_inner();
    match collection.find_one(doc! {"username": username}, None).await.unwrap() {
        Some(document) => {
            let user: User = bson::from_bson(bson::Bson::Document(document)).unwrap();
            HttpResponse::Ok().json(user)
        }
        None => {
            HttpResponse::Ok().json(default_user())
        }
    }
}

#[get("/get_restaurants/{name}")]
async fn get_restaurants(name: web::Path<String>) -> HttpResponse {
    let mut client_options =
        ClientOptions::parse("mongodb+srv://manaspatil:Manas2003@rustcrud.o7kvyly.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DATABASE_NAME);
    let collection = db.collection(RESTAURANT_COLLECTION);
    let name = name.into_inner();
    match collection.find_one(doc! {"name": name}, None).await.unwrap() {
        Some(document) => {
            let restaurant: Restaurant = bson::from_bson(bson::Bson::Document(document)).unwrap();
            HttpResponse::Ok().json(restaurant)
        }
        None => {
            HttpResponse::Ok().json(default_restaurant())
        }
    }
}

#[get("/get_reviews/{restaurant}/{username}")]
async fn get_reviews(info: web::Path<(String, String)>) -> HttpResponse {
    let mut client_options =
        ClientOptions::parse("mongodb+srv://manaspatil:Manas2003@rustcrud.o7kvyly.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DATABASE_NAME);
    let collection = db.collection(REVIEW_COLLECTION);
    let (restaurant, username) = info.into_inner();
    match collection.find_one(doc! {"restaurant": restaurant, "username": username}, None).await.unwrap() {
        Some(document) => {
            let review: Review = bson::from_bson(bson::Bson::Document(document)).unwrap();
            HttpResponse::Ok().json(review)
        }
        None => {
            HttpResponse::Ok().json(default())
        }
    }
}

async fn create_user_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder().keys(doc! {"username": 1}).options(options).build();
    client.database(DATABASE_NAME).collection::<User>(USER_COLLECTION).create_index(model, None).await.unwrap();
}

async fn create_restaurant_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder().keys(doc! {"name": 1}).options(options).build();
    client.database(DATABASE_NAME).collection::<Restaurant>(RESTAURANT_COLLECTION).create_index(model, None).await.unwrap();
}

async fn create_review_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder().keys(doc! {"restaurant": 1}).options(options).build();
    client.database(DATABASE_NAME).collection::<Review>(REVIEW_COLLECTION).create_index(model, None).await.unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client_options =
        ClientOptions::parse("mongodb+srv://manaspatil:Manas2003@rustcrud.o7kvyly.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();
    create_user_index(&client).await;
    create_restaurant_index(&client).await;
    create_review_index(&client).await;

    client
        .database("rustcrud")
        .run_command(doc! {"ping": 1}, None)
        .await.expect("Failed to ping database.");
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(None);
        App::new()
            .wrap(cors)
            .service(add_user)
            .service(add_restaurant)
            .service(add_review)
            .service(get_users)
            .service(get_restaurants)
            .service(get_reviews)
    })
    .bind("127.0.0.1:5000")?
    .run().await
}

