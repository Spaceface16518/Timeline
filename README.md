# Timeline

[![Build Status](https://travis-ci.com/Spaceface16518/Timeline.svg?branch=master)](https://travis-ci.com/Spaceface16518/Timeline)

Parsing and processing of a simple timeline format

## Summary

Timeline is a command line tool that helps manage a growing timeline over time. It can parse a file for dates and convert them between serial types. Timeline accomplishes this using [the Rust programming language](https://www.rust-lang.org/) and [Serde](https://serde.rs/), and efficent (de)serialization library.

## Backstory
I was inspired to make this while taking [AP World History](https://apstudent.collegeboard.org/apcourse/ap-world-history). As I went through my textbook, there were so many dates that I needed to rememeber, sometimes not even presented in chronological order. For this reason, I needed a way to store dates on a timeline in non-chronological order in a way that would allow them to be constructed in chronological order. This was my answer.

## Features

Timeline may not be feature complete. These are all the planned (and implemented) features

- [ ] Parse input
  - [x] into JSON
  - [x] into BSON
  - [ ] into other formats (open ended)
- [ ] Store dates
  - [ ] in internal database
  - [ ] in user provided database
- [ ] Display dates
  - [ ] as storage format
  - [ ] as text
  - [ ] as graphic
  - [ ] from database
