# hubauth

[![Build Status](https://travis-ci.com/liamdawson/hubauth.svg?branch=master)](https://travis-ci.com/liamdawson/hubauth)
[![Percentage of issues still open](http://isitmaintained.com/badge/open/liamdawson/hubauth.svg)](http://isitmaintained.com/project/liamdawson/hubauth "Percentage of issues still open")
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/liamdawson/hubauth.svg)](http://isitmaintained.com/project/liamdawson/hubauth "Average time to resolve an issue")
![Maintenance status: experimental](https://img.shields.io/badge/status-experimental-red.svg)

## Installation

**Not Recommended**: I suggest you wait until version 1.0

On the releases tab, you'll find `tar.gz` and `deb` releases. The `deb` will configure your system to have man pages, private folders/config, and install the utility. The `tar.gz` contains some documentation, and the utility itself--configuring your system securely will be up to you.

## Development

Prerequisites:

* Rust stable and cargo (rustup preferred)
* `clippy` and `rustfmt` support via cargo (e.g. `rustup component add clippy rustfmt`)
* [changelog-cli](https://pypi.org/project/changelog-cli/) (Optional: can manually work on the changelog by hand)
* [ronn-ng](https://rubygems.org/gems/ronn-ng) (Optional: to generate man pages)
* make and [fpm](https://rubygems.org/gems/ronn-ng) (Optional: to generate packages)
* dpkg tools (Optional: to generate `deb` packages)
* rpm tools (Optional: to generate untested `rpm` packages)

## Roadmap

* [ ] More unit testing
* [ ] Integration testing
* [ ] `hubauth init` to make the `sshd_config` changes automatically
* [ ] Debug logging
