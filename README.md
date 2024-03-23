[![build & test](https://github.com/thibault-cne/shiprs/actions/workflows/rust.yml/badge.svg)](https://github.com/thibault-cne/shiprs/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/shiprs.svg)](https://crates.io/crates/shiprs)
[![license](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![circleci](https://circleci.com/gh/thibault-cne/shiprs.svg?style=shield)](https://circleci.com/gh/thibault-cne/shiprs)
[![appveyor](https://ci.appveyor.com/api/projects/status/82p07asa7l0gv3t0?svg=true)](https://ci.appveyor.com/project/thibault-cne/shiprs)
[![docs](https://docs.rs/shiprs/badge.svg)](https://docs.rs/shiprs/)
[![rust report card](https://rust-reportcard.xuri.me/badge/github.com/thibault-cne/shiprs)](https://rust-reportcard.xuri.me/report/github.com/thibault-cne/shiprs)

# shiprs

A rust wrapper for the Docker Engine API. This library is intended to use the minimal amount of dependencies and to be as close to the Docker Engine API as possible.

## Sources

This library is strongly inspired by the [shiplift](https://github.com/softprops/shiplift) and [bollard](https://github.com/fussybeaver/bollard/tree/master) libraries.

## Warning

This library is still in development and is not ready for production use.

## Roadmap

- [ ] Implement the Docker Engine API with non-asynchronous calls
- [ ] Make the http module asynchronous
- [ ] Implement the Docker Engine API with asynchronous calls using tokio
