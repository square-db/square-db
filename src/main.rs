use warp::Filter;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RequestParams {
  username: String,
  password: String,
  query: String,
}

#[tokio::main]
async fn main() {
  let api = warp::post()
  .and(warp::path("db"))
  .and(warp::body::json())
  .map(|params: RequestParams| {
    println!("{:?}", params.username);
    println!("{:?}", params.password);
    println!("{:?}", params.query);
    warp::reply::json(&params)
  });
  
  //change for production mode
  warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}