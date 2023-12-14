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

//
// EXERCISE 1
// ----------
//
// You are creating a todo app. Furthermore, you want to utilize best
// practice architecture and design patterns that may cost more in
// the short term, but will increase the quality of your solution and
// lower costs of maintenance.
//
// Naturally, you will be using the Rust programming language for this
// project. You have decided to use Axum as your web framework, and
// Postgres as the database, with SQLx as the database library.
//
// Your first step is to design the data structures that will be used
// for modeling different entities in the domain of a todo application.
//
// Use best-practice data modeling techniques to design these data
// structures, deriving implementations for traits like `Clone`,
// `Debug`, `PartialEq`, `Serialize`, and `Deserialize`.
//

//
// EXERCISE 2
// ----------
//
// Now that you have designed data structures for the todo app, it's
// natural to think about how you might persist them to a database.
//
// Construct a relational data model for your data structures that
// follows best practices for relational data modeling, including
// normalization and foreign keys.
//
// Then write SQLx migration scripts to create the relational data
// model in Postgres. Finally, use the sqlx CLI to run the migration
// on your Postgres database so you are ready to proceed to the next
// step.
//

//
// EXERCISE 3
// ----------
//
// When working on application architecture, your top priorities are
// as follows:
//
// 1. Modularity. You want to be able to break down the application
//   into distinct pieces, each of is devoted to specific and highly
//   scoped functionality. No part of the application should need to
//   know about the internals of any other part of the application.
//   Each part should be independently replaceable.
//
// 2. Testability. You want to be able to test all of your application
//   logic in isolation, without having to cover everything with
//   integration or system tests. This means that your application logic
//   must be independent of real web servers, real database, and real
//   APIs, which implies that you have facades that abstract over
//   the core pieces of functionality required by your logic.
//
// In the exercises in this workshop so far, we have been directly
// talking to databases inside handlers. Moreover, our handlers have
// been thoroughly entangled with the data types of Axum. This close
// coupling to database and web server does not meet the modularity
// and testability requirements of a well-designed application.
//
// Do a thought experiment for the todo app you are developing:
//
// 1. What, if anything, does your application logic require from Axum?
//
// 2. What, if anything, does your application logic require from SQLx?
//
// Now try to scope these requirements to the smallest possible surface
// area. For example, obviously a todo app requires persistence, but it
// does not require the full power of SQL: in fact, only a few different
// SQL queries will be necessary to implement the entire application.
//
// In Rust, traits are essential tools of abstraction. Beyond traits, you
// have your choice of polymorphism or dynamic dispatch (Box<dyn Trait>).
//
// Design a set of traits that abstract over the functionality required
// by your application logic. Can you think of any patterns from other
// programming languages that might be useful here?
//

//
// EXERCISE 4
// ----------
//
// Now that you have designed a set of traits that abstract over the
// functionality required by your application logic, you can implement
// these traits for data types that are bound to Axum and SQLx.
//
// The implementations will be "live" implementations that directly
// talk to Axum and SQLx, but they will be hidden behind the traits
// you designed in the previous exercise.
//

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
