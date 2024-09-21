// 使用async关键字声明异步函数
async fn some_sync_function() -> i8 {
    unimplemented!()
}

async fn do_something() {
    // 在异步函数中，使用.await调用异步函数
    // 用.await时，它会暂时停止执行并检索实际结果值
    let result = some_sync_function().await;
    println!("result is {}", result);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    do_something().await;

    // async operations
    let contents = tokio::fs::read("Cargo.toml").await;

    // concurrency
    let weather = reqwest::get("https://hello/forecast");
    let news = reqwest::get("https://world/topstories");
    let (weather, news) = tokio::join!(weather, news);

    Ok(())
}
