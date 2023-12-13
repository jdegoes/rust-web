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
