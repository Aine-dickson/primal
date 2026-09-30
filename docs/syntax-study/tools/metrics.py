"""Line and token counts for the syntax study candidates.

For each candidate file and each reference program section (a heading that
starts with "## RP-"), counts the non-blank, non-comment lines and the tokens
of the program code (fenced blocks tagged "text") and, separately, of the run
and expectation code (blocks tagged "cases"). Also reports the maximum brace
or indentation depth reached by program code.

Standard library only. Run from the repository root:

    python docs/syntax-study/tools/metrics.py
"""

import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
STUDY = HERE.parent
CANDIDATES = [("A", "candidate-a.md"), ("B", "candidate-b.md"), ("C", "candidate-c.md"), ("W", "working-syntax.md")]

TOKEN = re.compile(
    r'"[^"]*"'                      # string literal
    r"|\d+(?:\.\d+)?(?:e-?\d+)?"    # number
    r"|[^\W\d]\w*"                  # identifier or keyword (Unicode letters)
    r"|:=|\+=|==|<=|>=|->|\.\.|=>"  # two-character operators
    r"|\S"                          # any other single symbol
)


def strip_comment(line):
    in_string = False
    for i, ch in enumerate(line):
        if ch == '"':
            in_string = not in_string
        elif not in_string and line.startswith("//", i):
            return line[:i]
    return line


def blocks(text):
    """Yield (section, tag, lines) for every fenced block under an RP heading."""
    section = None
    tag = None
    buf = []
    for line in text.splitlines():
        if tag is None:
            m = re.match(r"## (RP-\d\d)", line)
            if m:
                section = m.group(1)
            elif line.startswith("## "):
                section = None
            m = re.match(r"```(\w+)", line)
            if m and section:
                tag, buf = m.group(1), []
        elif line.startswith("```"):
            yield section, tag, buf
            tag = None
        else:
            buf.append(line)


def depth(lines):
    brace = best = 0
    for line in lines:
        code = strip_comment(line)
        indent = (len(code) - len(code.lstrip(" "))) // 2
        if code.strip():
            best = max(best, brace, indent)
        brace += code.count("{") - code.count("}")
    return best


def main():
    rows = {}
    for name, fname in CANDIDATES:
        text = (STUDY / fname).read_text(encoding="utf-8")
        for section, tag, lines in blocks(text):
            code = [strip_comment(l) for l in lines]
            code = [l for l in code if l.strip()]
            tokens = sum(len(TOKEN.findall(l)) for l in code)
            key = (section, name)
            r = rows.setdefault(key, {"lines": 0, "tokens": 0, "clines": 0, "ctokens": 0, "depth": 0})
            if tag == "cases":
                r["clines"] += len(code)
                r["ctokens"] += tokens
            else:
                r["lines"] += len(code)
                r["tokens"] += tokens
                r["depth"] = max(r["depth"], depth(lines))

    names = [n for n, _ in CANDIDATES]
    sections = sorted({s for s, _ in rows})
    out = sys.stdout
    out.write("Program code (model, presentation, timeline): lines / tokens / max depth\n\n")
    out.write("| Program | " + " | ".join(names) + " |\n")
    out.write("|---|" + "---|" * len(names) + "\n")
    totals = {n: [0, 0] for n in names}
    for s in sections:
        cells = []
        for n in names:
            r = rows.get((s, n))
            if r:
                cells.append(f"{r['lines']} / {r['tokens']} / {r['depth']}")
                totals[n][0] += r["lines"]
                totals[n][1] += r["tokens"]
            else:
                cells.append("-")
        out.write(f"| {s} | " + " | ".join(cells) + " |\n")
    out.write("| Total | " + " | ".join(f"{totals[n][0]} / {totals[n][1]}" for n in names) + " |\n\n")

    out.write("Runs and expectations: lines / tokens\n\n")
    out.write("| Program | " + " | ".join(names) + " |\n")
    out.write("|---|" + "---|" * len(names) + "\n")
    for s in sections:
        cells = []
        for n in names:
            r = rows.get((s, n))
            cells.append(f"{r['clines']} / {r['ctokens']}" if r and r["clines"] else "-")
        out.write(f"| {s} | " + " | ".join(cells) + " |\n")


if __name__ == "__main__":
    main()
