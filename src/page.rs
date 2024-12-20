use axum::{
    extract::{Path, State},
    response::Html,
    routing::{delete, get, post},
    Form, Router,
};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(sqlx::FromRow)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

impl Todo {
    pub fn create_table_query(
    ) -> sqlx::query::Query<'static, sqlx::Postgres, sqlx::postgres::PgArguments> {
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            text TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
        );"#,
        )
    }
}

#[derive(Deserialize)]
struct CreateTodo {
    text: String,
}

pub fn routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new()
        .route("/", get(index))
        .route("/todos", post(create_todo))
        .route("/todos/:id/toggle", post(toggle_todo))
        .route("/todos/:id", delete(delete_todo))
}

fn todo_item(todo: Todo) -> Markup {
    html! {
        div class="todo-item" {
            input
                type="checkbox"
                checked?[todo.completed]
                hx-post={"/todos/" (todo.id) "/toggle"}
                hx-swap="outerHTML"
                hx-target="closest div";

            span class=(if todo.completed { "completed" } else { "" }) {
                (todo.text)
            }

            button
                hx-delete={"/todos/" (todo.id)}
                hx-target="closest div"
                hx-swap="outerHTML" {
                "Delete"
            }
        }
    }
}

async fn index(State(pool): State<Pool<Postgres>>) -> Html<String> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap();

    let markup = html! {
        (DOCTYPE)
        html {
            head {
                title { "HTMX Todo App" }
                script src="https://unpkg.com/htmx.org@1.9.10" {}
                style { r#"
                    .completed { text-decoration: line-through; }
                    .todo-item { display: flex; align-items: center; gap: 8px; margin: 4px 0; }
                "#}
            }
            body {
                h1 { "Todo List" }

                form hx-post="/todos" hx-swap="beforeend" hx-target="#todo-list" {
                    input type="text" name="text" placeholder="New todo..." required;
                    button type="submit" { "Add" }
                }

                div id="todo-list" {
                    @for todo in todos {
                        (todo_item(todo))
                    }
                }
            }
        }
    };

    Html(markup.into_string())
}

async fn create_todo(
    State(pool): State<Pool<Postgres>>,
    Form(todo): Form<CreateTodo>,
) -> Html<String> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (text, completed) VALUES ($1, false) RETURNING *",
    )
    .bind(&todo.text)
    .fetch_one(&pool)
    .await
    .unwrap();

    Html(todo_item(todo).into_string())
}

async fn toggle_todo(State(pool): State<Pool<Postgres>>, Path(id): Path<i32>) -> Html<String> {
    let todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET completed = NOT completed WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Html(todo_item(todo).into_string())
}

async fn delete_todo(State(pool): State<Pool<Postgres>>, Path(id): Path<i32>) -> Html<String> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Html("".to_string())
}
