use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Row};
use std::env;

fn run_socket() -> Result<String> {
    let ctx = zmq::Context::new();
    let sock = ctx.socket(zmq::REP)?;
    sock.bind("tcp://*:8989")?;
    println!("Send a message to port 8989 to get a response (use a REQ socket)");
    let s = sock
        .recv_string(0)?
        .unwrap_or_else(|_| "not a utf-8 string".into());
    sock.send(&format!("Hello, {s}"), 0)?;
    println!("Check your socket for the response");
    Ok(s)
}

async fn run_db(pool: &SqlitePool, name: &str) -> Result<()> {
    let mut conn = pool.acquire().await?;
    sqlx::query("INSERT INTO sample (name) VALUES (?)")
        .bind(name)
        .execute(&mut conn)
        .await?;
    let count: u32 = sqlx::query("SELECT COUNT(*) FROM sample WHERE name = ?")
        .bind(name)
        .fetch_one(&mut conn)
        .await?
        .get(0);
    println!("{name} has sent {count} messages");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database = env::var("DATABASE_URL").unwrap_or_else(|_| {
        log::warn!("DATABASE_URL variable undefined. Using 'sqlite:/tmp/sample.db'");
        "sqlite:/tmp/sample.db".into()
    });
    let pool = SqlitePool::connect(&database).await?;
    let name = run_socket()?;
    run_db(&pool, &name).await?;
    Ok(())
}
