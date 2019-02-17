# Timeline

[![Build Status](https://travis-ci.com/Spaceface16518/Timeline.svg?branch=master)](https://travis-ci.com/Spaceface16518/Timeline)

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

_Coming soon, sorry_
