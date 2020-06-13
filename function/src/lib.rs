#[macro_use]
extern crate serde_json;
use warp::{Filter, filters::BoxedFilter, Reply};

pub fn main() -> BoxedFilter<(impl Reply,)> {
  warp::get()
    .and(warp::header("user-agent"))
    .map(|agent: String| {
      let message = format!("Hello world! Your user agent is {}", agent);
      json!({"message": message}).to_string()
    })
    .boxed()
}