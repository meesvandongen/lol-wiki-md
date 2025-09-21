import os
import re
from collections import Counter

ROOT = os.path.join(os.path.dirname(__file__), '..', 'out')

# Keep in sync with simple_converter.py handlers
IMPLEMENTED = {
    # Core handlers in converter
    'ap','pp','fd','tt','as','st','sbc','sti','ci','cis','ai','ais','bi','ui','tip','ri','ii','tft item','w','sm2','bug','champion without ability power ratio',
    # Extra mappings added
    'tiplor','lor','tiptft','wrtip','csl','si','cai','cid','iis','stil','g','gold value','times','degree','plus','tftt',
    'references','lol navigation','champions','champion categories','doc','fairuse','section top','rune header','rune footer','game banner','patch box','!',
    'rd','nie','spells','recurring','effect at cast time end','tftemblemdesc','pending for test','rutngt','a','ct'
}

TEMPLATE_RE = re.compile(r"\{\{([^{}\r\n]+)\}\}")


def iter_files(root):
    for base, _dirs, files in os.walk(root):
        for f in files:
            p = os.path.join(base, f)
            # Consider only text-y files
            if not f.lower().endswith(('.txt', '.wikitext', '.wiki', '.md', '.lua', '.json', '.xml')):
                continue
            yield p


def extract_template_names(text):
    names = []
    for m in TEMPLATE_RE.finditer(text):
        body = m.group(1)
        name = body.split('|', 1)[0].strip().lower()
        if name:
            names.append(name)
    return names


def main():
    if not os.path.isdir(ROOT):
        print(f"out/ folder not found at: {ROOT}")
        return 1
    counter = Counter()
    file_count = 0
    for path in iter_files(ROOT):
        try:
            with open(path, 'r', encoding='utf-8', errors='ignore') as fh:
                text = fh.read()
        except Exception:
            continue
        names = extract_template_names(text)
        counter.update(names)
        file_count += 1
    total = sum(counter.values())
    print(f"Scanned {file_count} files, found {total} template invocations, {len(counter)} unique names.\n")

    def is_handled(name: str) -> bool:
        if name in IMPLEMENTED:
            return True
        # Drop numeric-only helpers like '{{1}}', '{{2}}', etc.
        if name.isdigit():
            return True
        # Drop templates containing '<noinclude>' fragments
        if '<noinclude>' in name:
            return True
        # Drop cargo/scribunto invocations
        if name.startswith('#var:') or name.startswith('#invoke:'):
            return True
        return False

    unhandled = [(n, c) for n, c in counter.most_common() if not is_handled(n)]
    print("Top 50 unhandled templates:")
    for n, c in unhandled[:50]:
        print(f"{n}\t{c}")

    return 0


if __name__ == '__main__':
    raise SystemExit(main())
