# toss

Simple peer-to-peer file transfer over [iroh](https://iroh.computer). No server, no account — just a direct connection between two machines.

## How it works

1. The sender loads a file into memory and hosts it on an iroh endpoint.
2. iroh gives back a **ticket** (a long string) that encodes how to reach that endpoint directly.
3. The receiver pastes the ticket, iroh connects peer-to-peer, and the file is downloaded and saved to disk.

No file ever touches a third-party server — the ticket is just connection info.

## Install / Build

```bash
git clone <your-repo-url>
cd toss
cargo build --release
```

The binary will be at `target/release/toss`.

## Usage

**Send a file:**

```bash
toss -s <filename>
```

Prints a ticket. Keep the process running — it stays alive to serve the file until you `Ctrl+C` or the transfer completes.

**Receive a file:**

```bash
toss -r <ticket> <output-filename>
```

Connects to the sender and saves the file locally as `<output-filename>`.

**Help:**

```bash
toss -h
```

## Example

Sender:
```bash
$ toss -s report.pdf
Share this exact passcode ticket:

blobabc123...
```

Receiver:
```bash
$ toss -r blobabc123... report_copy.pdf
  Success! Stored incoming data as 'report_copy.pdf'
```

## Notes

- The receiver will refuse to overwrite an existing file with the same output name.
- The sender must stay online/running until the receiver finishes downloading — this is a live peer-to-peer transfer, not a store-and-forward system.
- The whole file is currently read into memory (`std::fs::read`) before sending, so very large files will use a lot of RAM. Fine for typical use; worth knowing before tossing a 10GB video.

## License

Add a license of your choice (MIT/Apache-2.0 are common for Rust projects).