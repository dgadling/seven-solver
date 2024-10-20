#!/usr/bin/env python3

for w in open("7x7-word-list.txt"):
    if len(w) == 7:
        print(w, end="")
