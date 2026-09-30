"""A host in Python that embeds Prismal through the prismal-stdio process (HI-6.4).

It loads a program, opens a lab, feeds an input from its own "environment" (here a loop
that switches a thrust on and off), and reads the frames Prismal describes. Any language
that can start a process and read and write lines can do the same.

    cargo build --release -p prismal-stdio
    python crates/prismal-stdio/client.py [path/to/prismal-stdio]
"""

import itertools
import json
import os
import subprocess
import sys

CART = """model Cart {
  const { m: Mass = 2 kg }
  input { thrust: Force = 0 N }
  state { x: Length = 0 m; v: Velocity = 0 m/s }
  flow { der(x) = v; der(v) = thrust / m }
}
presentation Lab for Cart {
  view speed: plot(x: [0 s, 6 s], y: [0 m/s, 10 m/s]) { series_plot(v every 0.1 s) }
  panel status { label(x); label(v) }
}
"""


class Prismal:
    """One engine in a child process; `call` sends a request and waits for its response."""

    def __init__(self, exe):
        self.proc = subprocess.Popen([exe], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, encoding="utf-8")
        self.ids = itertools.count(1)

    def call(self, op, **fields):
        request = {"protocol": 1, "op": op, "id": next(self.ids), **fields}
        self.proc.stdin.write(json.dumps(request) + "\n")
        self.proc.stdin.flush()
        response = json.loads(self.proc.stdout.readline())
        assert response["id"] == request["id"]
        if "error" in response:
            raise RuntimeError(response["error"])
        return response["ok"]

    def close(self):
        self.proc.stdin.close()
        self.proc.wait()


def main():
    default = os.path.join("target", "release", "prismal-stdio" + (".exe" if os.name == "nt" else ""))
    exe = sys.argv[1] if len(sys.argv) > 1 else default
    p = Prismal(exe)
    doc = p.call("load", text=CART)["document"]
    lab = p.call("open", document=doc, presentation="Lab")["instance"]
    # The host's own clock and environment: thrust on for the first two seconds, then off.
    for step in range(7):
        t = float(step)
        p.call("seek", instance=lab, time=t)
        if step in (0, 2):
            p.call("set_input", instance=lab, input="thrust", value=4.0 if step == 0 else 0.0)
        frame = p.call("frame", instance=lab)
        labels = [r["text"] for v in frame["views"] for r in v["reps"] if r["kind"] == "label"]
        print(f"t = {frame['t']:.0f} s: " + ", ".join(labels))
    p.close()


if __name__ == "__main__":
    main()
