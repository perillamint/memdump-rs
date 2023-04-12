<!--
SPDX-FileCopyrightText: 2023 perillamint

SPDX-License-Identifier: CC0-1.0
-->

# memdump-rs
Handy unsafe memory dumper utility library written in Rust.

## Usage example
```rust
const FOO: &str = "Hello, world!\n What is your name?";

fn example_func() {
    unsafe {
        memdump(FOO.as_ptr(), FOO.len(), |s| println!("{}", s));
    }
}
```
