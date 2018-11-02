use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use hyper::{Body, Response, StatusCode};
use schema::students;

use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use mime;
use serde_json;

#[derive(Queryable, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
    pub hobby: Option<String>,
    pub graduated: bool,
}

#[derive(Insertable)]
#[table_name = "students"]
pub struct NewStudent<'a> {
    pub name: &'a str,
    pub age: Option<i32>,
    pub hobby: Option<String>,
    pub graduated: bool,
}

pub fn create_student<'a>(
    conn: &PgConnection,
    name: &'a str,
    age: i32,
    hobby: Option<&'a str>,
) -> Student {
    use schema::students;

    let new_student = NewStudent {
        name: name,
        age: Some(age),
        hobby: hobby.map(|s| s.to_string()),
        graduated: true,
    };

    insert_into(students::table)
        .values(&new_student)
        .get_result(conn)
        .expect("Error saving new post")
}

impl IntoResponse for Student {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized student"),
        )
    }
}

pub fn studetns_to_response(students: Vec<Student>, state: &State) -> Response<Body> {
    create_response(
        state,
        StatusCode::OK,
        mime::APPLICATION_JSON,
        serde_json::to_string(&students).expect("serialized student"),
    )
}
