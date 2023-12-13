#![allow(dead_code)]
#![allow(unreachable_code)]

//!
//! CONTEXT
//! -------
//!
//! So far, you have seen context-free web applications in Rust. These web applications
//! do not share context with a higher level or between themselves.
//!
//! While appropriate for very simple applications, most real world applications will need
//! some form of context. For example, a web application might need to access a database,
//! and it would be inefficient to open a new connection to the database for every request.
//! So most handlers will end up drawing from a database connection pool.
//!
//! Axum has been designed to facilitate sharing context, both between handlers, and
//! between handlers and higher levels of the application.
//!
//! In this section, you will explore these mechanisms.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::Path;
use axum::{Extension, Json};
#[allow(unused_imports)]
use axum::extract::State;
#[allow(unused_imports)]
use axum::{body::Body, http::Method, routing::*};
#[allow(unused_imports)]
use hyper::Request;
use tokio::sync::Mutex;

///
/// EXERCISE 1
///
/// While not a highly maintainable solution, it is possible to create contextual
/// web applications by using closures to capture context.
///
/// In this exercise, share the same `usd_to_gbp` rate between the two routes
/// by using closures.
///
#[tokio::test]
async fn closure_shared_context() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let gbp_to_usd_rate = 1.3;

    let _app = Router::<()>::new()
        .route("/usd_to_gbp", get(move |usd: String| async move { convert_usd_to_gbp(usd, gbp_to_usd_rate)}))
        .route("/gbp_to_usd", get(move |gbp: String| async move { convert_gbp_to_usd(gbp, gbp_to_usd_rate)}));

    let response = _app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/usd_to_gbp")
                .body(Body::from("100"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let _body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(_body_as_string, "130");
}
fn convert_usd_to_gbp(usd: String, gbp_to_usd_rate: f64) -> String {
    format!("{}", usd.parse::<f64>().unwrap() * gbp_to_usd_rate)
}
fn convert_gbp_to_usd(gbp: String, gbp_to_usd_rate: f64) -> String {
    format!("{}", gbp.parse::<f64>().unwrap() / gbp_to_usd_rate)
}

///
/// EXERCISE 2
///
/// The previous exercise was almost too easy, because the context was of type
/// `f64`, which is `Copy`. This means that the context was copied into both
/// closures, rather than truly shared between them.
///
/// Of course, for any data type that you do not wish to mutate, you can always
/// implement `Clone`, and then manually clone the context into each closure.
///
/// But what if you want to share a mutable context between handlers?
///
/// In this exercise, you will share a mutable context between handlers.
/// Specifically, you will share a mutably editable exchange rate between
/// GBP and USD currencies. Consider using the `Arc` type, which you will
/// have to use atop Tokio's Mutex in order to support mutation.
///
/// When you are done, try to generalize what you have learned about sharing
/// context between handlers. What would you use if the context were
/// immutable? What would you use if the context were mutable?
///
#[tokio::test]
async fn shared_mutable_context() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    use tokio::sync::Mutex;

    let _gbp_to_usd_rate: Arc<Mutex<f64>> = Arc::new(Mutex::new(1.3));

    let rate1 = _gbp_to_usd_rate.clone();
    let rate2 = _gbp_to_usd_rate.clone();
    let rate3 = _gbp_to_usd_rate.clone();

    let _app = Router::<()>::new()    
        .route(
            "/usd_to_gbp",
            get(|usd: String| async move {                 
                let mut locked = rate1.lock().await;

                *locked = 1.4;

                convert_usd_to_gbp(usd, *locked)
            }),
        )
        .route(
            "/gbp_to_usd",
            get(|gbp: String| async move { 
                let mut locked = rate2.lock().await;

                *locked = 1.5;

                convert_gbp_to_usd(gbp, *locked) 
            }),
        )
        .route(
            "/usd_to_gbp_rate",
            get(|_: ()| async move { 
                let locked = rate3.lock().await;

                format!("{}", *locked)
            }),
        );

    let response = _app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/usd_to_gbp")
                .body(Body::from("100"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let _body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(_body_as_string, "130");
}

///
/// EXERCISE 3
///
/// Having to write all your handlers as closures is not very ergonomic, and could
/// lead to either boilerplate or gigantic functions that define all handlers.
///
/// Instead, Axum provides direct support for sharing context. This shared context
/// can be specified in your Router, and it can be passed into your handlers as
/// a State parameter.
///
/// In this exercise, share the same `usd_to_gbp` rate between the two routes
/// by using the `State` extractor, defined in `axum::extract`. Note that you
/// will have to supply the state by using the `.with_state` method on your
/// Router. An example (using () as the state type) has been provided below.
///
#[tokio::test]
async fn state_shared_context() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let gbp_to_usd_rate = 1.3;

    let _app = Router::new()
        .route("/usd_to_gbp", get(usd_to_gbp_handler))
        .route("/gbp_to_usd", get(gbp_to_usd_handler))
        .with_state(gbp_to_usd_rate);

    let response = _app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/usd_to_gbp")
                .body(Body::from("100"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let _body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(_body_as_string, "130");
}
async fn usd_to_gbp_handler(State(rate): State<f64>, usd: String) -> String {
    convert_usd_to_gbp(usd, rate)
}
async fn gbp_to_usd_handler(State(rate): State<f64>, gbp: String) -> String {
    convert_gbp_to_usd(gbp, rate)
}

///
/// EXERCISE 4
///
/// Now that you have seen Axum's first-class support for context sharing, it's
/// time to leverage your knowledge of Rust to enable sharing mutable context
/// between handlers, building upon what you have done in previous exercises.
///
/// Modify this exercise to share a mutable exchange rate between GBP and USD.
///
#[tokio::test]
async fn mutable_state_shared_context() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let _gbp_to_usd_rate = 1.3;

    let _app = Router::new()
        .route("/usd_to_gbp", get(mutable_usd_to_gbp_handler))
        .route("/gbp_to_usd", get(mutable_gbp_to_usd_handler))
        .with_state(Arc::new(Mutex::new(_gbp_to_usd_rate)));

    let response = _app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/usd_to_gbp")
                .body(Body::from("100"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let _body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(_body_as_string, "130");
}
async fn mutable_usd_to_gbp_handler(State(arc): State<Arc<Mutex<f64>>>, usd: String) -> String {
    let mut locked = arc.lock().await;
    
    *locked = 1.4;

    convert_usd_to_gbp(usd, *locked)
}
async fn mutable_gbp_to_usd_handler(State(arc): State<Arc<Mutex<f64>>>, usd: String) -> String {
    let mut locked = arc.lock().await;
    
    *locked = 1.5;

    convert_gbp_to_usd(usd, *locked)
}

///
/// EXERCISE 5
///
/// The type `S` flows through a lot of the types in Axum (Router, MethodRouter,
/// Handler, etc.). If you examine closely the signatures for methods that combine
/// routers, you will see that their state types have to be exactly the same.
///
/// What happens if your handlers, from different parts of your application,
/// require totally different state?
///
/// One possible solution to this problem is to make your handlers polymorphic
/// in the type of state they handle, and to use traits that expose "accessors"
/// for the specific state type they require.
///
/// In this exercise, you will use this technique to complete the following
/// exercise.
///
/// Assume that some handlers require state type `GBPtoUSD`, and that other
/// handlers require state type `EURtoUSD`. Further, assume you have a
/// composite state type, `AllExchangeRates`, that contains both `GBPtoUSD`
/// and `EURtoUSD`.
///
/// Invent traits that can describe what each type of handler requires from
/// the "global state", and then make the handlers polymorphic in the state
/// type, requiring only an implementation of the appropriate trait.
///
/// You might have to supply some type hints to the compiler in order to
/// construct the routes with your polymorphic handlers.
///
/// This technique is very powerful, and it can allow state to vary across
/// a modular web application, where different types of endpoints have
/// different requirements for context.
///
#[tokio::test]
async fn generic_state_shared_context() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let _app = Router::new()
        .route("/usd_to_gbp", get(generic_usd_to_gbp_handler::<AllExchangeRates>))
        .route("/gbp_to_usd", get(generic_gbp_to_usd_handler::<AllExchangeRates>))
        .route("/eur_to_usd", get(generic_eur_to_usd_handler::<AllExchangeRates>))
        .route("/usd_to_eur", get(generic_usd_to_eur_handler::<AllExchangeRates>))
        .with_state(AllExchangeRates {
            gbp_to_usd: GBPtoUSD(1.3),
            eur_to_usd: EURtoUSD(1.2),
        });

    let response = _app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/usd_to_gbp")
                .body(Body::from("100"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let _body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(_body_as_string, "130");
}
async fn generic_usd_to_gbp_handler<S: HasGBPtoUSD>(State(s): State<S>, _price: String) -> String {
    let gbp_to_usd = s.gbp_to_usd();

    format!("{}", _price.parse::<f64>().unwrap() * gbp_to_usd.0)
}
async fn generic_gbp_to_usd_handler<S: HasGBPtoUSD>(State(s): State<S>, _price: String) -> String {
    let gbp_to_usd = s.gbp_to_usd();

    format!("{}", _price.parse::<f64>().unwrap() / gbp_to_usd.0)
}
async fn generic_eur_to_usd_handler<S: HasEURtoUSD>(State(s): State<S>, _price: String) -> String {
    let eur_to_usd = s.eur_to_usd();

    format!("{}", _price.parse::<f64>().unwrap() * eur_to_usd.0)
}
async fn generic_usd_to_eur_handler<S: HasEURtoUSD>(State(s): State<S>, _price: String) -> String {
    let eur_to_usd = s.eur_to_usd();

    format!("{}", _price.parse::<f64>().unwrap() / eur_to_usd.0)
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct AllExchangeRates {
    gbp_to_usd: GBPtoUSD,
    eur_to_usd: EURtoUSD,
}
impl HasGBPtoUSD for AllExchangeRates {
    fn gbp_to_usd(&self) -> GBPtoUSD {
        self.gbp_to_usd
    }
}
impl HasEURtoUSD for AllExchangeRates {
    fn eur_to_usd(&self) -> EURtoUSD {
        self.eur_to_usd
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct GBPtoUSD(f64);
#[derive(Clone, Copy, Debug, PartialEq)]
struct EURtoUSD(f64);
trait HasGBPtoUSD {
    fn gbp_to_usd(&self) -> GBPtoUSD;
}
trait HasEURtoUSD {
    fn eur_to_usd(&self) -> EURtoUSD;
}

///
/// EXERCISE 6
///
/// Although it is possible to share virtually any kind of context using State,
/// with the appropriate type classes and polymorphic handlers allowing state
/// to vary across a web application, some would prefer to reduce the amount of
/// ceremony required to share varying context, and are willing to accept a
/// tradeoff in terms of static type safety.
///
/// For this audience, Axum has a solution called Extensions. Extensions can be
/// used to share context between middleware and handlers, or just to share
/// context either between handlers, between middleware, or between either
/// handlers or middleware and higher levels of the application.
///
/// In order to use extensions, your handler may require a parameter of type
/// `axum::extract::Extension<T>` where `T` is the type of the context you
/// wish to share. Then you must install a layer in your router, which holds
/// the context, and you can do that with the `Extension(...)` constructor.
///
/// In this exercise, you will implement the same exchange-rate-sharing
/// application, but this time using an extension to share state.
///
/// Experiment with what happens when you forget to install the extension.
/// Under what circumstances would you prefer extensions to state for
/// sharing context? Under what circumstances would you prefer the reverse?
///
#[tokio::test]
async fn extension_shared_context() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let _gbp_to_usd_rate = 1.3;

    let _app = Router::<()>::new()
        .route("/usd_to_gbp", get(extension_usd_to_gbp_handler))
        .route("/gbp_to_usd", get(extension_gbp_to_usd_handler))
        .layer(Extension(_gbp_to_usd_rate));

    let response = _app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/usd_to_gbp")
                .body(Body::from("100"))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let _body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(_body_as_string, "130");
}
async fn extension_usd_to_gbp_handler(Extension(rate): Extension<f64>, usd: String) -> String {
    format!("{}", usd.parse::<f64>().unwrap() * rate)
}
async fn extension_gbp_to_usd_handler(Extension(rate): Extension<f64>, gbp: String) -> String {
    format!("{}", gbp.parse::<f64>().unwrap() / rate)
}

///
/// GRADUATION PROJECT
///
/// Provide a complete implementation of the following API, which uses shared mutable
/// state across all the handlers to provide a fake implementation of the full CRUD
/// API.
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
    let state: Arc<UsersState> = Arc::new(UsersState {
        map: Mutex::new(HashMap::new()),
    });
    
    let app = Router::new()
        .route("/users",     get(get_users_handler))
        .route("/users/:id", get(get_user_by_id))
        .route("/users",     post(create_user))
        .route("/users/:id", put(update_user_by_id))
        .route("/users/:id", delete(delete_user_by_id))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
async fn get_users_handler(State(state): State<Arc<UsersState>>) -> Json<Vec<User>> {
    let map = state.map.lock().await;

    Json(map.values().map(|x| { x.clone() }).collect())
}
async fn get_user_by_id(State(state): State<Arc<UsersState>>, Path(id): Path<String>) -> Json<Option<User>> {
    let map = state.map.lock().await;

    Json(map.get(&id).map(|x| { x.clone() }))
}
async fn create_user(State(state): State<Arc<UsersState>>, Json(user): Json<User>) -> () {
    let mut map = state.map.lock().await;

    map.insert(user.id.clone(), user.clone());
}
async fn update_user_by_id(State(state): State<Arc<UsersState>>, Path(id): Path<String>, Json(user): Json<User>) -> () {
    let mut map = state.map.lock().await;

    map.insert(id.clone(), user.clone());
}
async fn delete_user_by_id(State(state): State<Arc<UsersState>>, Path(id): Path<String>) -> () {
    let mut map = state.map.lock().await;

    map.remove(&id);
}
struct UsersState {
    map: Mutex<HashMap<String, User>>,
}
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}