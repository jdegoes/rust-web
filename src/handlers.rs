#![allow(dead_code)]
#![allow(unreachable_code)]

//!
//! HANDLERS
//! --------
//!
//! In Axum, handlers are the building block of web servers. They provide the
//! implementation of the functionality of every route.
//!
//! Handlers may seem magical to new users of Axum, which can lead to surprises
//! when a given Rust function fails to satisfy the requirements of the
//! handler.
//!
//! In this section, you will embark on a comprehensive tour of handlers,
//! exploring all major ways to create them, learning more about their required
//! (and optional) structure, and discovering how to diagnose troubles with
//! their types. You will also see more details about how handlers relate to
//! and interact with paths in a route definition.
//!

#[allow(unused_imports)]
use axum::{body::Body, extract::*, http::Method, routing::*, Json};
use hyper::{Request, StatusCode};
use std::collections::HashMap;

///
/// EXERCISE 1
///
/// The most fundamental type your handlers may take is the type `Request<Body>`.
/// The `Request` type is a struct that contains all of the information about
/// the incoming request, including the HTTP method, the headers, and the body.
///
/// In this exercise, you will create a handler that takes a `Request<Body>` as
/// an argument and returns a `String` as a response. The `String` should be
/// the body of the request.
///
/// Although we will cover this in more depth soon, for now, just note that the
/// return value of the handler is being used as the body of the response.
///
#[tokio::test]
async fn basic_request_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users", get(basic_request_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users")
                .body(Body::from("<h1>Hello!</h1>"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "<h1>Hello!</h1>");
}

async fn basic_request_handler(request: Request<Body>) -> String {
    use http_body_util::BodyExt;
    let body = request
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    String::from_utf8(body).unwrap()
}

///
/// EXERCISE 2
///
/// A handler may accept a `String` as an argument. See if you can discover what part
/// of the request the `String` might come from by adding a succeeding `assert_eq!`
/// assertion to the following test.
///
#[tokio::test]
async fn string_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users", get(string_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users")
                .body(Body::from("<h1>Hello!</h1>"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "<h1>Hello!</h1>");
}

async fn string_handler(string: String) -> String {
    string
}

#[tokio::test]
async fn string_query_param_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users", get(request_to_string_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users?test=abc")
                .body(Body::from("<h1>Hello!</h1>"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "abc");
}

async fn request_to_string_handler(_request: Request<Body>) -> String {
    todo!("Return the query parameter `test` as a string")
}

///
/// EXERCISE 3
///
/// A handler may accept a `hyper::body::Bytes` as an argument. See if you can discover
/// what part of the request the `Bytes` might come from by adding a succeeding `assert_eq!`
/// assertion to the following test.
///
#[tokio::test]
async fn bytes_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users", get(bytes_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users")
                .body(Body::from("<h1>Hello!</h1>"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(body, "<h1>Hello!</h1>".as_bytes());
}

async fn bytes_handler(bytes: hyper::body::Bytes) -> hyper::body::Bytes {
    bytes
}

///
/// EXERCISE 4
///
/// A handler may accept a `axum::Json<A>` for any type `A` that has an implementation of
/// the `serde::Deserialize` trait. Create a `Person` data structure with a single field
/// `name` of type `String` and implement `serde::Deserialize` for it. Then, modify the
/// handler `json_handler` to return the name of the person.
///
#[tokio::test]
async fn json_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users/jdoe", get(json_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users/jdoe")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name": "John Doe"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "John Doe");
}

async fn json_handler(Json(Person { name }): Json<Person>) -> String {
    name
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Person {
    name: String,
}

///
/// EXERCISE 5
///
/// A handler may also accept something of type `Path<A>`, for any type `A` that has an
/// implementation of the `serde::Deserialize`. Axum will automatically deserialize the
/// path segment variables into the type `A` (if possible), and pass them into the
/// handler.
///
/// In this exercise, change the route to include a path segment variable `name`,
/// using the notation `:name`. Then, modify the handler `path_handler` to return the
/// name of the person.
///
#[tokio::test]
async fn path_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users/:name/:age", get(path_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users/jdoe/45")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "jdoe45");
}

async fn path_handler(
    axum::extract::Path((name, age)): axum::extract::Path<(String, u8)>,
) -> String {
    name + &age.to_string()
}

///
/// EXERCISE 6
///
/// Many route patterns have more than one variable. You might think that in order to
/// handle these routes, you would need to create a handler with multiple `Path<A>`
/// parameters. However, this will not work, because the mechanism by which the `Path`
/// extractor works expects to be able to extract a value for each path segment variable
/// in one go. Instead of using multiple path parameters, however, you can achieve the
/// same effect by using a tuple (or a struct).
///
/// In this exercise, change the broken handler to use either a tuple or a struct, and
/// ensure the test case passes.
///
#[tokio::test]
async fn path2_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users/:name/posts/:post_id", get(path2_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users/jdoe/posts/1")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "jdoe:1");
}

async fn path2_handler(
    axum::extract::Path((name, post_id)): axum::extract::Path<(String, u32)>,
) -> String {
    format!("{}:{}", name, post_id)
}

