#!/usr/bin/env python3
"""Harvest self-contained template invocations from a wiki export for use as
differential test cases. Emits `validation_reports/test-mining/candidates.tsv`
with `group<TAB>input<TAB>wiki_plain`, rendering each on the wiki (cached).

Self-contained = no page/champion context needed: we drop invocations that
reference template params ({{{..}}}), variables (#var), data lookups (ccd/cid),
#invoke, subst, HTML comments, refs, or that span multiple lines.

Usage: python3 harvest.py <export_dir> [--per N]
"""
import sys, os, re, collections, urllib.parse
import mwparserfromhell as mw
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import batch_render as br  # reuse render + cache

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "..", "validation_reports", "test-mining", "candidates.tsv")

# group -> (set of normalized template names that map to it)
GROUPS = {
    "ap":     {"ap"},
    "pp":     {"pp"},
    "fd":     {"fd"},
    "expr":   {"#expr"},
    "if":     {"#if"},
    "ifeq":   {"#ifeq"},
    "switch": {"#switch"},
    "tt":     {"tt"},
    # broader families (issue-hunting; exact matches still become tests)
    "pptooltip":     {"pptooltip"},
    "as":            {"as"},
    "sti":           {"sti"},
    "dv":            {"dv", "delimit values"},
    "ft":            {"ft"},
    "color":         {"color"},
    "rd":            {"rd"},
    "sbc":           {"sbc"},
    "numbersup":     {"numbersup"},
    "minutedisplay": {"minutedisplay"},
    "recurring":     {"recurring"},
    "aug":           {"aug"},
    "adaptive":      {"adaptive"},
    "ccs":           {"ccs"},
    "lll":           {"lll"},
    "gold":          {"gold"},
}
NAME_TO_GROUP = {n: g for g, names in GROUPS.items() for n in names}

BAD = ("{{{", "#var", "{{ccd", "{{cid", "#invoke", "subst:", "<!--", "<ref",
       "#ifexist", "#expr:", "{{#tag", "PAGENAME")
# note: for the #expr group the name itself contains "#expr:", handled separately

def norm(n):
    return re.sub(r"\s+", " ", n.strip().replace("_", " ")).lower()

def acceptable(s, group):
    if "\n" in s or len(s) > 160:
        return False
    # forbid context-dependent constructs (but a parser-fn's own prefix is ok)
    low = s
    for b in BAD:
        if b == "#expr:" and group == "expr":
            continue
        if b in low:
            return False
    return True

def main():
    export = sys.argv[1]
    per = 60
    if "--per" in sys.argv:
        per = int(sys.argv[sys.argv.index("--per")+1])
    caps = {"ap":170,"pp":130,"fd":95,"expr":130,"if":30,"ifeq":30,"switch":15,"tt":35,
            "pptooltip":25,"as":40,"sti":30,"dv":25,"ft":25,"color":25,"rd":35,"sbc":25,
            "numbersup":20,"minutedisplay":15,"recurring":20,"aug":20,"adaptive":20,
            "ccs":20,"lll":15,"gold":20}

    buckets = collections.defaultdict(list)
    seen = collections.defaultdict(set)
    files = [f for f in os.listdir(export) if f.endswith(".txt")]
    for f in files:
        try:
            raw = open(os.path.join(export, f), encoding="utf-8").read()
        except Exception:
            continue
        try:
            code = mw.parse(raw)
        except Exception:
            continue
        for t in code.filter_templates():
            name = norm(str(t.name))
            # parser fns: name may be like "#expr:1+2*3"
            base = name.split(":")[0] if name.startswith("#") else name
            group = NAME_TO_GROUP.get(name) or NAME_TO_GROUP.get(base)
            if not group:
                continue
            s = str(t)
            if not acceptable(s, group):
                continue
            if s in seen[group]:
                continue
            if len(buckets[group]) >= caps.get(group, per):
                continue
            seen[group].add(s)
            buckets[group].append(s)

    import json, time
    cache = {}
    if os.path.exists(br.CACHE):
        cache = json.load(open(br.CACHE))
    rows = []
    for group, items in buckets.items():
        for s in items:
            if s in cache:
                plain = cache[s]
            else:
                try:
                    plain = br.strip_html(br.render(s))
                except Exception:
                    continue
                cache[s] = plain
                time.sleep(0.12)
            if not plain or "\n" in plain or len(plain) > 200:
                continue
            rows.append((group, s, plain))
    os.makedirs(os.path.dirname(br.CACHE), exist_ok=True)
    json.dump(cache, open(br.CACHE,"w"), ensure_ascii=False)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as fh:
        for group, s, plain in rows:
            fh.write(f"{group}\t{s}\t{plain}\n")
    print("harvested", len(rows), "candidates ->", OUT)
    for g in caps:
        print(f"  {g}: {sum(1 for r in rows if r[0]==g)}")

if __name__ == "__main__":
    main()
