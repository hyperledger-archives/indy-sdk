#!/usr/bin/env python
import json
import time


TIME_OFF_SET = 1515500000


def main():
    return (int(time.time() - TIME_OFF_SET))

if __name__ == '__main__':
    main()

