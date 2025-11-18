use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};
use rocket_db_pools::{Database, sqlx};

#[derive(Database)]
#[database("sqlite_db")]
pub struct Db(sqlx::SqlitePool);

pub type DbResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => {
                println!("Database migrations completed successfully");
                Ok(rocket)
            }
            Err(e) => {
                eprintln!("Failed to run database migrations: {}", e);
                Err(rocket)
            }
        },
        None => {
            eprintln!("Database connection not found");
            Err(rocket)
        }
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("Database Migrations", run_migrations))
    })
}
