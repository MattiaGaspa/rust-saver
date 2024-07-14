use actix_web::{web::{self, Json}, HttpResponse};
use sqlx::PgPool;
use types::{Password, PasswordList};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    password: String,
}

pub async fn add(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO passwords (id, name, password)
        VALUES ($1, $2, $3)
        "#,
        uuid::Uuid::new_v4(),
        form.name,
        form.password
        )
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


pub async fn show(pool: web::Data<PgPool>) -> Json<PasswordList> {
    let password = sqlx::query!(
        r#"
        SELECT * FROM passwords
        "#
        )
        .fetch_all(pool.get_ref())
        .await
        .expect("Failed to read database");
    let mut vector: Vec<Password> = vec![];
    
    for elem in password {
        let elem = Password {
            name: elem.name,
            password: elem.password,
        };
        vector.push(elem);
    }

    Json (
        PasswordList {
            list: vector,
        }
    )
}
