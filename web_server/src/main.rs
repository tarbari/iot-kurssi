use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Listening to port 8085");
    let index = warp::path::end()
        .and(warp::fs::file("static/index.html"))
        .boxed(); // Box the filter to ensure it resolves to a Reply

    let routes = index;

    warp::serve(routes).run(([0, 0, 0, 0], 8085)).await;
}
