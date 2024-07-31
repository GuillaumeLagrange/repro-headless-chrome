use headless_chrome::{Browser, LaunchOptions};

fn main() {
    let _browser = Browser::new(
        LaunchOptions::default_builder()
            .sandbox(false)
            .build()
            .unwrap(),
    )
    .unwrap();

    println!("Hello world!")
}
