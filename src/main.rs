pub mod env;
mod errors;

mod model;
mod repository;
mod routes;

use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use env::Env;

use repository::get_config;
use routes::command::command;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match Env::check_variables() {
        Ok(env) => match get_config() {
            Ok(config) => {
                HttpServer::new(move || {
                    App::new()
                        .app_data(Data::new(config.clone()))
                        .wrap(
                            middleware::DefaultHeaders::new()
                                .add(("Cache-Control", "no-cache, no-store, must-revalidate"))
                                .add(("Pragma", "no-cache"))
                                .add(("Expires", "0")),
                        )
                        .wrap(Cors::permissive())
                        .service(command)
                })
                .bind(("0.0.0.0", env.port))?
                .run()
                .await
            }
            Err(e) => Ok(println!("{e}")),
        },
        Err(e) => Ok(println!("{e}")),
    }
}
