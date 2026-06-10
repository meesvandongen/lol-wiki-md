#!/usr/bin/env python3
"""Render arbitrary wikitext via the LoL wiki API and return plain text.

Authoritative ground truth for differential template tests (see AGENTS.md).
Usage: echo '{{ap|40|50|60}}' | python3 render.py
   or: python3 render.py '{{ap|40|50|60}}'
"""
import sys, json, urllib.parse, urllib.request, re, html

API = "https://wiki.leagueoflegends.com/en-us/api.php"

def render(wikitext: str) -> str:
    params = {
        "action": "parse",
        "format": "json",
        "contentmodel": "wikitext",
        "prop": "text",
        "disablelimitreport": "1",
        "wrapoutputclass": "",
        "text": wikitext,
    }
    data = urllib.parse.urlencode(params).encode()
    req = urllib.request.Request(API, data=data, headers={"User-Agent": "lol-wiki-md-test-mining/1.0"})
    with urllib.request.urlopen(req, timeout=30) as r:
        payload = json.load(r)
    return payload["parse"]["text"]["*"]

def strip_html(h: str) -> str:
    # drop scripts/styles
    h = re.sub(r"<(script|style)[^>]*>.*?</\1>", "", h, flags=re.S)
    # mw-parser-output wrapper -> keep inner
    # turn <br> into newline
    h = re.sub(r"<br\s*/?>", "\n", h)
    # strip tags
    h = re.sub(r"<[^>]+>", "", h)
    h = html.unescape(h)
    return h.strip()

if __name__ == "__main__":
    wt = sys.argv[1] if len(sys.argv) > 1 else sys.stdin.read()
    raw = render(wt)
    print("=== RAW HTML ===")
    print(raw.strip())
    print("=== PLAIN ===")
    print(strip_html(raw))
