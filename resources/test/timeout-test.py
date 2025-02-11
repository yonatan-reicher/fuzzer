#!/usr/bin/env python3

from time import sleep
import sys


if __name__ == '__main__':
    a = sys.stdin.read()
    print(a)
    if 'a' in a:
        exit(1)
    else:
        sleep(10)
