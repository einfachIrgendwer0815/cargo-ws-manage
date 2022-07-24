# Cargo-ws-manage

[![Crates.io][crates_img]][crates_lnk]
[![Docs.rs][docs_img]][docs_lnk]
[![Crates.io][dwn_img]][crates_lnk]
[![Crates.io][dwn_recent_img]][crates_lnk]
[![Github.com][issues_img]][issues_lnk]
[![Github.com][license_img]][license_lnk]

[crates_img]:https://img.shields.io/crates/v/cargo-ws-manage
[crates_lnk]:https://crates.io/crates/cargo-ws-manage

[docs_img]:https://img.shields.io/docsrs/cargo-ws-manage/latest
[docs_lnk]:https://docs.rs/cargo-ws-manage

[dwn_img]:https://img.shields.io/crates/d/cargo-ws-manage

[dwn_recent_img]:https://img.shields.io/crates/dr/cargo-ws-manage

[license_img]:https://img.shields.io/crates/l/cargo-ws-manage
[license_lnk]:https://github.com/einfachIrgendwer0815/cargo-ws-manage/blob/main/LICENSE

[issues_img]:https://img.shields.io/github/issues/einfachIrgendwer0815/cargo-ws-manage
[issues_lnk]:https://github.com/einfachIrgendwer0815/cargo-ws-manage/issues

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
