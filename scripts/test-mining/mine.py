#!/usr/bin/env python3
"""Mine template usage from a downloaded wiki export.

Scans every *.txt page, parses with mwparserfromhell, reports per-template
frequency, nesting combinations (parent->child), and diverse real invocations.

Usage: python3 mine.py <export_dir> [--top N]
"""
import sys, os, json, urllib.parse, collections, re
import mwparserfromhell as mw

HERE = os.path.dirname(os.path.abspath(__file__))

def decode_name(fname):
    base = fname[:-4] if fname.endswith(".txt") else fname
    return urllib.parse.unquote(base)

def norm_name(n):
    return re.sub(r"\s+", " ", n.strip().replace("_", " "))

def main():
    export_dir = sys.argv[1]
    top = 60
    if "--top" in sys.argv:
        top = int(sys.argv[sys.argv.index("--top") + 1])

    freq = collections.Counter()
    nesting = collections.Counter()
    samples = collections.defaultdict(list)
    sample_seen = collections.defaultdict(set)
    pages_with = collections.defaultdict(set)
    page_count = 0
    parse_errors = []

    files = [f for f in os.listdir(export_dir) if f.endswith(".txt")]
    for f in files:
        path = os.path.join(export_dir, f)
        title = decode_name(f)
        try:
            raw = open(path, encoding="utf-8").read()
        except Exception as e:
            parse_errors.append((title, "read: %s" % e)); continue
        try:
            code = mw.parse(raw)
        except Exception as e:
            parse_errors.append((title, "parse: %s" % e)); continue
        page_count += 1
        for t in code.filter_templates():
            name = norm_name(str(t.name))
            if not name:
                continue
            freq[name] += 1
            pages_with[name].add(title)
            for child in t.params:
                for ct in child.value.filter_templates(recursive=False):
                    cn = norm_name(str(ct.name))
                    if cn:
                        nesting[(name, cn)] += 1
            sig = (len(t.params),)
            if sig not in sample_seen[name] and len(samples[name]) < 8:
                inv = str(t).replace("\n", "\\n")
                if len(inv) <= 400:
                    samples[name].append(inv)
                    sample_seen[name].add(sig)

    out = {
        "page_count": page_count,
        "unique_templates": len(freq),
        "top_templates": freq.most_common(top),
        "top_nesting": [
            {"parent": p, "child": c, "count": n}
            for (p, c), n in nesting.most_common(120)
        ],
        "samples": {k: samples[k] for k, _ in freq.most_common(top)},
        "pages_with_examples": {k: sorted(list(pages_with[k]))[:5] for k, _ in freq.most_common(top)},
        "parse_errors": parse_errors[:50],
    }
    out_path = os.path.join(HERE, "..", "..", "validation_reports", "test-mining", "mining.json")
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    json.dump(out, open(out_path, "w"), indent=2, ensure_ascii=False)
    print("pages=%d unique_templates=%d parse_errors=%d" % (page_count, len(freq), len(parse_errors)))
    print("--- top 50 templates ---")
    for name, n in freq.most_common(50):
        print("%6d  %s" % (n, name))
    print("--- top 30 nesting (parent -> child) ---")
    for (p, c), n in nesting.most_common(30):
        print("%6d  %s  ->  %s" % (n, p, c))

if __name__ == "__main__":
    main()
