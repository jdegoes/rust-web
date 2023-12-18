//!
//! ARCHITECTURE
//! ------------
//!
//! This module allows an exploration of how you might structure a larger
//! Axum application. It is the final graduation project for the course.
//!
//! In particular, the emphasis is on modularity and testability: being
//! able to break down the functionality of the whole app into distinct
//! pieces, each of which may be tested independently, and without having
//! to cover everything with integration or system tests.
//!
//! To achieve a modular and testable design, we can take advantage of
//! both features of Axum and features of the Rust programming language
//! itself, as well as learning lessons from modularity and testability
//! in other languages that have long been used for building web apps.
//!
//!

//
// EXERCISE 5
// ----------
//
// Now it is time to write your application logic, in a way that is not
// tied to Axum or SQLx. You will use the traits you designed in the
// previous exercise to write your application logic. Avoid using any
// data types directly from Axum or SQLx.
//
// A key test of your application architecture is whether or not you can
// write unit tests for the logic that do not require any real web servers,
// real databases, or real APIs. So as you develop your application logic,
// be sure to introduce tests, which might necessitate you providing
// alternate implementations of the traits you designed previously.
//

//
// EXERCISE 6
// ----------
//
// Now that you have written and tested your application logic, you can use
// Axum to develop routes, with handlers that call into your application logic.
// Take care to wire up everything correctly for production operation.
// Start your web server and verify its behavior matches your expectations.
//
