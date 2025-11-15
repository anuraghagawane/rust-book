use trpl::{Html, Either, StreamExt, Stream, ReceiverStream};
use std::future::Future;
use std::time::{Duration, Instant};
use std::pin::{Pin, pin};
use std::thread;

async fn page_title(url: &str) -> (&str, Option<String>) {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;

    // can chain like this aswell
    let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text) 
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}

// above function can be written like below
fn page_title_n(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text) 
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // trpl::run(async {
    //     let url = &args[1];
    //     match page_title(url).await {
    //         Some(title) => println!("{url} => {title}"),
    //         None => println!("{url} had no title"),
    //     }
    // })

    // trpl::run(async {
    //     let title_fut_1 = page_title(&args[1]);
    //     let title_fut_2 = page_title(&args[2]);
    //
    //     let (url, maybe_title) = 
    //         match trpl::race(title_fut_1, title_fut_2).await {
    //             Either::Left(left) => left,
    //             Either::Right(right) => right,
    //         };
    //
    //     println!("{url} returned first");
    //     match maybe_title {
    //         Some(title) => println!("title => {title}"),
    //         None => println!("No title"),
    //     }
    // });

    // trpl::run(async {
    //     let handle = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("First task {i}");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });
    //
    //     for i in 1..5 {
    //         println!("second task {i}");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    //
    //     handle.await.unwrap();
    // })

    // trpl::run(async {
    //     let fu1 = async {
    //         for i in 1..10 {
    //             println!("First task {i}");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //
    //     let fu2 = async {
    //         for i in 1..5 {
    //             println!("First task {i}");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //
    //     trpl::join(fu1, fu2).await;
    // })

    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel();
    //     // let val = String::from("hi");
    //
    //     let tx_fut = async move {
    //         let vals = vec![
    //             String::from("hi"),
    //             String::from("how"),
    //             String::from("about"),
    //             String::from("you"),
    //         ];
    //
    //         for val in vals {
    //             tx.send(val).unwrap();
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //
    //     let rx_fut = async {
    //         while let Some(received) = rx.recv().await {
    //             println!("received {received}");
    //         }
    //     };
    //     trpl::join(tx_fut, rx_fut).await;

    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel();
    //     // let val = String::from("hi");
    //     let tx1 = tx.clone();
    //     let tx_fut1 = pin!(async move {
    //         let vals = vec![
    //             String::from("hi"),
    //             String::from("how"),
    //             String::from("about"),
    //             String::from("you"),
    //         ];
    //
    //         for val in vals {
    //             tx1.send(val).unwrap();
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });
    //
    //     let rx_fut = pin!(async {
    //         while let Some(received) = rx.recv().await {
    //             println!("received {received}");
    //         }
    //     });
    //
    //     let tx_fut2 = pin!(async move {
    //         let vals = vec![
    //             String::from("hii"),
    //             String::from("howww"),
    //             String::from("abouttttt"),
    //             String::from("youuuuuuu"),
    //         ];
    //
    //         for val in vals {
    //             tx.send(val).unwrap();
    //             trpl::sleep(Duration::from_millis(1500)).await;
    //         }
    //     });
    //
    //     // trpl::join3(tx_fut1, rx_fut, tx_fut2).await;
    //     // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(tx_fut1), Box::pin(rx_fut), Box::pin(tx_fut2)];
    //
    //     let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut1, rx_fut, tx_fut2];
    //     trpl::join_all(futures).await;
    //
    //     let a = async { 1u32 };
    //     let b = async {"Hello!"};
    //     let c = async { true };
    //
    //     let (a_result, b_result, c_result) = trpl::join!(a, b, c);
    //     println!("{a_result}, {b_result}, {c_result}");
    // })

    // trpl::run(async {
    //     let slow = async {
    //         println!("slow started");
    //         trpl::sleep(Duration::from_millis(100)).await;
    //         println!("slow finished");
    //     };
    //
    //     let fast = async {
    //         println!("fast started");
    //         trpl::sleep(Duration::from_millis(50)).await;
    //         println!("fast finished");
    //     };
    //
    //     trpl::race(slow, fast).await;
    // })

    // trpl::run(async {
    //     let one_ms = Duration::from_millis(1);
    //     let a = async {
    //         println!("a started");
    //         slow("a", 30);
    //         trpl::yield_now().await;
    //         slow("a", 10);
    //         trpl::yield_now().await;
    //         slow("a", 20);
    //         trpl::sleep(Duration::from_millis(50)).await;
    //         print!("a finished");
    //     };
    //
    //     let b = async {
    //         println!("b started");
    //         slow("b", 75);
    //         trpl::yield_now().await;
    //         slow("b", 10);
    //         trpl::yield_now().await;
    //         slow("b", 15);
    //         trpl::yield_now().await;
    //         slow("b", 350);
    //         trpl::sleep(Duration::from_millis(50)).await;
    //         print!("b finished");
    //     };
    //
    //     trpl::race(a, b).await;
    // });

    // trpl::run(async {
    //     let one_ns = Duration::from_nanos(1);
    //     let start = Instant::now();
    //     async {
    //         for _ in 1..1000 {
    //             trpl::sleep(one_ns).await;
    //         }
    //     }
    //     .await;
    //     let time = Instant::now() - start;
    //     println!(
    //         "'sleep' version finished after {} seconds.",
    //         time.as_secs_f32()
    //     );
    //
    //     let start = Instant::now();
    //     async {
    //         for _ in 1..1000 {
    //             trpl::yield_now().await;
    //         }
    //     }
    //     .await;
    //     let time = Instant::now() - start;
    //     println!(
    //         "'yield' version finished after {} seconds.",
    //         time.as_secs_f32()
    //     );
    // })

    // trpl::run(async {
    //     let slow = async {
    //         trpl::sleep(Duration::from_secs(1)).await;
    //         "I finished!"
    //     };
    //
    //     match timeout(slow, Duration::from_secs(2)).await {
    //         Ok(message) => println!("Succeded with {message}"),
    //         Err(duration) => {
    //             println!("Failed after {} seconds", duration.as_secs())
    //         }
    //     }
    // })

    // trpl::run(async {
    //     let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //     let iter = values.iter().map(|n| n*2);
    //     let mut stream = trpl::stream_from_iter(iter);
    //
    //     while let Some(value) = stream.next().await {
    //         println!("The value was: {value}");
    //     }
    // });

    // trpl::run(async {
    //     let values = 1..100;
    //     let iter = values.map(|n| n*2);
    //     let stream = trpl::stream_from_iter(iter);
    //
    //     let mut filtered = stream.filter(|val| val % 3 == 0 || val & 5 == 0);
    //
    //     while let Some(value) = filtered.next().await {
    //         println!("The value was: {value}");
    //     }
    // })

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
                .map(|count| format!("Interval: {count}"))
                .throttle(Duration::from_millis(100))
                .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            if let Err(send_error) = tx.send(format!("Message: {message}")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
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
            count+=1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("Cannot send message '{count}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

async fn timeout<F: Future> (
    future_to_try: F,
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(message) => Ok(message),
        Either::Right(_) => Err(max_time)
    }
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms} ms");
}
