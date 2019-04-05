# hubauth

[![Build Status](https://travis-ci.com/liamdawson/hubauth.svg?branch=master)](https://travis-ci.com/liamdawson/hubauth)
[![Percentage of issues still open](http://isitmaintained.com/badge/open/liamdawson/hubauth.svg)](http://isitmaintained.com/project/liamdawson/hubauth "Percentage of issues still open")
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/liamdawson/hubauth.svg)](http://isitmaintained.com/project/liamdawson/hubauth "Average time to resolve an issue")
![Maintenance status: experimental](https://img.shields.io/badge/status-experimental-red.svg)

## Installation

**Not Recommended**: I suggest you wait until version 1.0

## Development

Prerequisites:

* Rust stable and cargo (rustup preferred)
  * Goal: use 1.24.1 for stretch `deb-src` compatibility
  * Might not be feasible: pre-2018 edition
* `clippy` and `rustfmt` support via cargo (e.g. `rustup component add clippy rustfmt`)
  * Don't need to run at an older version
* `pandoc` (= [1.17.2](https://github.com/jgm/pandoc/releases/tag/1.17.2))

(Package versions are generally chosen for Debian `stretch` compatibility)

## Roadmap

* [ ] More unit testing
* [ ] Integration testing
* [ ] `hubauth init` to make the `sshd_config` changes automatically
* [ ] Debug logging
* [ ] Cache file locking (reduce race conditions)
* [ ] Cache clear/clean
* [ ] Debian packaging (in separate repo)
