#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

//!
//! MIDDLEWARE
//! ----------
//!
//! Now that you have mastered handlers, the core building block of Axum
//! web applications, it is time to learn about middleware.
//!
//! Middleware allow you to uniformly modify behavior of your handlers,
//! usually in ways that are orthogonal to their core logic.
//!
//! For example, middleware are often used to do things like logging,
//! metrics, and timeouts.
//!
//! In this section, you will learn about how to use Axum middleware.
//!

use axum::body::Body;
/// for Method::GET
use axum::http::Method;
use axum::{routing::*, Router};
use base64::Engine as _;
use hyper::{Request, StatusCode};
use std::time::Duration;
// for Body::collect
use http_body_util::BodyExt;
/// for ServiceExt::oneshot
use tower::util::ServiceExt;

const BASE64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

///
/// EXERCISE 1
///
/// Axum does not have its own distinct middleware type. Instead, Axum
/// relies on `tower` to provide an abstraction for middleware. Indeed,
/// some of the main middleware you can use with Axum are actually
/// `tower_http` middleware.
///
/// In Tower, middleware are represented as a `Layer`. A `Layer` is a
/// type that implements the `tower::layer::Layer` trait. The `Layer`
/// trait is generic over a `Service`, which in Axum is a handler.
/// The layer trait has a single method, called `layer`, which takes
/// a `Service` and returns a new `Service`.
///
/// Thus, middleware in Axum is essentially a handler transformer:
/// given the old handler, the middleware returns a new handler.
///
/// Axum middleware can thus modify requests and responses of the
/// handlers they are applied to.
///
/// In Axum, you can apply middleware using the `.layer` method of
/// the `Router` type.
///
/// In the following exercise, construct a tracing middleware, and
/// explore configuring it using the `tower_http` Crate
/// documentation resources. Finally, add the middleware to the
/// `Router` by using the `.layer` method.
///  
async fn tracing_middleware() {
    #![allow(unused_imports)]
    use tower_http::trace::TraceLayer;

    let layer = TraceLayer::new_for_http();

    let _app = Router::<()>::new().layer(layer);
}

///
/// EXERCISE 2
///
/// Middleware is often used for securing APIs by using some form of authentication. The
/// `tower_http::validate_request::ValidateRequestHeaderLayer` middleware allows you to
/// validate a request header, and has a constructor (`basic`) for basic authentication.
///
/// In this exercise, you will add this middleware in such a fashion as to ensure the
/// unit test passes.
///
/// NOTE: You will not find very sophisticated authentication middleware in `tower_http`.
/// For more sophisticated authentication middleware, you will need to look elsewhere,
/// such as `axum_login` or `axum-auth`.
///
#[tokio::test]
async fn auth_middleware() {
    use tower_http::auth::require_authorization::Basic;

    #[allow(unused_imports)]
    use tower_http::validate_request::ValidateRequestHeaderLayer;

    let layer: ValidateRequestHeaderLayer<Basic<Body>> =
        ValidateRequestHeaderLayer::basic("foo", "bar");

    let app = Router::<()>::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .header(
                    "Authorization",
                    format!("Basic {}", BASE64.encode("foo:bar")),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "Hello, World!");
}

///
/// EXERCISE 3
///
/// In many web applications, you need to impose an upper bound on the amount
/// of time a request can take to process. This is often done to prevent
/// malicious actors from tying up server resources, to ensure a good user
/// experience, and to cap the amount of resources a single request can use.
///
/// In Axum, you can use the `tower_http::timeout::TimeoutLayer` middleware to
/// impose a timeout on requests. The constructor for `TimeoutLayer` takes a
/// `Duration`.
///
#[tokio::test]
async fn timeout_middleware() {
    #![allow(unused_imports)]
    use tower_http::timeout::TimeoutLayer;

    let layer = TimeoutLayer::new(Duration::from_secs(5));

    let app = Router::<()>::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "Hello, World!");
}

///
/// EXERCISE 4
///
/// Most web applications will need to support CORS (Cross-Origin Resource Sharing).
/// CORS is a mechanism that allows a web application to make requests to a different
/// domain than the one it was loaded from. This is a security feature of web browsers
/// that prevents malicious websites from making requests to other websites on behalf
/// of the user; but sometimes you want to allow this behavior, and CORS allows you to
/// do so in a way that gives the server control.
///
/// In Axum, you can use the `tower_http::cors::CorsLayer` middleware to add CORS
/// support to your application. In this exercise, you will add CORS support to the
/// application, by starting with `CorsLayer::new()`, and calling methods on the
/// layer to configure it.
///
#[tokio::test]
async fn cors_middleware() {
    use tower_http::cors::{Any, CorsLayer};

    let layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allow_origin(Any);

    let app = Router::<()>::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let response = app
        .oneshot(
            Request::builder()
                .header("Origin", "https://example.com")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "Hello, World!");
}

///
/// EXERCISE 5
///
/// Modern cloud applications need robust support for observability, and one way
/// developers can improve observability is by adding metrics to their applications.
///
/// In Axum, you can use the `tower_http::metrics::InFlightRequestsLayer` middleware
/// to add support for in-light request metrics. This middleware will track the number
/// of requests that are currently being processed by the server.
///
/// In this exercise, add support for in-flight request metrics to the application
/// by using the middleware. Note that if you use the primary `new` constructor, you
/// have to pass a counter, or call the `InFlightRequestsLayer::pair` method to both
/// perform the construction of the middleware, and to get the counter.
///
/// NOTE: The metrics support in `tower_http` is quite primitive, and we will see
/// in the next section how to use `axum_prometheus` to get something more usable
/// for a production application.
///
#[tokio::test]
async fn basic_metrics_middleware() {
    #![allow(unused_imports)]
    use tower_http::metrics::InFlightRequestsLayer;

    let (layer, counter) = InFlightRequestsLayer::pair();

    let app = Router::<()>::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let response = app
        .oneshot(
            Request::builder()
                .header("Origin", "https://example.com")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(counter.get(), 1);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "Hello, World!");
}

///
/// EXERCISE 6
///
/// The Crate `axum_prometheus` provides a more sophisticated metrics middleware
/// that allows you to expose a Prometheus metrics endpoint.
///
/// In this exercise, you will add the `PrometheusMetricLayer` middleware to the
/// application, and then add a route to the application that will expose the
/// Prometheus metrics.
///
/// You can create both the middleware and the metrics handle by using the
/// `PrometheusMetricLayer::pair` method. The handle exposes a `render` method
/// that returns a `String` containing the Prometheus metrics.
///
pub async fn prometheus_metrics_middleware() {
    use axum_prometheus::PrometheusMetricLayer;

    let (layer, handle) = PrometheusMetricLayer::pair();

    let app = Router::<()>::new()
        .route("/fast", get(|| async {}))
        .route(
            "/slow",
            get(|| async {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }),
        )
        .layer(layer)
        .route("/metrics", get(|| async move { handle.render() }));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

///
/// EXERCISE 7
///
/// You can create custom middleware using `tower`. However, Axum includes special
/// support for custom middleware that is easier to use.
///
/// In this exercise, you will use the `axum::middleware::from_fn` to create an
/// "identity" middleware that does nothing.
///
#[tokio::test]
async fn custom_middleware() {
    use axum::middleware::from_fn;

    let layer = from_fn(my_identity_middleware);

    let app = Router::<()>::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let response = app
        .oneshot(
            Request::builder()
                .header("Origin", "https://example.com")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "Hello, World!");
}

async fn my_identity_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    next.run(request).await
}
