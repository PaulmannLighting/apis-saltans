# Dependency review, 2026-09-29

AI-assisted review of the dependency updates in `Cargo.lock`. Successful delta
audits are recorded in `audits.toml`; no exemptions were added. The following
findings prevent certification of the complete updated dependency graph.

## wasm-bindgen 0.2.126 → 0.2.129

In `src/rt/mod.rs`, the new safe, public functions
`__wbindgen_jspi_set_rejected` and `jspi_rejected` write and read the mutable
global `JSPI_REJECTED` without synchronization. Neither the global nor these
functions is restricted to single-threaded targets. The adjacent
`GLOBAL_EXNDATA` does have thread-local handling for atomics targets.

The comment explains why sequential fiber resumes are safe, but does not
justify concurrent calls from threads through the public `__rt` module. Such
calls can create a Rust data race. This is a static soundness finding, not a
demonstrated exploit in this workspace. The `js-sys` JSPI module's atomics
restriction does not restrict the runtime functions themselves.

No runtime audit is recorded. The `js-sys` 0.3.106 and
`wasm-bindgen-macro-support` 0.2.129 reviews also remain uncertified; these are
coupled to the new runtime. The small macro facade and shared schema changes
have separate delta audits, which do not certify their dependencies.

## libc 0.2.186 → 0.2.189

The new `src/new/glibc/sysdeps/x86/nptl/bits/struct_mutex.rs` selects the
mutex-kind offset by pointer width: 16 bytes for 64-bit pointers, 12 otherwise.
On `x86_64-unknown-linux-gnux32`, pointers are 32 bits but the x86_64 mutex
layout still places `__kind` at byte 16. The old x32 initializers correctly
placed their nonzero kind byte there.

This follows from glibc's [x86 mutex definition](https://github.com/bminor/glibc/blob/master/sysdeps/x86/nptl/bits/struct_mutex.h),
which includes the preceding `__nusers` field whenever `__x86_64__` is defined,
independently of pointer width. The new initializer instead writes the kind
into `__nusers`, leaving `__kind` zero. Recursive and error-checking mutexes
therefore receive the wrong kind on x32. Ordinary x86_64 with 64-bit pointers
does not take the incorrect branch. This finding was established by source
and ABI comparison; no x32 execution was performed.

No libc delta audit is recorded.

## Versioning and validation

The original user changes update only lockfile resolutions, with no workspace
API or manifest dependency-requirement changes. No workspace crate version
bump is required for those changes under SemVer 2.0.

`cargo +nightly fmt --check`, `cargo clippy --all-features`, and
`cargo test --workspace --all-features` passed on the host. These checks do
not resolve the platform-specific findings above.

`cargo vet regenerate unpublished` and `cargo vet prune` completed. The final
`cargo vet --locked` run reports four remaining gaps: `libc` 0.2.189,
`wasm-bindgen` 0.2.129, `js-sys` 0.3.106, and
`wasm-bindgen-macro-support` 0.2.129. The user's dependency updates are retained
pending a decision about restoring previously audited versions.
