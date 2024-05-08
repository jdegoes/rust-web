# rust-web

This is the repository containing all workshop materials for _Introduction to Rust Web Programming_.

## Overview

Rust, though often celebrated for its superior system-level programming capabilities, is rapidly gaining traction in the world of web development. Its promise of fast performance, minimal memory usage, instantaneous startup, and unparalleled memory/concurrency safety sets it apart from traditional web development languages. However, many are daunted by the perceived steep learning curve.

"Build Web Apps with Rust" is a workshop crafted specifically for individuals new to Rust. The aim is to bridge the gap between Rust’s powerful potential and the practical realm of web applications. Dive into the world of Rust-based web apps, and learn how to leverage the unique features of Rust to build nimble and efficient web platforms. By the end of this course, participants will confidently be able to create basic CRUD applications, integrate persistent data storage, and seamlessly interact with cloud APIs using Rust.

## Who Should Attend

Developers intrigued by Rust and its potential for web application development, especially those with little to no experience in Rust. Whether you're a seasoned developer in other languages or just starting, this workshop is tailored to get you up and running.
Prerequisites

A basic understanding of web development concepts and practices. Familiarity with any programming language is beneficial, but not required.

## Topics

 - Introduction to Rust for Web Development
 - Rust's memory and concurrency model in web context
 - Setting up a Rust web environment
 - Building basic CRUD operations with Rust
 - Integrating data persistence in Rust web apps
 - Interacting with cloud APIs using Rust
 - Tips and best practices for smooth development

## Daily Structure

Two days, 8 hours a day, starting at 9:00 AM Mountain Time.

## Attendance

Participation in this workshop is in-person and online. Participants will receive a link to a virtual meeting room a day prior to the event. In this session, they will be able to view the workshop in real-time, engage in discussions, pose queries to the instructor, and collaborate with fellow attendees. To make the most of the workshop, ensure your computer is equipped with a text editor, and have Rust and Cargo (Rust's package manager) set up.

## Materials

Each attendee will receive sample code snippets, a detailed course itinerary, and hands-on exercises in digital format. Note that recording by participants is strictly not allowed.

## Setup

You need to install Rust and Cargo on your machine. You can do this by following the instructions on the official Rust website: 

https://www.rust-lang.org/tools/install

When you have completed installation, you can attempt to build by using `cargo build`:

```bash
cargo build
```

This will produce a SQLx error that complains that the environment variable `DATABASE_URL` is undefined.

To fix this error, you will have to prepare your computer for SQLx development using Postgres.

## Postgres Preparation

To prepare your computer for SQLx development, you will need to install Postgres, and ensure it is running as a service.

The way you do this depends on your operating system. However, the official Postgres website provides detailed instructions for all operating systems:

https://www.postgresql.org/download/

After you have installed Postgres, you will need to install `sqlx-cli`, a command-line tool for SQLx applications:

```bash
cargo install sqlx-cli
```

Next, ensure you have set `DATABASE_URL` environment variable to be a valid Postgres connection URL:

```bash
export DATABASE_URL=postgres://localhost:5432/postgres
```

Finally, you need to create the table that is used in the examples by running the only migration script:

```bash
cargo sqlx migrate run
```

Once you have completed all these steps, you are now ready for SQLx development using Postgres.

If you have trouble, keep in mind you can always replace the `query!` macros with a call to 
`query` in order to eliminate the compile-time errors. However, you will still have to have a 
valid and running Postgres database in order to complete the exercises.