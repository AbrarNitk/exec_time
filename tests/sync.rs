use exec_time::exec_time;

#[exec_time(print = "never")]
fn adds_one(value: i32) -> i32 {
    value + 1
}

#[exec_time()]
fn multiplies(left: i32, right: i32) -> i32 {
    left * right
}

#[exec_time(print = "never", prefix = "math", suffix = "sum")]
fn sum_slice(values: &[i32]) -> i32 {
    values.iter().sum()
}

#[exec_time(print = "never")]
fn first_owned<T>(mut values: Vec<T>) -> T
where
    T: Clone,
{
    values.remove(0)
}

struct Accumulator {
    value: i32,
}

impl Accumulator {
    #[exec_time(print = "never")]
    fn add(&mut self, amount: i32) -> i32 {
        self.value += amount;
        self.value
    }
}

#[test]
fn sync_function_returns_expected_value() {
    assert_eq!(adds_one(41), 42);
}

#[test]
fn sync_function_with_empty_attribute_arguments_still_runs() {
    assert_eq!(multiplies(6, 7), 42);
}

#[test]
fn sync_function_accepts_prefix_and_suffix_arguments() {
    assert_eq!(sum_slice(&[10, 20, 12]), 42);
}

#[test]
fn sync_function_preserves_generics_and_where_clause() {
    assert_eq!(first_owned(vec![42, 7, 9]), 42);
}

#[test]
fn sync_method_inside_impl_preserves_mutation_and_return_value() {
    let mut accumulator = Accumulator { value: 40 };

    assert_eq!(accumulator.add(2), 42);
    assert_eq!(accumulator.value, 42);
}
