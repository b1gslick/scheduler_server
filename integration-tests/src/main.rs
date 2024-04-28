use futures_util::FutureExt;
use std::io::{self, Write};
use std::process::Command;

use activities_scheduler_server::{config, handle_errors, oneshot, setup_store};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Activity {
    title: String,
    content: String,
    time: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ActivityAnswer {
    id: i32,
    title: String,
    content: String,
    time: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Token(String);

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenvy::dotenv().ok();
    let config = config::Config::new().expect("Config can't be set");

    let s = Command::new("sqlx")
        .arg("database")
        .arg("drop")
        .arg("--database-url")
        .arg(format!(
            "postgres://{}:{}/{}",
            config.database_host, config.database_port, config.database_name
        ))
        .arg("-y")
        .output()
        .expect("sqlx command failed to start");

    io::stdout().write_all(&s.stderr).unwrap();

    let s = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(format!(
            "postgres://{}:{}/{}",
            config.database_host, config.database_port, config.database_name
        ))
        .output()
        .expect("sqlx command failed to start");

    io::stdout().write_all(&s.stderr).unwrap();

    let store = setup_store(&config).await?;

    let handler = oneshot(store).await;

    let u = User {
        email: "test@mail.com".to_string(),
        password: "password".to_string(),
    };
    let token;

    print!("Running register_new_user...");
    let result = std::panic::AssertUnwindSafe(register_new_user(&u))
        .catch_unwind()
        .await;
    match result {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            println!("x");
            std::process::exit(1);
        }
    }

    print!("Running login...");
    match std::panic::AssertUnwindSafe(login(&u)).catch_unwind().await {
        Ok(t) => {
            token = t;
            println!("✓");
        }
        Err(_) => {
            let _ = handler.sender.send(1);
            println!("x");
            std::process::exit(1);
        }
    }

    print!("Running post_activities...");
    match std::panic::AssertUnwindSafe(post_activities(token))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            println!("x");
            std::process::exit(1);
        }
    }

    let _ = handler.sender.send(1);

    Ok(())
}

async fn register_new_user(user: &User) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/registration")
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await;

    assert_eq!(res.unwrap(), "Account added".to_string());
}

async fn login(user: &User) -> Token {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/login")
        .json(&user)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    res.json::<Token>().await.unwrap()
}

async fn post_activities(token: Token) {
    let q = Activity {
        title: "First activitiess".to_string(),
        content: "How can I test?".to_string(),
        time: 10,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/activities")
        .header("Authorization", token.0)
        .json(&q)
        .send()
        .await
        .unwrap()
        .json::<ActivityAnswer>()
        .await
        .unwrap();

    assert!(res.id != 0);
    assert_eq!(res.title, q.title);
}
