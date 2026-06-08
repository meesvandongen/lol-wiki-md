#!/usr/bin/env python3
"""Render many wikitext snippets via the wiki API and print `INPUT\\tPLAIN`.

Reads snippets one per line from a file or stdin. Caches to render_cache.json.
"""
import sys, os, json, re, html, time, urllib.parse, urllib.request

API = "https://wiki.leagueoflegends.com/en-us/api.php"
HERE = os.path.dirname(os.path.abspath(__file__))
CACHE = os.path.join(HERE, "render_cache.json")

def strip_html(h):
    h = re.sub(r"<(script|style)[^>]*>.*?</\1>", "", h, flags=re.S)
    h = re.sub(r"<br\s*/?>", "\n", h)
    h = re.sub(r"<[^>]+>", "", h)
    return html.unescape(h).strip()

def render(wt):
    params = {"action":"parse","format":"json","contentmodel":"wikitext",
              "prop":"text","disablelimitreport":"1","wrapoutputclass":"","text":wt}
    data = urllib.parse.urlencode(params).encode()
    req = urllib.request.Request(API, data=data, headers={"User-Agent":"lol-wiki-md-test-mining/1.0"})
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=30) as r:
                return json.load(r)["parse"]["text"]["*"]
        except Exception as e:
            if attempt==3: raise
            time.sleep(2*(attempt+1))

def main():
    cache = {}
    if os.path.exists(CACHE):
        cache = json.load(open(CACHE))
    src = open(sys.argv[1]) if len(sys.argv)>1 else sys.stdin
    lines = [l.rstrip("\n") for l in src if l.strip() and not l.startswith("#")]
    for wt in lines:
        if wt not in cache:
            cache[wt] = strip_html(render(wt))
            time.sleep(0.15)
        print("%s\t%s" % (wt, cache[wt].replace("\n","\\n")))
    json.dump(cache, open(CACHE,"w"), indent=0, ensure_ascii=False)

if __name__ == "__main__":
    main()
