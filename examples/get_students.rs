extern crate diesel;
extern crate gotham;
extern crate student_service;

#[macro_use]
extern crate gotham_derive;

use diesel::pg::PgConnection;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use student_service::models::{create_student, Student, studetns_to_response};
use student_service::schema::students::dsl::*;
use student_service::*;

#[derive(Clone, StateData)]
struct DBConn {
    inner: Arc<Mutex<PgConnection>>,
}

impl DBConn {
    fn new(conn: PgConnection) -> Self {
        Self {
            inner: Arc::new(Mutex::new(conn)),
        }
    }

    fn get_students(&self) -> Vec<Student> {
        let conn = self.inner.lock().unwrap();
        students
            .limit(10)
            .load::<Student>(&*conn)
            .expect("error load students")
    }
}

fn router() -> Router {
    let conn = DBConn::new(student_service::establish_connection());
    let middleware = StateMiddleware::new(conn);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);
    build_router(chain, pipelines, |route| {
        route.get("/students").to(|state| {
            let stus = DBConn::borrow_from(&state).get_students();
            let resp = studetns_to_response(stus, &state);
            (state, resp)
        });
    })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = "127.0.0.1:7777";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
