#!/usr/bin/env python3
"""
Scrape Project Euler ‘Archives’ pages for [id, name, difficulty%]
and write euler_difficulty.csv (id‑empotent & resume‑friendly).

© 2025  –  Needs:  requests  beautifulsoup4  tqdm
"""

from __future__ import annotations
import csv, random, time, re, pathlib, sys
from typing import Iterator, Tuple, List

import requests
from bs4 import BeautifulSoup
from tqdm import tqdm

BASE          = "https://projecteuler.net/archives;page={}"   # semicolon, not ampersand
OUTFILE       = pathlib.Path("euler_difficulty.csv")
HEADERS       = {
    "User-Agent": (
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 "
        "(KHTML, like Gecko) Chrome/124.0 Safari/537.36"
    )
}
SLEEP_RANGE   = (1.0, 2.5)           # be polite
WIDTH_RE      = re.compile(r"width:\s*([0-9.]+)%")

# ─────────────────────────────────────────────────────────────────────────────
def grab(page: int) -> BeautifulSoup:
    """Return a BeautifulSoup object for one Archives page."""
    url  = BASE.format(page)
    resp = requests.get(url, headers=HEADERS, timeout=20)
    if resp.status_code == 404:
        raise IndexError("no such page")          # stops the crawl
    resp.raise_for_status()
    return BeautifulSoup(resp.text, "lxml")

def parse(soup: BeautifulSoup) -> List[Tuple[int, str, int]]:
    """Extract triples (id, name, difficulty%) from one page."""
    rows = soup.select("table#problems_table tbody tr")
    out: List[Tuple[int, str, int]] = []
    for tr in rows:
        # 1. ID
        id_cell = tr.select_one("td.id_column")
        if not id_cell:
            continue
        pid = int(id_cell.get_text(strip=True))

        # 2. Name  (title cell usually next one)
        name_cell = tr.find_all("td")[1]
        title = name_cell.get_text(" ", strip=True)

        # 3. Difficulty % (from inner div style)
        diff_cell = tr.select_one("td.difficulty_column div.progress_bar_block")
        if diff_cell and diff_cell.has_attr("style"):
            m = WIDTH_RE.search(diff_cell["style"])
            pct = int(float(m.group(1))) if m else None
        else:
            pct = None                      # should never happen

        out.append((pid, title, pct))
    return out

def existing_ids() -> set[int]:
    if not OUTFILE.exists():
        return set()
    with OUTFILE.open(newline="", encoding="utf-8") as fh:
        return {int(r["problem_id"]) for r in csv.DictReader(fh)}

def write(rows: List[Tuple[int, str, int]], mode: str) -> None:
    with OUTFILE.open(mode, newline="", encoding="utf-8") as fh:
        w = csv.writer(fh)
        if mode == "w":
            w.writerow(["problem_id", "problem_name", "difficulty_percent"])
        w.writerows(rows)
        fh.flush()

# ─────────────────────────────────────────────────────────────────────────────
def main() -> None:
    done = existing_ids()
    mode = "a" if done else "w"

    page = 1
    pbar = tqdm(desc="Scraping pages", unit="page")
    try:
        while True:
            soup  = grab(page)
            triples = [t for t in parse(soup) if t[0] not in done]
            if triples:
                write(triples, mode)
                mode = "a"          # headers written only once
            pbar.update()
            page += 1
            time.sleep(random.uniform(*SLEEP_RANGE))
    except IndexError:
        pass                # ran past the last archives page
    finally:
        pbar.close()

    print(f"All done – CSV at {OUTFILE.resolve()}")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        sys.exit("\nInterrupted – partial CSV preserved.")

