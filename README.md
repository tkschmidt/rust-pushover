rust-pushover
=============

CURRENTLY NON-FUNCTIONAL
I'm in the process of updating for Rust v1.0, and changing the http library that is used from rust-http to hyper. Once done, hopefully it will be back up and running.


[![Build Status](https://travis-ci.org/EdBrereton/rust-pushover.svg?branch=master)](https://travis-ci.org/EdBrereton/rust-pushover)

A rust crate for sending messages via the [Pushover.net](https://pushover.net/) service. It currently uses the excellent [rust-http](https://github.com/chris-morgan/rust-http) but will be converted to [teepee](https://github.com/teepee/teepee) when that reaches a usable state. Currently very basic, but functionality will be added. 

Rust Version
------------
rust-pushover should track with the Rust nightly versions, however at least one of it's dependancies tracks to the master branch (rust-http). This will occassionally result in cases where the nightly and the master are out of sync and the build failing. Whenever I notice this occurring, I will pin the version of the offending dependancy at the latest working version as a temporary measure until things are resolved. Hopefully once rust reaches V1.0 this will cease to be an issue.

Building
----------
Just use ```cargo build``` to build. There is also a test suit available with ```cargo test```

