# benilla's `winit` fork — what differs from upstream, and how to check

Upstream: [`rust-windowing/winit`](https://github.com/rust-windowing/winit), MIT OR Apache-2.0,
version `0.30.13` — the version resolved by the workspace before this fork. The crate is copied
from the Cargo registry with its source, manifest, build script, documentation, examples, tests
and license files. It is wired through `[patch.crates-io]` in the workspace root.

## iOS redraw carry

`src/platform_impl/ios/app_state.rs` coalesces GPU redraw invalidations raised reentrantly during
the `RedrawRequested` phase. UIKit can invalidate a layer while winit is already processing its
current draw; upstream asserted that the queued redraw set must be empty, which panicked inside an
iOS CoreFoundation observer and aborted the app. Replaying those invalidations inside the same
phase can re-enter the renderer, so the current draw satisfies them and the duplicate requests are
dropped. The main-end observer also ignores nested invocations while the event loop is already
inside a callback or redraw phase. UIKit can enter a nested run loop during rendering, and that
observer must leave the outer event-loop state intact until the current callback returns.

The supplied iOS crash reports show `SIGABRT` from
`winit::platform_impl::ios::event_loop::setup_control_flow_observers::control_flow_main_end_handler`
about one to two seconds after process launch. They corroborate the redraw-state failure; the
`.ips` reports do not contain the Rust panic payload itself.

Compare the patched iOS file against the registry copy of winit 0.30.13 to inspect the carry.
