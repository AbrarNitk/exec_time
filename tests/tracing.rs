#![cfg(feature = "tracing")]

use exec_time::exec_time;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn block_on<F: Future>(future: F) -> F::Output {
    fn raw_waker() -> RawWaker {
        fn clone(_: *const ()) -> RawWaker {
            raw_waker()
        }

        fn wake(_: *const ()) {}
        fn wake_by_ref(_: *const ()) {}
        fn drop(_: *const ()) {}

        RawWaker::new(
            std::ptr::null(),
            &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
        )
    }

    let waker = unsafe { Waker::from_raw(raw_waker()) };
    let mut future = Box::pin(future);
    let mut context = Context::from_waker(&waker);

    loop {
        match Pin::as_mut(&mut future).poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[exec_time(print = "never", backend = "tracing", level = "debug")]
fn sync_tracing_backend(value: i32) -> i32 {
    value + 1
}

#[exec_time(
    print = "never",
    backend = "tracing",
    name = "async.work",
    level = "trace"
)]
async fn async_tracing_backend(value: i32) -> i32 {
    value + 1
}

#[test]
fn sync_function_accepts_tracing_backend_arguments() {
    assert_eq!(sync_tracing_backend(41), 42);
}

#[test]
fn async_function_accepts_tracing_backend_arguments() {
    assert_eq!(block_on(async_tracing_backend(41)), 42);
}
