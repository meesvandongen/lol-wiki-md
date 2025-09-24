from pathlib import Path
import sys
sys.path.append(str(Path(__file__).resolve().parents[1]))
from simple_converter import SimpleLoLConverter

# Prepare converter with default paths; these may not exist but are optional for this smoke test
conv = SimpleLoLConverter(Path('./out'), Path('./markdown'))

sample = """{{sti|critical damage|{{as|{{ap|((1+({{ccd|Graves|crit_base}}-100)*0.45/100)*(1+5*0.33302)/(1+3*0.33302)*100)|round=3}}% (+ {{ii|Infinity Edge|icononly=true}} {{ap|{{cid|Infinity Edge|critdamage}}*0.45*(1+5*0.33302)/(1+3*0.33302)|round=3}}%)|critical damage}}}}"""

# Use champion context for ccd
print(conv._convert_wiki_to_markdown_ctx(sample, 'Graves'))
