# Timeline

[![Build Status](https://travis-ci.com/Spaceface16518/Timeline.svg?branch=master)](https://travis-ci.com/Spaceface16518/Timeline)
![License](https://img.shields.io/github/license/Spaceface16518/Timeline.svg)
![GitHub repo size in bytes](https://img.shields.io/github/repo-size/Spaceface16518/Timeline.svg)

Parsing and processing of a simple timeline format

## Summary

Timeline is a command line tool that helps manage a growing timeline over time. It can parse a file for dates and convert them between serial types. Timeline accomplishes this using [the Rust programming language](https://www.rust-lang.org/) and [Serde](https://serde.rs/), and efficent (de)serialization library.

## Backstory

I was inspired to make this while taking [AP World History](https://apstudent.collegeboard.org/apcourse/ap-world-history). As I went through my textbook, there were so many dates that I needed to rememeber, sometimes not even presented in chronological order. For this reason, I needed a way to store dates on a timeline in non-chronological order in a way that would allow them to be constructed in chronological order. This was my answer.

## Features

Timeline may not be feature complete. These are all the planned (and implemented) features

- [x] Parse input
  - [x] from command line arguments
  - [x] from YAML
  - [x] into YAML
  - [x] into JSON
  - [ ] into other formats (open ended)
- [ ] Display dates
  - [x] as text
  - [ ] as graphic

## Schema

Timeline files currently have the following scheme

```yaml
- label: a label for this entry
  tag: an (optional) tag
  date:
    start: 1400
    end: 1750
```

or

```yaml
- label: this entry is a singular point instead of a range
  tag: an (optional) tag
  date: 600
```

or

```yaml
- label: this entry does not have a tag
  date: 1500
```

Simply list your entries in standard YAML format.

## Usage

Timeline is a command line application. At the moment, you must either obtain a precompiled binary or build it yourself from source code.

### Timeline format

Timeline uses YAML format for storing timelines. Check out the [schema section](#Schema) for how to format individual entries. Here's an example that I used often to test early versions of Timeline

```yaml
---
- label: A range with a tag
  tag: test
  date:
    start: 0
    end: 1
- label: A point with a tag
  tag: test
  date: 1
- label: A range without a tag
  date:
    start: 3
    end: 5
```

the output of which is

```plain
(test) 0 CE - 1 CE: A range with a tag
(test) 1 CE: A point with a tag
3 CE - 5 CE: A range without a tag
```

You are not required to list your entries in order; Timeline will do this for you upon rendering them.

### Building from source

You will need [Rust](https://www.rust-lang.org/) to build from source. Install Rust [here](https://www.rust-lang.org/tools/install).

Once you have Rust (which includes it's package manager, Cargo), download the source using the download option in GitHub or by cloning the repository.

Once you have the source code and Rust, run this command at the root directory of the project:

```shell
cargo build --release
```

and then

```shell
cargo run --release
```

to get the help message. The `render` command is the most commonly used command. Print it's help information like this:

```shell
cargo run --release -- help render
```

In all of these commands except the `build` command, the `cargo run --release --` can be replaced with the path to the binary file, usually `target/release/timeline` after running `cargo build --release` or `target/debug/timeline` after running just `cargo build`.

### Using a pre-compiled binary

There is not currently pre-compiled binaries available for distribution, so unless you get one from somebody who has built it from source, there is no way to obtain one.

That said, if you do have a precompiled binary, location your YAML file (`/path/to/file.yml` in this example) and run the binary with these arguments

```shell
timeline render --path /path/to/file.yml --text
```

You can also use the short versions of flags, ie `-p` instead of `--path` and `-t` instead of `--text`.

A couple notes about this example:

- Unless you `chmod` the binary or add it to a `$PATH` included directory, unix-like OS users must run it like `./timeline` instead of `timeline`
- I'm not familiar with how you run executables on windows 😬 so you're on your own for that one. I will update this as soon as I find out.

## Developer Section

Feels free to clone this repository and mess with it. If you are familiar with Rust, I am using `serde` (as well as `serde_json` and `serde_yaml`) for (de)serialization and `structopt` for command line argument parsing. Graphics are not implemented as of yet, but I am planning to use `handlebars-rs` for html templating and the Google Charts API for timeline rendering. Using SVG for rendering is also an option but there is no plan to implement this as of yet.

### Project structure

This project is split into a `bin`ary application and its `lib`rary backend.

Comments are (hopefully) provided for detailed documentation, but here are some summaries.

Run `cargo doc` to build documentation. It will be available at `target/doc/`

#### Binary

The binary portion is contained within one file, `src/main.rs`.

It can be seen as split into two sections. One section handles command line argument processing and the other deals with the actions chosen by the user through the command line arguments. There are some basic structs to collect arguments and glue options together, but the actual subcommands and their associated actions are abstracted into a separate `structopt` struct and a function with the same name (except lower-cased). The `main` method simply collects the arguments from the command line and routes the subcommands to the correct function. Said function deals with all the actions.

#### Library

The library section is split into two modules, the main module (`src/lib/mod.rs`) and a supplementary module for the actual `Date` type. The library is a lot more complex, so it won't be summarized here. There should be documentation inside the file (or you can build documentation using `cargo doc`). If I have free time, I might add a summary, but for now there is none.
