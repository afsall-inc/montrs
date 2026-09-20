#!/usr/bin/env python3
import re
import sys

if len(sys.argv) < 2:
    print("Usage: autofix_prdoc.py <path_to_prdoc>", file=sys.stderr)
    sys.exit(1)

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as f:
    content = f.read()

if re.search(r"^crates:\s*$", content, re.MULTILINE):
    content = re.sub(
        r"^crates:\s*$",
        "crates:\n  - name: montrs-cli\n    bump: patch",
        content,
        flags=re.MULTILINE,
    )
elif not re.search(r"^- name:", content):
    content = content.rstrip()
    if content.endswith("---"):
        content = (
            content[:-3]
            + "crates:\n  - name: montrs-cli\n    bump: patch\n---"
        )
    else:
        content += "\ncrates:\n  - name: montrs-cli\n    bump: patch\n"

with open(path, "w", encoding="utf-8") as f:
    f.write(content)
