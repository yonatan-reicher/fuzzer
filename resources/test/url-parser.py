#!/usr/bin/env python3
'''
This is a URL parser to test our URL fuzzer!

What is a URL? Ask [mozilla](https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics/What_is_a_URL)
'''


def parse_and_print(url: str):
    if '://' in url:
        scheme, url = url.split('://')
        print(f"Scheme: {scheme}")

    if '#' in url:
        url, anchor = url.split('#')
        print(f"Anchor: {anchor}")
    if '?' in url:
        url, query = url.split('?')
        print(f"Query: {query}")

    if '/' in url:
        authority, path = url.split('/', 1)
        print(f"Domain: {authority}")
        print(f"Path: {path}")


def main():
    url = input('Enter a URL: ')
    parse_and_print(url)


if __name__ == '__main__':
    main()

