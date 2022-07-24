# Cargo-ws-manage

Cargo-ws-manage is a command line tool written in rust and is made for an easier and faster way of handling cargo workspaces. Becaues of the 'cargo-' prefix, it can be used as a sub-command of cargo with `cargo ws-manage`.



## Planned features

There is not much to see currently, but these are some of the features that are to be added in the future:

- create workspaces:
  
  - with and without root crates

- add crates

- remove crates

- pass cargo commands to all crates (like cargo fmt, cargo add, cargo publish, ...)



## Getting started

For installation, run:

```bash
cargo install cargo-ws-manage
```

It can then be used with:

```bash
cargo ws-manage # -h for help
```

To remove it, run:

```bash
cargo uninstall cargo-ws-manage
```
