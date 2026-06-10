import urllib.parse, urllib.request, json, sys

API = "https://wiki.leagueoflegends.com/en-us/api.php"

def members(cat, ns=None):
    out = []
    cont = None
    while True:
        params = {"action":"query","format":"json","list":"categorymembers",
                  "cmtitle":cat,"cmlimit":"500","cmtype":"page"}
        if cont: params["cmcontinue"] = cont
        url = API + "?" + urllib.parse.urlencode(params)
        req = urllib.request.Request(url, headers={"User-Agent":"lol-wiki-md-test-mining/1.0"})
        with urllib.request.urlopen(req, timeout=30) as r:
            d = json.load(r)
        for m in d.get("query",{}).get("categorymembers",[]):
            t = m["title"]
            if ns is not None and m.get("ns") != ns: continue
            out.append(t)
        c = d.get("continue",{}).get("cmcontinue")
        if not c: break
        cont = c
    return out

cats = sys.argv[1:] or ["Category:Champions","Category:Items","Category:Runes"]
seen = set(); titles = []
for c in cats:
    ms = members(c)
    print(f"# {c}: {len(ms)}", file=sys.stderr)
    for t in ms:
        # skip subpages/odd namespaces
        if ":" in t and not t.startswith(("Category:",)):
            pass
        if t not in seen:
            seen.add(t); titles.append(t)
print("\n".join(titles))
