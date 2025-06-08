<pre>
██╗      ██████╗ ███████╗██╗    ██╗     ██╗ ██████╗ ██╗  ██╗████████╗███╗   ██╗██╗███╗   ██╗ ██████╗ 
██║     ██╔═══██╗██╔════╝██║    ██║     ██║██╔════╝ ██║  ██║╚══██╔══╝████╗  ██║██║████╗  ██║██╔════╝ 
██║     ██║   ██║█████╗  ██║    ██║     ██║██║  ███╗███████║   ██║   ██╔██╗ ██║██║██╔██╗ ██║██║  ███╗
██║     ██║   ██║██╔══╝  ██║    ██║     ██║██║   ██║██╔══██║   ██║   ██║╚██╗██║██║██║╚██╗██║██║   ██║
███████╗╚██████╔╝██║     ██║    ███████╗██║╚██████╔╝██║  ██║   ██║   ██║ ╚████║██║██║ ╚████║╚██████╔╝
╚══════╝ ╚═════╝ ╚═╝     ╚═╝    ╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═══╝╚═╝╚═╝  ╚═══╝ ╚═════╝ 
</pre>

🎵 Beats by Btrust 🎵

## Overview

A hands-on workshop where you'll set up and run your own Lightning Network node using LDK Node.
This session offers a relaxed introduction to LDK Node, encourages creativity, and leaves a secret to be uncovered.
Will you find it first? We'll try to get you leaving the session with a working Lightning Network node.
Encore, do you want more? You are invited to spend more time in the room hacking on Bitcoin outside of the session
time while enjoying LoFi beats by Btrust.

This workshop has a start, a few steps, but the rest is up to you. Let's see what you create.

## Project setup

### Rustup

If you've never used Rust before, you'll need a Rust toolchain. The easiest way to manage Rust toolchains is
via https://rustup.rs. If (rightfully so) you don't feel comfortable directly executing the response of a curl,
install rustup through your package manager of choice. You'll need at least Rust version 1.75 to work with LDK Node.

### Clone this repository

```bash
git clone git@github.com:dunxen/lofi-lightning.git
```

### Crates used in this workshop

The two essential crates we'll be using for this workshop are:

1. [`ldk-node`](https://crates.io/crates/ldk-node) - (obviously) to create our own LN node
2. [`rustyline`](https://crates.io/crates/rustyline) - to make working with stdin easier in order to interact with our node

However, this workshop encourages creativity and exploration. Feel free to spice up your node. Some ideas include:

* [`ratatui`](https://crates.io/crates/ratatui) - A crate that makes it easier to create terminal user interfaces (TUIs)
* [`lnurl-rs`](https://crates.io/crates/lnurl-rs) - Make your node compatible with various LNURL features, such as paying to a Lightning address



