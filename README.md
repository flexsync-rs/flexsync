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

## "Pre-release" versions for libraries & library dev - FOR LATER

Some consumer _library_ crates may compile in a limited way without some heavier
proc macros used by `flexsync`, even though they won't be fully usable
(runnable) that way.

That is useful for fast development (of libraries; for example, for fast
responses from `rust-analyzer` or your IDE's built-in Rust support). Because of
that, each "logical" version of `flexsync` has three "physical" versions on
[crates.io/crates/flexsync](https://crates.io/crates/flexsync).

Those three "streams" of "physical" versions are in two groups:

1. Pre-releases: for
   - (static) libraries, and
   - development of (static) libraries
2. Releases: for binaries (and shared objects/dynamic libraries/proc macros).

Each `x.y.z` "logical" version has three "physical" versions:
- `x.0.0-lib.y.z+for-lib`
- `x.0.0-lite.y.z+for-lib-dev`
- `x.y.z+for-bin`

For example, "logical" versions 1.1.4, 1.1.5 and 1.1.6 will have "physical"
versions:
- `1.0.0-lib.1.4+for-lib`
- `1.0.0-lib.1.5+for-lib`
- `1.0.0-lib.1.6+for-lib`
- `1.0.0-lite.1.4+for-lib-dev`
- `1.0.0-lite.1.5+for-lib-dev`
- `1.0.0-lite.1.6+for-lib-dev`
- `1.1.4+for-bin`
- `1.1.5+for-bin`
- `1.1.6+for-bin`

`x.0.0-lib.y.z+for-lib` and `x.y.z+for-bin` versions are identical, except that 

- `x.0.0-lib.y.z+for-lib` _will_ match/update to `x.0.0-lite.y.z+for-lib-dev`,
  and will _also_ match/update to any `x.y.z+for-bin`,
- but `x.0.0-lite.y.z+for-lib-dev` will _not_ match/update to any
  `x.y.z+for-bin`. (More below.)

`x.0.0-lite.y.z+for-lib-dev` is a lightweight version. It minimizes use of heavy
proc macros. It is _not_ used by default, only if you request it as an exact
version (below). It's for (limited) development of libraries only.

`x.y.z+for-bin` is published _before_ `x.0.0-lite.y.z+for-lib-dev`. That way any
dependemts that specify/request `flexsync` with `version =
"x.0.0-lite.y.z+for-lib-dev"` will continue to build with _full_ functionality.

Only when you specify/request an exact version of `flexsync` with exact version
match, like `version = "=x.0.0-lite.y.z+for-lib-dev"`, you will not get the
heavy proc macros compiled at all.

### Rules of thumb

- in dependents that themselves are (static) library crates, specify `version =
  "1.0.0-lib.1.4+for-lib"`. Do _not_ specify it as an exact version (so do _not_
  use the `=` inside the version string: `version = "=1.0.0-lib.1.4+for-lib"`).
- in other types of crates (binary, shared object, proc macro...), specify
  `version = "1.1.4+for-bin"`
- use `version = "=1.0.0-lite.1.4+for-lib-dev"` only for local development of
  (static) libraries. Never publish it - change it to one of the above before
  publishing.

### No patch-only updates for libraries

Unlike common, consumers that themselves are (static) libraries can't easily
restrict/limit any future updates to patch-level. That would restrict the
updates to stay within the same _minor_ version (and its major version, of
course).

We trade that fine control for development speed. And really, dependent
libraries shouldn't need this fine control. There should not be any
incompatible, or problematic, updates within the same major version.

However, this is so only if the consumers also want to support the "lite"
mode/version.

If a dependent library needs a patch level-specific exact version of a library
dependency, try using
https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#tilde-requirements
on another crate. @TODO

## Preventing lite versions from matching the for-bin versions

`x.0.0-lite.y.z+for-lib-dev` will _not_ match/update to any `x.y.z+for-bin`,
because they both depend on different fixed minor versions (but both within the
same major version) of `flexsync-versions`.

@TODO 

0.0.0-lib-lite <- linked with exact ="= version" <--- *-lib-lite.*, so that
*-lib-lite.* _CAN'T_ auto-update/match to (the next, or any newer) *+for-bin.

@TODO

0.0.1-for-bin <- linked with exact ="= version" <--- 1.1.4+for-bin


## See also
- [Cargo Reference > Manifest > The version
Field](https://doc.rust-lang.org/nightly/cargo/reference/manifest.html#the-version-field)
- [Cargo Reference > Specifying Dependencies >
Pre-releases](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#pre-releases)
- [Cargo Reference > Specifying Dependencies >Version
  metadata](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#version-metadata)

## Proc macro functionality - forward compatible

For now, `flexsync` is a proc macro crate. Once there is any non-proc macro
functionality, proc macros will move to a separate crate and `flexsync` will
re-export them. Forward compatible.

## Disambiguation

_Not_ related to [github.com/flexsync](https://github.com/flexsync) or any other
project with a similar name.
