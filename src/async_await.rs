use trpl::Html;

pub async fn test() {
    println!("[ASYNC] start...");
    let url = String::from("https://hf.app");
    let result = page_title(&url).await;
    match result {
        Some(title) => println!("title: {title}"),
        None => println!("{} not found", url),
    }
    println!("[ASYNC] end...");
}

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}
