# bbdo-tool

## Introduction

If you use [centreon-broker](https://github.com/centreon/centreon-collect/),
maybe you saw files such as `central-broker-master.queue.sql` or
`central-broker-master.memory.sql` or `central-broker-master.unprocessed`.

And if you saw them, maybe you wanted to know their content or even more to
change their content.

So here is a tool for that purpose. It is not finished yet but begins to give
results.

## Installation

This software is written in [Rust](https://www.rust-lang.org/).
So you need this language to compile it. The build/installation are then very
simple (to install it into the /usr/local/bin directory), execute the following
commands:

```
cd bbdo-tool
cargo build --release
cargo install --path . --root /usr/local
```

## Usage

If you have a `/var/lib/centreon-broker/central-broker-master.unprocessed`
file, you can see its content using the command:

```
bbdo-tool /var/lib/centreon-broker/central-broker-master.unprocessed
```

*Warnings*.
* All the BBDO are not yet implemented in bbdo-tool, so you can have
  incomplete results.
* This software works with all bbdo files, compressed and not compressed.
