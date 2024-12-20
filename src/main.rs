use std::net::SocketAddr;

mod page;

const fn socket_addr([a, b, c, d]: [u8; 4], port: u16) -> std::net::SocketAddr {
    let ip = std::net::Ipv4Addr::new(a, b, c, d);
    SocketAddr::new(std::net::IpAddr::V4(ip), port)
}

async fn health_check() {
    use tokio::time::{sleep, Duration};
    loop {
        sleep(Duration::from_secs(5)).await;
        eprintln!("Chugging along...");
    }
}

mod middleware {
    use axum::{
        body::Body,
        http::{Request, Response},
        middleware::Next,
    };

    pub async fn log_incoming_ip(req: Request<Body>, next: Next) -> Response<Body> {
        eprintln!("Incoming request!");
        next.run(req).await
    }
}

async fn run(database_url: &str, addr: SocketAddr) {
    let pool = sqlx::PgPool::connect(database_url)
        .await
        .expect("Failed to create pool");

    page::Todo::create_table_query()
        .execute(&pool)
        .await
        .expect("Failed to create table");

    let app = page::routes()
        .layer(axum::middleware::from_fn(middleware::log_incoming_ip))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    eprintln!("Listening on {}", addr);

    tokio::spawn(health_check());
    axum::serve::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let addr: SocketAddr = std::env::var("APP_ADDRESS")
        .map(|x| x.parse().expect("APP_ADDRESS is incorrect!"))
        .unwrap_or(const { socket_addr([172, 0, 0, 0], 3000) });

    run(&database_url, addr).await;
}
