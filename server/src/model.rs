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

pub fn default() -> Review {
    Review {
        restaurant: String::from("No review found"),
        user: String::from(""),
        rating: String::from(""),
        comment: String::from(""),
    }
}

pub fn default_user() -> User {
    User {
        username: String::from("No user found"),
        password: String::from(""),
        restaurants: Vec::new(),
    }
}

pub fn default_restaurant() -> Restaurant {
    Restaurant {
        name: String::from("No restaurant found"),
        address: String::from(""),
        email: String::from(""),
        description: String::from(""),
        rating: String::from(""),
        price: String::from(""),
    }
}
