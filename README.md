# hemoglobin

[![Join the chat at https://gitter.im/hemoglobin-rs/Lobby](https://badges.gitter.im/hemoglobin-rs/Lobby.svg)](https://gitter.im/hemoglobin-rs/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Build Status](https://travis-ci.org/bemeurer/hemoglobin.svg?branch=master)](https://travis-ci.org/bemeurer/hemoglobin)

A Cellular Automaton written in Rust

To run, [install Rust](https://www.rust-lang.org/en-US/install.html) and then in the project's root directory do this

```
cargo run --release xxxxxxx
```

where `xxxxxxx` is a number that encodes the rule for the automaton. This number can be anything from 0 to 2^512 -1. Once the program starts, hit `a` to start the automaton, `g` to regenerate a new starting state, and `q` to quit.
