use yew::prelude::*;
use crate::model::{User, Restaurant, Review};
use gloo_net::http::Request;
use wasm_bindgen_futures::JsFuture;

const MONGO_URL: &str = "http://localhost:8000";
