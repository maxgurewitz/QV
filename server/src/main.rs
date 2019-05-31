#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix_web;

pub mod models;
pub mod schema;

use self::models::*;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use rand::Rng;
use std::env;

#[derive(Serialize)]
struct HomePageResource {
    name: String,
}

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn build_pg_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(pg_manager).unwrap()
}

// routes
fn home_page_route(_req: HttpRequest) -> Result<web::Json<HomePageResource>, Error> {
    Ok(web::Json(HomePageResource {
        name: "foo".to_string(),
    }))
}

fn not_found_page_route(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound()
        .content_type("application/json")
        .body("{ \"message\": \"route not found\" }"))
}

fn login_route(data: web::Data<AppData>, _reg: HttpRequest) -> Result<HttpResponse, Error> {
    use schema::users;

    // TODO error handling
    let connection = data.pg_pool.get().unwrap();

    let mut rng = rand::thread_rng();
    let num: f32 = rng.gen();

    let new_user = NewUser {
        openid: format!("test-user-{}", num.to_string()),
        email: "test@email.com",
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&*connection)
        .expect("Error saving new post");

    // create user if not found
    Ok(HttpResponse::NotFound()
        .content_type("application/json")
        .body("{ \"message\": \"success\" }"))
}

struct AppData {
    pg_pool: PgPool,
}

// auth flow https://auth0.com/docs/flows/concepts/auth-code
// https://auth0.com/docs/flows/guides/implicit/add-login-implicit
// https://auth0.com/docs/architecture-scenarios/spa-api
fn main() {
    let port = env::var("PORT").unwrap_or("8000".to_string());

    HttpServer::new(|| {
        let pg_pool = build_pg_pool();

        App::new()
            .data(AppData {
                pg_pool: pg_pool.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .route("/login", web::post().to(login_route))
                    .route("/home", web::post().to(home_page_route)),
            )
            .default_service(web::route().to(not_found_page_route))
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect("Can not bind to port")
    .run()
    .unwrap();
}
