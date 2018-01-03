extern crate reqwest;
use std::thread;

static VERSION: f32 = 0.1;


fn crawl_worker(url: &str) {
    let parsed_url = reqwest::Url::parse(url).expect("Bad url format.");
    let response = reqwest::get(parsed_url).expect("Failed to get url.");
    println!("Response of url: {} is {:?}", url, response.status().to_string());
}


fn main() {
    println!("Example version: {}", VERSION);
    let urls = vec![
        "https://httpbin.org/html",
        "https://httpbin.org/links/10/0",
        "https://httpbin.org/robots.txt",
        "https://www.w3schools.com/html/html_tables.asp",
        "https://httpbin.org/user-agent",
        "https://www.nytimes.com/",
        "https://httpbin.org/forms/post",
        "https://www.google.pl/",
        "https://httpbin.org/links/10/0",
        "https://httpbin.org/robots.txt",
        "https://httpbin.org/xml",
        "https://httpbin.org/redirect/1",
        "https://httpbin.org/redirect/2",
        "https://httpbin.org/cookies",
        "https://httpbin.org/basic-auth/user/passwd",
        "https://httpbin.org/gzip",
        "https://jsonplaceholder.typicode.com/posts",
        "http://quotes.toscrape.com/",
        "http://books.toscrape.com/"
    ];
    let mut queue = vec![];

    for url in urls {
        queue.push(thread::spawn(move || {
            crawl_worker(url);
        }));
    }

    for job in queue {
        let _ = job.join();
    }
}
