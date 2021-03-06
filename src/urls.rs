use rocket;

use resources::users;

pub fn collect_urls(app: rocket::Rocket) -> rocket::Rocket {
    return app.mount("/", users::collect_urls());
}
