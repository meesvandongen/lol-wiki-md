#!/usr/bin/env python3
from pathlib import Path
from simple_converter import SimpleLoLConverter

if __name__ == "__main__":
    conv = SimpleLoLConverter(Path('./out'), Path('./markdown'))
    sample = "{{sbc|''{{ai|Takedown|Nidalee}}'' scales with ''{{ais|Aspect of the Cougar|Nidalee}}'' rank}}"
    print(conv._convert_wiki_to_markdown(sample))
