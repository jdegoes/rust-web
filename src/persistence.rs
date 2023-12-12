#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

//!
//! PERSISTENCE
//! -----------
//! 
//! Every web application needs to store data. There are, of course, many Rust 
//! crates for interacting with NoSQL databases and AWS services like DynamoDB.
//! There are even some ORM-like solutions for Rust that aim to emulate the 
//! ORM solutions from the Java world. However, most web applications will rely 
//! on relational databases for persistence because of their ubiquity,
//! flexibility, performance, and ACID guarantees.
//! 
//! Rust has many solutions for interacting with relational databases. One of 
//! the most common that does not try to hide SQL from the user, and which is
//! fully compatible with Tokio, is the `sqlx` crate.
//! 
//! In this section, you will learn the basics of using the `sqlx` crate to
//! interact with a PostgreSQL database. 
//! 
//! As a first step, run `cargo install sqlx-cli` to install the SQLx CLI.
//! 


