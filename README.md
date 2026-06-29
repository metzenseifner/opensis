# OpenSIS

An open-sourced library for working with the Simple Instruction Set used by many Extron devices.

## Why?

There are several reasons why this library is worthwhile:

1. Relies on a stable, unchanging protocol.
2. Hides complexities of managing connections.
3. Hides complexities of byte-level communication.

## Python Consumer Example

The following is an example of how this library can be consumed in Python.

```py
from smp_pool import PyRecorders

rec = PyRecorders(config_path="devices.toml")
dev = "10.0.0.7"

# Metadata must be set BEFORE start; only legal while Idle.
rec.set_metadata(dev, "Title", "Calculus 101 — Lecture 4")
rec.set_metadata(dev, "Presenter", "Dr. Dre")

rec.start(dev)          # flushes metadata, then Y1RCDR
rec.pause(dev)          # Y2RCDR (toggle) — metadata untouched
rec.resume(dev)         # Y2RCDR (toggle) — metadata untouched
rec.stop(dev)           # Y0RCDR — metadata reset to empty

# Illegal: setting metadata mid-recording raises IllegalTransition.
rec.start(dev)          # would raise MetadataRequired now (register cleared)

# Power user — owns the response, may need resync() afterward.
raw = rec.raw_sis(dev, "Q")   # firmware version, unparsed
```
