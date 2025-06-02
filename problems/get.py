#!/usr/bin/env python3
"""
Download Project Euler “minimal” pages 130 – 300
and save each to problem_<n>.txt
"""

import os
import time
import random
import pathlib
import requests

START = 130          # inclusive
END   = 300          # inclusive
DEST  = pathlib.Path(".")   # change if you want another folder
PAUSE_RANGE = (2, 4)        # seconds, (min, max)

HEADERS = {
    # Using a browser‑like User‑Agent avoids some 403 blocks
    "User-Agent": (
        "Mozilla/5.0 (X11; Linux x86_64) "
        "AppleWebKit/537.36 (KHTML, like Gecko) "
        "Chrome/124.0 Safari/537.36"
    )
}

def fetch_problem(n: int) -> str:
    """Return raw text of minimal page n, or raise for HTTP errors."""
    url = f"https://projecteuler.net/minimal={n}"
    response = requests.get(url, headers=HEADERS, timeout=15)
    response.raise_for_status()
    return response.text

def save(text: str, n: int) -> pathlib.Path:
    """Save text to problem_<n>.txt and return the path."""
    path = DEST / f"problem_{n}.txt"
    path.write_text(text, encoding="utf-8")
    return path

def main():
    DEST.mkdir(parents=True, exist_ok=True)

    for n in range(START, END + 1):
        try:
            html = fetch_problem(n)
        except requests.RequestException as exc:
            print(f"[{n}] ⚠️  download failed: {exc}")
        else:
            out = save(html, n)
            print(f"[{n}] ✅  saved to {out}")

        # Random pause to be gentle on the server
        delay = random.uniform(*PAUSE_RANGE)
        time.sleep(delay)

if __name__ == "__main__":
    main()

