# benilla's `winit` fork — what differs from upstream, and how to check

Upstream: [`rust-windowing/winit`](https://github.com/rust-windowing/winit), MIT OR Apache-2.0,
version `0.30.13` — the version resolved by the workspace before this fork. The crate is copied
from the Cargo registry with its source, manifest, build script, documentation, examples, tests
and license files. It is wired through `[patch.crates-io]` in the workspace root.

## iOS redraw carry

`src/platform_impl/ios/app_state.rs` defers GPU redraws queued reentrantly during a
`RedrawRequested` callback until the callback finishes. UIKit can invalidate a layer while winit
is already processing redraws; upstream asserted that the queued redraw set must be empty, which
panicked inside an iOS CoreFoundation observer and aborted the app. The deferred redraws are
coalesced by the existing `HashSet` and delivered before winit returns to the run loop.

The seven iOS crash reports supplied with this change all show `SIGABRT` from
`winit::platform_impl::ios::event_loop::setup_control_flow_observers::control_flow_main_end_handler`
about one to two seconds after process launch. They corroborate the redraw-state failure; the
`.ips` reports do not contain the Rust panic payload itself.

Compare the patched iOS file against the registry copy of winit 0.30.13 to inspect the carry.
