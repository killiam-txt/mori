# mori (森)

disk usage tree viewer for the terminal.

```
  73.3 MiB ████████████████████ 100.0%  .
  73.3 MiB ████████████████████  99.9%  ├── target
  73.3 MiB ████████████████████  99.9%  │   ├── debug
   4.0 KiB ░░░░░░░░░░░░░░░░░░░░   0.0%  │   └── CACHEDIR.TAG
  24.0 KiB ░░░░░░░░░░░░░░░░░░░░   0.0%  ├── src
   8.0 KiB ░░░░░░░░░░░░░░░░░░░░   0.0%  ├── Cargo.lock
   4.0 KiB ░░░░░░░░░░░░░░░░░░░░   0.0%  └── Cargo.toml
```

## install

```
git clone https://github.com/killiam-txt/mori
cd mori
cargo install --path .
```

> [!NOTE]
> make sure `~/.cargo/bin` is in your `PATH`.

## usage

```
mori [options] [path]
```

| option | description | default |
|---|---|---|
| `-d, --depth <n>` | levels of subdirectories to show | 3 |
| `-l, --limit <n>` | max entries per directory | 20 |
| `-H, --hidden` | include hidden files and directories | off |
| `-h, --help` | print help | |
| `-V, --version` | print version | |

```
mori                    # scan current directory
mori [path]             # scan a specific directory
mori -d 1               # show one level deep
mori -l 10              # max 10 entries per directory
mori -H                 # include hidden files
```

each line shows the size on disk, a bar and the percentage relative to the scanned directory, followed by the tree branch. entries are sorted from largest to smallest.

when a directory has more entries than the limit, the smallest ones are collapsed into a single line showing how many were hidden and their combined size.

sizes are measured the same way as `du`, the standard unix disk usage command: real space used on disk, hard links counted once, and only one filesystem scanned.

## development

```
cargo test
cargo run -- [args]
```

## structure

```
mori/
└── src/
    ├── main.rs      # entrypoint
    ├── cli.rs       # argument parsing
    ├── scan.rs      # directory walk
    ├── tree.rs      # node and sorting
    ├── render.rs    # ascii output
    └── size.rs      # human readable sizes
```

## license

MIT