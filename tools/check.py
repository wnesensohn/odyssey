#!/usr/bin/env python3
"""Compile, format-check and test the three ODYSSEY components."""
import os
import pathlib
import subprocess
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
ENV = os.environ.copy()
ENV.setdefault("CARGO_TARGET_DIR", str(pathlib.Path(tempfile.gettempdir()) / "odyssey-cargo"))
ENV.setdefault("GOCACHE", str(pathlib.Path(tempfile.gettempdir()) / "odyssey-go-cache"))
ENV.setdefault("CGO_ENABLED", "0")


def run(command, cwd=ROOT):
    subprocess.run(command, cwd=cwd, env=ENV, check=True)


run(["cargo", "fmt", "--manifest-path", "flight/Cargo.toml", "--", "--check"])
run(["cargo", "test", "--manifest-path", "flight/Cargo.toml", "--quiet"])
result = subprocess.run(["gofmt", "-l", "console"], cwd=ROOT, capture_output=True, text=True, check=True)
if result.stdout.strip():
    raise SystemExit("Unformatted Go files: " + result.stdout)
run(["go", "test", "./..."], ROOT / "console")
with tempfile.TemporaryDirectory(prefix="odyssey-erlang-") as temporary:
    sources = sorted((ROOT / "comms/src").rglob("*.erl")) + sorted((ROOT / "comms/test").glob("*.erl"))
    run(["erlc", "-Werror", "-o", temporary, *map(str, sources)])
    tests = sorted(path.stem for path in (ROOT / "comms/test").glob("*_tests.erl"))
    expression = "case eunit:test([" + ",".join(tests) + "],[verbose]) of ok -> halt(0); _ -> halt(1) end."
    run(["erl", "+S", "2", "-noshell", "-pa", temporary, "-eval", expression])
print("ODYSSEY checks passed")
