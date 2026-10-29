# quiklean

An automated file cleaner for Linux which can cleanup unneeded files from trash, and eventually, other directories

## The Problem

Often times, you leave temporary files in places without permanently cleaning them from your system and freeing up space. It's time-consuming to micromanage many directories and delete files which are not needed anymore; it would be far more convenient if files could be deleted automatically after a certain period of time.

## How It Works

![System diagram](docs/System%20Diagram.png)

## Tech Stack

- **Language:** Rust
- **Libraries:** chrono, serde_json
- **External tools:** Neovim, Git, systemd (planned)

## Usage

*Note: Rust must be installed on your machine for this program to work.*

```bash
git clone https://github.com/wAABBsif/quiklean # Clone repo
cd quiklean # Go into the repo
cargo run # Run the program (debug mode)
```

## Roadmap

- [ ] Support cleaning of arbitrary
folders outside of just trash
- [ ] Add different cleaning modes
using file modified, accessed,
and created dates
- [ ] Add different removal modes
such as sending files to trash or
shredding so they are
unrecovable.
- [ ] Create systemd service so that
the program doesn’t need to be
run manually
- [ ] Add support for scheduling the
program in different ways

## Author

[Wasif Javed](https://github.com/wAABBsif)
