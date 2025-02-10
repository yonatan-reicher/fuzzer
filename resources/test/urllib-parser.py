#!/usr/bin/env python3

from urllib.parse import urlparse


if __name__ == "__main__":
    url = input("Enter a URL: ")
    parse_result = urlparse(url)
    keys = parse_result._fields
    for key in keys:
        print(f"{key}: {getattr(parse_result, key)}")
