#![allow(dead_code)]
#![allow(unreachable_code)]

//!
//! BASICS
//! ------
//!
//! Axum is a web application framework that focuses on ergonomics, modularity,
//! and performance.
//!
//! In this section, you will learn the basics of building web applications
//! using the Axum framework. Although many of the specifics that you learn
//! will be Axum-specific, the concepts that you learn will be applicable to
//! other web frameworks as well.
//!  

#[allow(unused_imports)]
use axum::{
    body::Body,
    http::{Method, Request},
    response::Html,
    routing::*,
    Json, Router,
};

#[cfg(test)]
use hyper::StatusCode;

///
/// In this "hello world" example, you can see the core elements of an Axum
/// web application:
///
/// 1. A router, which is used for specifying routes.
/// 2. A single route, defined with a path and a handler.
/// 3. A handler, which is an asynchronous function that returns a response.
/// 4. A listener, which is used to listen for incoming connections.
/// 5. A call to `axum::serve`, which starts the server.
///
pub async fn hello_world() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

///
/// EXERCISE 1
///
/// Use the `Html` constructor to construct an HTML response that includes the
/// text "Hello, World!".
///
/// Run the hello_world() function to ensure that you can browse the `/` route
/// and that it properly serves the static HTML.
///
async fn handler() -> Html<&'static str> {
    Html("Hello world! handler test")
}

///
/// EXERCISE 2
///
/// Add the following routes to `_router``, using the dummy_handler for each.
///
/// GET /users/
/// GET /users/:id
/// POST /users/
/// PUT /users/:id
/// DELETE /users/:id
///
fn build_router<S: Clone + Send + Sync + 'static>(_router: Router<S>) -> Router<S> {
    _router
        .route("/users", get(dummy_handler))
        .route("/users/:id", get(dummy_handler))
        .route("/users", post(dummy_handler))
        .route("/users/:id", put(dummy_handler))
        .route("/users/:id", delete(dummy_handler))
}

async fn dummy_handler() -> Html<&'static str> {
    Html("<h1>Dummy Handler</h1>")
}

///
/// EXERCISE 2
///
/// Using Router::merge, combine two routers into one.
///
/// What are the semantics of the resulting router?
///
fn merge_routers<S: Clone + Send + Sync + 'static>(left: Router<S>, right: Router<S>) -> Router<S> {
    left.merge(right)
}

///
/// EXERCISE 3
///
/// To factor out duplication across route paths, you can use the `nest` method
/// on Router. This method takes a path prefix and a router, and returns a new
/// router that has the path prefix applied to all of the routes in the nested
/// router.
///
/// In the following example, use the `nest` method to nest all of the user
/// routes under the `/users` path prefix of the specified router.
///
fn nest_router<S: Clone + Send + Sync + 'static>(_router: Router<S>) -> Router<S> {
    let _user_routes = Router::<S>::new()
        .route("/", get(handler))
        .route("/:id", get(handler))
        .route("/", post(handler))
        .route("/:id", put(handler))
        .route("/:id", delete(handler));

    _router.nest("/users", _user_routes)
}

///
/// EXERCISE 4
///
/// Being able to test your routes without spinning up a server is very important for
/// performance and determinism. Fortunately, Axum is built on Tower, which provides a
/// convenient way to test your routes (`oneshot`).
///
/// Use `Request::builder` to construct a `Request` that makes the following unit test
/// pass. Try to pay attention to how to use `oneshot` and which imports are needed and
/// for what reasons.
///
#[tokio::test]
async fn test_routes() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let _app = Router::new().route("/users", get(identity_handler));

    let _req: Request<Body> = Request::builder()
        .method(Method::GET)
        .uri("/users")
        .body(Body::empty())
        .unwrap();

    let response = _app.oneshot(_req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "/users");
}

///
/// EXERCISE 5
///
/// Axum makes it easy for your handlers to return JSON responses. To do so, you
/// can use the `Json` wrapper type, which implements `From<T>` for any type `T`
/// that implements `serde::Serialize`.
///
/// Create a `struct` and be sure to derive Serialize, and then use your struct
/// in the following test and handler.
///
#[tokio::test]
async fn test_basic_json() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users/jdoe", get(return_json_hello_world));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users/jdoe")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, r#"{"name":"John","age":45}"#);
}

async fn return_json_hello_world() -> Json<Person> {
    Json(Person {
        name: "John".to_string(),
        age: 45,
    })
}

#[derive(serde::Serialize)]
struct Person {
    name: String,
    age: u8,
}

async fn identity_handler(request: Request<Body>) -> Body {
    Body::from(request.uri().path().to_string())
}

#[tokio::test]
async fn test_hello_world() {
    let Html(s) = handler().await;

    assert!(s.contains("Hello world! handler test"));
}
