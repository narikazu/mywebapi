extern crate iron;
extern crate router;
extern crate logger;
extern crate env_logger;
extern crate rustc_serialize;
extern crate chrono;
extern crate uuid;

mod model;
mod database;
mod handlers;

use model::*;
use database::Database;
use handlers::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;
use logger::Logger;
use uuid::Uuid;

fn main() {
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(None);

    let mut database = Database::new();
    let author = Author::new("Me");
    let post = Post::new("First Post",
                         "This is the first post ever",
                         &author,
                         chrono::offset::utc::UTC::now(),
                         Uuid::new_v4());
    database.add_post(post);
    let post = Post::new("My web app is now online",
                         "Today marks the day that this app is online!",
                         &author,
                         chrono::offset::utc::UTC::now(),
                         Uuid::new_v4());
    database.add_post(post);

    let handlers = Handlers::new(database);
    let json_content_middleware = JsonAfterMiddleware;

    let mut router = Router::new();
    router.get("/feed", handlers.feed, "feed");
    router.post("/post", handlers.make_post, "make_post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
