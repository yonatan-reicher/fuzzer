#!/usr/bin/env python3

from time import sleep


if __name__ == '__main__':
    a = ''
    try:
        a = input()
    except:
        pass
    if 'a' in a:
        exit(1)
    else:
        sleep(10)
