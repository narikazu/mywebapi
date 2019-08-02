use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{Handler, status, IronResult, Response, Request, AfterMiddleware};
use iron::headers::ContentType;
use rustc_serialize::json;
use database::Database;
use uuid::Uuid;
use router::Router;
use model::Post;
use std::error::Error;

pub #[derive(Debug)]
struct Handlers {
    pub feed: FeedHandler,
    pub make_post: MakePostHander,
    pub post: PostHnader,
}

impl Handlers {
    pub fn new(database: Database) -> Handlers {
        let database = Arc::new(Mutex::new(database));
        Handlers {
            feed: FeedHandler::new(database.clone()),
            make_post: MakePostHander::new(database.clone()),
            post: PostHnader::new(database.clone()),
        }
    }
}

pub struct FeedHandler {
  database: Arc<Mutex<Database>>,
}

impl FeedHandler {
  fn new(database: Arc<Mutex<Database>>) -> FeedHandler {
    FeedHandler { database: database }
  }
}

impl Handler for FeedHandler {
  fn handle(&self, _: &mut Request) -> IronResult<Response> {
    let payload = try_handler!(json::encode(lock!(self.database).posts()));
    Ok(Response::with((status::Ok, payload)))
  }
}

pub #[derive(Debug)]
struct MakePostHander {
  database: Arc<Mutex<Database>>,
}

impl MakePostHander {
  fn new(database: Arc<Mutex<Database>>) -> MakePostHander {
    MakePostHander { database: database }
  }
}

impl Handler for MakePostHander {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    try_handler!(req.body.read_to_string(&mut payload));

    let post = try_handler!(json::decode(&payload), status::BadRequest);
    lock!(self.database).add_post(post);
    Ok(Response::with((status::Created, payload)))
  }
}

macro_rules! try_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, e.description())))
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description())))
        }
    }
}

macro_rules! lock {
    ($e:expr) => { $e.lock().unwrap()}
}

macro_rules! get_http_param {
    ($r:expr, $e:expr) => {
        match $r.extensions.get::<Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(val) => val,
                    None => return Ok(Response::with(status::BadRequest)),
                }
            }
            None => return Ok(Response::with(status::InternalServerError)),
        }
    }
}
