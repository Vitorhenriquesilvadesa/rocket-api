# RESTful API in Rust with Rocket and SurrealDB

This project is a complete and robust RESTful API developed in Rust, using the [Rocket.rs](https://rocket.rs/) web framework and the NewSQL database [SurrealDB](https://surrealdb.com/). The application is designed with a focus on performance, security, and a clean, scalable architecture.

## ✨ Features

* **User CRUD:** Full Create, Read, Update, and Delete operations for users.
* **JWT Authentication:** Secure stateless authentication system using JSON Web Tokens.
* **Password Hashing with Argon2:** User passwords are protected using Argon2, one of the most secure hashing algorithms available today.
* **Role-Based Access Control (RBAC):** Flexible middleware to control route access based on roles (`Admin`, `User`).
* **Dynamic Configuration:** Configuration management via files and environment variables.
* **Structured Logging:** Contextual and structured logs in JSON format for easier monitoring, using the `tracing` crate.

## 🛠️ Tech Stack

* **Language:** [Rust](https://www.rust-lang.org/)
* **Web Framework:** [Rocket.rs](https://rocket.rs/)
* **Database:** [SurrealDB](https://surrealdb.com/)
* **Serialization:** [Serde](https://serde.rs/)
* **Authentication:** [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
* **Password Hashing:** [argon2](https://crates.io/crates/argon2)
* **Logging:** [tracing](https://crates.io/crates/tracing)

## 🏗️ Project Architecture

The project follows a clean architecture with separation of concerns to ensure maintainable, testable, and scalable code:

* **/src/api**: Contains the web layer, including routes, middlewares, and DTOs (Data Transfer Objects) for requests and responses.
* **/src/core**: The heart of the application, containing pure business logic (services and domain models).
* **/src/infra**: Infrastructure layer responsible for interacting with external services such as the database.
* **/src/auth**: Dedicated module encapsulating all authentication and authorization logic.
* **/src/config**: Responsible for loading and managing application configuration.

## 🚀 How to Run the Project

### Prerequisites

* [Rust (stable toolchain)](https://www.rust-lang.org/tools/install)
* [SurrealDB](https://surrealdb.com/docs/installation)
* [Python 3.x](https://www.python.org/downloads/) (for running benchmarks)

### 1. Clone the Repository

```bash
git clone https://github.com/Vitorhenriquesilvadesa/rocket-api.git
cd rocket-api