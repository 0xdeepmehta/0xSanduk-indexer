#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;
mod solana;
use rocket::routes;
use solana::subscribe_to_program;

use crate::routes::get_all_sanduk;
use crate::routes::index;
use crate::solana::get_accounts_and_update;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connnecting to {}", database_url))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_accounts_and_update();
    
    subscribe_to_program();

    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount("/", routes![index, get_all_sanduk])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