///
/// EXERCISE 7
///
/// A handler may also accept something of type `axum::extract::Query<A>`, for any type
/// `A` that has an implementation of the `serde::Deserialize`. Axum will automatically
/// deserialize the query string variables into the type `A` (if possible), and pass
/// them into the handler.
///
/// A common type to use for `A` is `HashMap<String, String>`, which will deserialize
/// the query string into a map of key-value pairs.
///
/// In this exercise, change the handler to capture and return the query parameters.
///
#[tokio::test]
async fn query_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users", get(query_handler2));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users?name=jdoe&age=42")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "name=jdoe&age=42");
}

async fn query_handler(
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> String {
    format!("name={}&age={}", query["name"], query["age"])
}

async fn query_handler2(
    axum::extract::Query(QueryParams {
        name,
        age, /* , limit*/
    }): axum::extract::Query<QueryParams>,
) -> String {
    format!("name={}&age={}", name, age.unwrap_or(12))
}

#[derive(serde::Deserialize)]
struct QueryParams {
    name: String,
    age: Option<u8>,
    // limit: Option<u8>,
}

///
/// EXERCISE 8
///
/// A handler may also accept `axum::http::header::HeaderMap` as a parameter. This
/// allows you to access the headers of the request.
///
/// In this exercise, change the handler to capture and return the `Content-Type` header.
///
#[tokio::test]
async fn header_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users", get(header_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "application/json");
}
async fn header_handler(headers: axum::http::HeaderMap) -> String {
    headers
        .get("Content-Type")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

///
/// EXERCISE 9
///
/// Unlike the examples seen so far, handlers may accept *multiple* parameters, which
/// Axum will automatically extract from the request.
///
/// In this exercise, change the handler to capture and return the `limit` query
/// parameter and the path segment variable `name`.
///
#[tokio::test]
async fn multiple_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users/:name/posts", get(multiple_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users/jdoe/posts?limit=10")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "jdoe:10");
}

async fn multiple_handler(
    Query(map): Query<HashMap<String, String>>,
    Path(name): Path<String>,
) -> String {
    format!("{}:{}", name, map["limit"])
}

///
/// EXERCISE 10
///
/// So far, we have seen how Axum handlers can accept a variety of types as parameters. Yet,
/// we have not seen exactly what types of return values are supported, nor exactly how they
/// are mapped into responses.
///
/// In this exercise, change the handler to return a `hyper::Response<Body>`, which you should
/// construct in such a fashion as to pass the unit test. The low-level Response type consists
/// of both parts (which include headers, status code, etc.) and a body, allowing you to specify
/// all possible information you want to include in the response.
///
/// Note that to construct a Response, you will be using `Response::builder()`, which is
/// is a builder that allows you to gradually specify the details of the response.
///
#[tokio::test]
async fn response_handler_test() {
    /// for StatusCode
    use axum::http::StatusCode;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users", get(response_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "text/plain"
    );
}

async fn response_handler() -> hyper::Response<Body> {
    #![allow(unused_imports)]
    use hyper::Response;

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::from(""))
        .unwrap()
}

///
/// EXERCISE 11
///
/// Your handlers may return a `Body`, in which case this body will be used as the body
/// of the response.
///
/// In this exercise, change the handler to return a `Body`, which contains the static
/// string `Hello, world!`.
///
#[tokio::test]
async fn body_handler_test() {
    /// for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/", get(body_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "Hello, world!");
}

async fn body_handler() -> Body {
    Body::from("Hello, world!")
}

///
/// EXERCISE 12
///
/// Your handlers may return `Json<A>` for any type `A` that has an implementation of
/// the `serde::Serialize` trait. This will automatically serialize the value into JSON
/// and use it as the body of the response.
///
/// In this exercise, change the handler to return a `Json<A>` value for some type A
/// that you design and derive a serializer for.
///
#[tokio::test]
async fn json_response_handler_test() {
    /// for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/", get(json_response_handler2));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, r#"{"name":"John Doe"}"#);
}

async fn json_response_handler() -> Json<Person> {
    Json(Person {
        name: "John Doe".to_string(),
    })
}

use serde_json::json;
use serde_json::Value as JsonValue;
async fn json_response_handler2() -> Json<JsonValue> {
    Json(json!({"name": "John Doe" }))
}

///
/// EXERCISE 13
///
/// In Axum, handlers may seem like magic, but now it is time to learn how they are
/// implemented.
///
/// Technically, a handler is any data type that implements `axum::handler::Handler`.
/// This has a single required method, `call`, which takes a `Request` and returns a
/// future of `Response`.
///
/// Axum provides implementations of this trait for functions up to arity 16, so
/// long as the input types of the function implement `FromRequest` (or
/// `FromRequestParts`), and the return type implements `IntoResponse`.
///
/// In this exercise, make your own custom data type for the handler's input and output,
/// and then implement the traits `FromRequest` and `IntoResponse` for it.
/// Fix the test to ensure it is passing for whatever data types you have chosen.
///
#[tokio::test]
async fn handler_trait_test() {
    /// for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/", get(handler_trait_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, r#"{"name":"John Doe"}"#);
}
async fn handler_trait_handler() -> () {
    todo!("Return a custom data type for which you provide an implementation of IntoResponse")
}

///
/// GRADUATION PROJECT
///
/// Provide a complete implementation of the following API, which uses dummy data.
///
/// GET /users
/// GET /users/:id
/// POST /users
/// PUT /users/:id
/// DELETE /users/:id
///
/// Place it into a web server and test to ensure it meets your requirements.
///
async fn run_users_server() {
    todo!("Implement the users API")
}
