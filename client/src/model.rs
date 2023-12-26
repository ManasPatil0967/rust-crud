use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
    restaurants: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Restaurant {
    name: String,
    address: String,
    email: String,
    description: String,
    rating: String,
    price: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Review {
    restaurant: String,
    user: String,
    rating: String,
    comment: String,
}
