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

    // These tests only use immediately-ready futures, so a no-op waker is enough.
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

#[exec_time(print = "never")]
async fn adds_one_async(value: i32) -> i32 {
    value + 1
}

#[exec_time()]
async fn multiplies_async(left: i32, right: i32) -> i32 {
    left * right
}

#[exec_time(print = "never", prefix = "math", suffix = "sum")]
async fn sum_slice_async(values: &[i32]) -> i32 {
    values.iter().sum()
}

#[exec_time(print = "never", name = "math.sum", unit = "us")]
async fn sum_slice_with_name_async(values: &[i32]) -> i32 {
    values.iter().sum()
}

#[exec_time(print = "never", log_over = "5ms", warn_over = "10ms")]
async fn sum_slice_with_thresholds_async(values: &[i32]) -> i32 {
    values.iter().sum()
}

#[exec_time(print = "never")]
async fn first_owned_async<T>(mut values: Vec<T>) -> T
where
    T: Clone,
{
    values.remove(0)
}

#[exec_time(print = "never")]
async fn clone_from_ref_async<T>(value: &T) -> T
where
    T: Clone,
{
    value.clone()
}

struct AsyncAccumulator {
    value: i32,
}

impl AsyncAccumulator {
    #[exec_time(print = "never")]
    async fn add(&mut self, amount: i32) -> i32 {
        self.value += amount;
        self.value
    }
}

#[test]
fn async_function_returns_expected_value() {
    assert_eq!(block_on(adds_one_async(41)), 42);
}

#[test]
fn async_function_with_empty_attribute_arguments_still_runs() {
    assert_eq!(block_on(multiplies_async(6, 7)), 42);
}

#[test]
fn async_function_accepts_prefix_and_suffix_arguments() {
    assert_eq!(block_on(sum_slice_async(&[10, 20, 12])), 42);
}

#[test]
fn async_function_accepts_name_and_unit_arguments() {
    assert_eq!(block_on(sum_slice_with_name_async(&[10, 20, 12])), 42);
}

#[test]
fn async_function_accepts_threshold_arguments() {
    assert_eq!(block_on(sum_slice_with_thresholds_async(&[10, 20, 12])), 42);
}

#[test]
fn async_function_preserves_generics_and_where_clause() {
    assert_eq!(block_on(first_owned_async(vec![42, 7, 9])), 42);
}

#[test]
fn async_function_supports_generic_return_value_with_where_clause() {
    assert_eq!(
        block_on(clone_from_ref_async(&String::from("answer"))),
        "answer"
    );
}

#[test]
fn async_method_inside_impl_preserves_mutation_and_return_value() {
    let mut accumulator = AsyncAccumulator { value: 40 };

    assert_eq!(block_on(accumulator.add(2)), 42);
    assert_eq!(accumulator.value, 42);
}
