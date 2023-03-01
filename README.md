# Rx

Small CLI utility wrapper around Nx tooling to make easy selection of project and task.

⚠️ I build this as a way to learn Rust, so it is definitely not robust and production ready. Use on your own danger.

## Motivation

I use primarely Nx from the terminal and I remember only the main commands. But as soon as I need
some custom command I usualy have to go to the `project.json` file and look it up. This program
makes selection of project and task much easier.

Also one of the main motivation was to try a Rust language and this use-case was a perfect fit and
it deals with terminals, parsing json files etc.

## How to use

If you really want to try this out, clone this repo and run `cargo build`. Then in your monorepo run
the binary `<path_to_rx_repo>/target/debug/rx` and follow the instructions.

Or if you want to add it as a binary to your path, run these commands:

```shell
cargo build --release
cargo install --path .
```

Then you can simply type `rx`.


Expect a lot of edgy behaviour and unpolished experience.
