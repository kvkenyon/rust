use std::{pin::pin, time::Duration};
use trpl::Html;
use trpl::{ReceiverStream, Stream, StreamExt};

pub async fn test() {
    println!("[ASYNC] start...");
    let url = String::from("https://hf.app");
    let result = page_title(&url).await;
    match result {
        Some(title) => println!("title: {title}"),
        None => println!("{} not found", url),
    }

    // message_passing().await;

    // message_passing_2().await;
    // message_passing_3().await;

    // message_passing_4().await;
    run_stream().await;
    println!("[ASYNC] end...");
}

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

async fn message_passing() {
    let (tx, mut rx) = trpl::channel();

    let msg = String::from("Hi!");

    tx.send(msg).unwrap();

    let result = rx.recv().await.unwrap();
    println!("recv: {result}");
}

async fn message_passing_2() {
    let (tx, mut rx) = trpl::channel();

    let vals = vec![
        String::from("Hi!"),
        String::from("This"),
        String::from("is"),
        String::from("a"),
        String::from("message!"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    }

    while let Some(val) = rx.recv().await {
        println!("{val}");
    }
}

async fn message_passing_3() {
    let (tx, mut rx) = trpl::channel();

    let tx_fut = async move {
        let vals = vec![
            String::from("Hi!"),
            String::from("This"),
            String::from("is"),
            String::from("a"),
            String::from("message!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(val) = rx.recv().await {
            println!("{val}");
        }
    };

    trpl::join(tx_fut, rx_fut).await;
}

async fn message_passing_4() {
    let (tx, mut rx) = trpl::channel();

    let tx2 = tx.clone();
    let tx_fut = async move {
        let vals = vec![
            String::from("Hi!"),
            String::from("This"),
            String::from("is"),
            String::from("a"),
            String::from("message!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let tx2_fut = async move {
        let vals = vec![
            String::from("WOW"),
            String::from("IM"),
            String::from("INterleaving"),
            String::from("a"),
            String::from("message!"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(msg) = rx.recv().await {
            println!("got: {msg}");
        }
    };

    trpl::join!(tx_fut, tx2_fut, rx_fut);
}

//async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
//    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
//        trpl::Either::Left(val) => Ok(val),
//        trpl::Either::Right(_) => Err(max_time),
//    }
//}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (i, msg) in messages.into_iter().enumerate() {
            let delay = if i % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(delay)).await;
            tx.send(format!("Message: {msg}")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });
    ReceiverStream::new(rx)
}

async fn run_stream() {
    //let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));
    let messages = get_messages().timeout(Duration::from_millis(200));
    let intervals = get_intervals()
        .map(|count| format!("Interval: {count}"))
        .throttle(Duration::from_millis(100))
        .timeout(Duration::from_secs(10));

    let merged = messages.merge(intervals).take(20);

    let mut stream = pin!(merged);

    while let Some(result) = stream.next().await {
        match result {
            Ok(val) => println!("received: {val}"),
            Err(to) => eprintln!("Problem: {to:?}"),
        }
    }
}
