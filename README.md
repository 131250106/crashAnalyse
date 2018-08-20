# crashAnalyse
de-duplicate and triage the crashes

## Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/lldb).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
lldb = "0.0.7"
```

On macOS, the `LLDB.framework` requires that an `@rpath`
be configured for your application so that the `LLDB.framework`
can be found. This isn't directly supported by Cargo today, but
for local work and development, you can do this:

```shell
export DYLD_FRAMEWORK_PATH=/Applications/Xcode.app/Contents/SharedFrameworks
```

