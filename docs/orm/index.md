# ORM: SQL-First Persistence

The `montrs-orm` package provides a unified, async-first interface for database operations. It prioritizes SQL clarity and type safety over complex abstractions.

## 🗄️ Core Philosophy

-   **SQL-First**: We believe SQL is the best language for querying data. Our ORM makes it easy to write and execute SQL while handling the mapping to Rust structs.
-   **Backend Agnostic**: Switch between SQLite for local development and PostgreSQL for production with minimal changes.
-   **Async by Default**: All database operations are non-blocking and integrated with the Tokio runtime.

## 🏗️ Models and Rows

Define your models as standard Rust structs and implement the `FromRow` trait (or use our macros).

```rust
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}
```

## 🔍 Querying Data

Use the `db` context within your Loaders and Actions to perform queries.

```rust
async fn get_users(ctx: Context) -> Result<Vec<User>> {
    let users = ctx.db()
        .query("SELECT * FROM users WHERE active = true")
        .fetch_all::<User>()
        .await?;
    Ok(users)
}
```

## 🛠️ Mutations

Perform inserts, updates, and deletes using a similar fluent interface.

```rust
async fn create_user(ctx: Context, input: CreateUserInput) -> Result<User> {
    let user = ctx.db()
        .execute("INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *", 
                 &[input.username, input.email])
        .fetch_one::<User>()
        .await?;
    Ok(user)
}
```

## 🤖 Agents and the ORM

For agents, the ORM layer is where the **Data Model** lives.
-   **Validator Inference**: By looking at structs implementing `FromRow`, an agent can understand the database structure.
-   **SQL Generation**: Since we use standard SQL, models can easily generate queries to fetch or modify data.
-   **Error Resolution**: `DbError` implements `AgentError`, providing specific codes for constraint violations (e.g., unique key conflict), allowing the agent to suggest structure or data fixes.
