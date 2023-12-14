#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

//!
//! CLIENTS
//! -------
//!
//! One of the main things that web servers do is talk to other web servers. This usually
//! takes the form of HTTP requests.
//!
//! In this section, you will see how you can "talk" to other web servers using Reqwest,
//! a popular HTTP client for Rust. Because Reqwest supports Tokio, it integrates
//! nicely into your Axum web applications.
//!

use axum::{
    body::Body,
    http::{Method, Request},
    response::Html,
    routing::*,
    Json, Router, extract::Path,
};

///
/// EXERCISE 1
///
/// In this exercise, you will make a web app that retrieves a random cat fact
/// from `https://catfact.ninja/fact` and displays it to the user in HTML.
///
/// In order to use Reqwest with json, you need to enable the `json` feature
/// (which is already enabled in this project). You then use one of the
/// reqwest methods, such as `reqwest::get`, to make a request. This
/// returns a future that can be awaited, and which returns a result
/// that may contain an error. If the result is successful, then using the
/// `json` feature, you can call the `json` method on the response to
/// deserialize the response into any type T that implements `serde::Deserialize`.
///
///
///
pub async fn cat_fact_server() {
    let app = Router::<()>::new().route("/", get(cat_fact_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
async fn cat_fact_handler() -> Html<String> {
    let cat_fact = reqwest::get("https://catfact.ninja/fact")
        .await 
        .unwrap()
        .json::<CatFact>()
        .await
        .unwrap();

    let html = format!("<h1>Random Cat Fact</h1><p>{}</p>", cat_fact.fact);

    Html(html)
}
#[derive(serde::Deserialize)]
struct CatFact {
    fact: String,
    length: u32,
}

///
/// EXERCISE 2
///
/// In this exercise, you will make a web app whose feature set is powered by
/// a third-party API. Namely, by JSONPlaceholder.
///
/// The URL root for JSONPlaceholder is `https://jsonplaceholder.typicode.com`.
///
/// The supported API endpoints are:
///
/// GET 	/posts
/// GET 	/posts/1
/// GET 	/posts/1/comments
/// GET 	/comments?postId=1
/// POST 	/posts
/// PUT 	/posts/1
/// PATCH 	/posts/1
/// DELETE 	/posts/1
///
/// Your job is to create an Axum web app that supports the following routes:
///
/// GET /posts
/// GET /posts/:id
/// GET /posts/:id/comments
/// POST /posts
/// PUT /posts/:id
/// DELETE /posts/:id
///
/// You have been provided with the structs Post and Comment, which you can use
/// to for interacting with the API using Reqwest.
///
/// You will have to use use a Reqwest client to make requests to the API.
/// One has been provided for you in the `posts_server` function. You can
/// set the body of a request using the `.body` method.`
///
async fn posts_server() {
    let app = Router::<()>::new()
        .route("/posts",                get(get_all_posts))
        .route("/posts/:id",            get(get_post_by_id))
        .route("/posts/:id/comments",   get(get_all_post_comments_by_id))
        .route("/posts",                post(create_post))
        .route("/posts/:id",            put(update_post_by_id))
        .route("/posts/:id",            delete(delete_post_by_id));

    let _client = reqwest::Client::new();
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
async fn get_all_posts() -> Json<Vec<Post>> {
    todo!()
}
async fn get_post_by_id(Path(id): Path<u32>) -> Json<Option<Post>> {
    todo!()
}
async fn get_all_post_comments_by_id(Path(id): Path<u32>) -> Json<Vec<Comment>> {
    todo!()
}
async fn create_post(post: Json<Post>) -> () {
    todo!()
}
async fn update_post_by_id(Path(id): Path<u32>, post: Json<Post>) -> () {
    todo!()
}
async fn delete_post_by_id(Path(id): Path<u32>) -> () {
    todo!()
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: u32,
    title: String,
    body: String,
    user_id: u32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Comment {
    post_id: u32,
    id: u32,
    name: String,
    email: String,
    body: String,
}

///
/// GRADUATION PROJECT
///
/// In this project, you will create a simple web app that needs to talk to
/// to any web server of your choosing. You should use Reqwest to make the
/// requests.
///
pub async fn graduation_project() {
    todo!("Create a web app that talks to a third-party web server of your choosing using Reqwest.")
}
