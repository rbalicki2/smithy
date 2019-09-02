# Version 0.0.7 (2019-09-02)

* Update the README to indicate with which version of the nightly compiler Smithy is compatible.
* Make Smithy more efficient: remove unnecessary code in the expansion of the `smd!` and `smd_borrowed!` macros, and move some path manipulation to compile time.

# Version 0.0.6 (2019-08-08)

* Update the README to point users to the `create-smithy-app`[https://github.com/rbalicki2/create-smithy-app/]
  repository.
* Got rid of some compiler warnings.
* Do not call `console_error_panic_hook` in Smithy.

# Version 0.0.5 (2019-07-08)

# Features

* Now works with rustc 1.37.0-nightly (8ebd67e4e 2019-06-27) due to
  the smd! macro using a file cache instead of an in-memory cache

# Version 0.0.4 (2019-06-26)

## Features and breaking changes

* Renamed `smd_no_move!` to `smd_borrowed!`
* Cache calls to `smd!` and `smd_borrowed!` to improve compile times
* Rename the "debug-logs" feature to "browser-logs"
* Additional events documentation

## Bugs

* Fix a bug that caused smithy to panic when updating some interpolated
  variables (e.g. if `count` was updated in `<div>{ count }</div>`
* Fix a bug allowing certain event related features to be enabled
  (including all-features)

# Version 0.0.3 (2019-04-29)

* Add the `smd_no_move!` macro
* Add documentation
* fix smd!() not compiling
* fix unused import compiler warnings
* add features for global events: `before-unload-events`, `hash-change-events`, `pop-state-events`, and `promise-rejection-events`
* add post-rendering tests
