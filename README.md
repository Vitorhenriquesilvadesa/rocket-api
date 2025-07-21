# API RESTful em Rust com Rocket e SurrealDB

Este projeto é uma API RESTful completa e robusta, desenvolvida em Rust, utilizando o framework web [Rocket.rs](https://rocket.rs/) e o banco de dados NewSQL [SurrealDB](https://surrealdb.com/). A aplicação foi projetada com foco em performance, segurança e uma arquitetura limpa e escalável.

## ✨ Funcionalidades

* **CRUD de Utilizadores:** Operações completas de Criar, Ler, Atualizar e Apagar utilizadores.
* **Autenticação JWT:** Sistema de autenticação stateless seguro utilizando JSON Web Tokens.
* **Hashing de Senhas com Argon2:** As senhas dos utilizadores são protegidas com o Argon2, um dos algoritmos de hashing mais seguros disponíveis atualmente.
* **Autorização Baseada em Papéis (RBAC):** Middleware flexível para controlo de acesso a rotas com base em papéis (`Admin`, `User`).
* **Configuração Dinâmica:** Gestão de configuração a partir de ficheiros e variáveis de ambiente.
* **Logging Estruturado:** Logs contextuais e estruturados em JSON para fácil monitorização, utilizando a crate `tracing`.

## 🛠️ Stack Tecnológica

* **Linguagem:** [Rust](https://www.rust-lang.org/)
* **Framework Web:** [Rocket.rs](https://rocket.rs/)
* **Banco de Dados:** [SurrealDB](https://surrealdb.com/)
* **Serialização:** [Serde](https://serde.rs/)
* **Autenticação:** [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
* **Hashing de Senhas:** [argon2](https://crates.io/crates/argon2)
* **Logging:** [tracing](https://crates.io/crates/tracing)

## 🏗️ Arquitetura do Projeto

O projeto segue uma arquitetura de separação de responsabilidades para garantir um código limpo, testável e de fácil manutenção:

* **/src/api**: Contém toda a camada web, incluindo rotas, middlewares e os DTOs (Data Transfer Objects) para requisições e respostas.
* **/src/core**: O coração da aplicação, onde reside a lógica de negócio pura (serviços e modelos de domínio).
* **/src/infra**: Camada de infraestrutura, responsável pela comunicação com serviços externos, como o banco de dados.
* **/src/auth**: Módulo dedicado que encapsula toda a lógica de autenticação e autorização.
* **/src/config**: Responsável por carregar e gerir as configurações da aplicação.

## 🚀 Como Executar o Projeto

### Pré-requisitos

* [Rust (toolchain stable)](https://www.rust-lang.org/tools/install)
* [SurrealDB](https://surrealdb.com/docs/installation)
* [Python 3.x](https://www.python.org/downloads/) (para executar os benchmarks)

### 1. Clonar o Repositório

```bash
git clone https://github.com/Vitorhenriquesilvadesa/rocket-api.git
cd rocket-api