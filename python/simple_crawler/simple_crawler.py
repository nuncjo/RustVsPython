# -*- coding:utf-8 -*-

import requests


VERSION = 0.1
URLS = (
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
)


def crawl_all():
    for url in URLS:
        try:
            print(f"Response of url: {url} is {requests.get(url).status_code}")
        except Exception:
            print("Failed to get url.")


if __name__ == "__main__":
    print(f"Example version: {VERSION}")
    crawl_all()
