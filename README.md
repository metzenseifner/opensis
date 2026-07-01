# sismatic

An open-sourced library for working with the Simple Instruction Set used by many Extron devices.
The name comes from SIS + automatic, because it handles the SIS machinery behind the scenes without human control.

## Why?

There are several reasons why this library is worthwhile:

1. Relies on a stable, unchanging protocol.
2. Hides complexities of managing connections.
3. Hides complexities of byte-level communication.

## Python Consumer Example

The following is an example of how this library can be consumed in Python.

### Examples
#### Configuration

```toml
[defaults]
port = 22023
connect_secs = 5
command_secs = 3

[[device]]
id = "atrium-101"
host = "10.0.0.7"
username = "admin"
password = "extron"

[[device]]
id = "annex-far"
host = "10.0.0.8"
username = "admin"
password = "extron"
connect_secs = 10   # override default connect timeout
command_secs = 8    # override default command timeout
```

#### Iterate over package public properties

```py
>>> from sismatic import Sis
>>> [m for m in dir(Sis) if not m.startswith('_')]
['command', 'from_toml', 'ids', 'query', 'register']
```

#### List Recorders (no network)

```py
from sismatic import Sis

sis = Sis.from_toml("devices.toml")
for device_id in sorted(sis.ids()):
  print(device_id)
```

#### Start a Recording

```py
# control_recording.py
# Starts recording and stamps a title across a batch of devices.

from dataclasses import dataclass
from sismatic import Sis

@dataclass(frozen=True)
class RecordingJob:
    """(device_id × title) as a product type. A job is exactly a pairing of
    the two — never one without the other — so the type says that, instead
    of the two strings being threaded separately through every call site as
    two positional arguments that happen to always travel together."""
    device_id: str
    title: str


def run_job(sis: sismatic, job: RecordingJob) -> None:
    sis.register(job.device_id, "title", job.title)   # "title" — see Register::Title
    sis.command(job.device_id, "start")                # "start" — see Command::Start


def main() -> None:
    sis = Sis.from_toml("devices.toml")
    jobs = [
        RecordingJob(device_id="atrium-101", title="Week 4 — Lecture"),
        RecordingJob(device_id="annex-far", title="Week 4 — Overflow Room"),
    ]
    for job in jobs:
        run_job(sis, job)


if __name__ == "__main__":
    main()
```

