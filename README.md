# Flexsync

(placeholder - work in progress).

Help reuse same Rust code for both `async` and sync/blocking. When used as
`async`, it will help to re-use same code for various types of `async` runtimes

- ones requiring the `Future` to be both `Send` and `'static` - for `tokio` and
  other multithreaded runtimes
- ones requiring the `Future` to be just `static` (but not necessarily `Send`) -
  for `no_std`/embedded runtimes
- ones not requiring the `Future` to be `Send` or `static` (scoped `no_std`,
  combinators)
- possibly for other requirements on the `Future`.

## lite versioning

This crate uses `lite` versioning for fast navigation/development of
libraries. See [crates.io/crates/lite](https:://crates.io/crates/lite).

## Proc macro functionality - forward compatible

For now, `flexsync` is a proc macro crate. Once there is any non-proc macro
functionality, proc macros will move to a separate crate and `flexsync` will
re-export them. Forward compatible.

## Third party/related issues and Announcements

Please

- subscribe to [Announcements
  (flexsync-rs/flexsync#1)](https://github.com/flexsync-rs/flexsync/issues/1)
- give thumbs up, and subscribe to, [3rd party/related issues
  (flexsync-rs/flexsync#2)](https://github.com/flexsync-rs/flexsync/issues/2).

## Disambiguation

_Not_ related to [github.com/flexsync](https://github.com/flexsync), [`flex`
crate](https://crates.io/crates/flex) or any other project with a similar name.
