# Testing

Tests live in files ending in `.test.deor`, such as `temperature.test.deor`. A test is a function named `test_*` that takes no arguments and returns a string:

- `SUCCESS` (an empty string) means the test passed.
- Any other string means it failed, and the string is the message shown.
- `FAILURE` is a shortcut for a failure with no message. It prints as `(No Message)`.

```
fn string test_absolute_zero_is_zero_kelvin()
	int celsius = -273
	int actual = celsius_to_kelvin(celsius)

	if actual is not 0
		return "absolute zero should be 0 kelvin"

	return SUCCESS
```

A test file has no `main`. It can `import` any code in the project.

## Running

`deor test` (or `just test`) searches the current directory and everything below it for `*.test.deor` files, skipping gitignored paths. It must run inside a git repository. `deor test path` narrows the search to one file or folder.

```
tests/temperature.test.deor
  SUCCESS: test_freezing_point_in_kelvin
  FAIL: test_below_zero - Temperature cannot be below absolute zero
  FAIL: test_crash - crashed: index out of range
2 ran, 2 failed
```

Every test runs even if an earlier one fails or crashes. The exit code is non-zero if any test failed or a test file did not build.

A function named `test_*` that does not read `fn string name()` is a transpile error.

## Running inside a larger crate

`deor input.test.deor output.rs --cargo-test` writes the file's Rust with no `main`. Each `test_*` function gets a `#[cfg(test)] #[test]` wrapper named `test_name_as_cargo_test`, so the output can be part of a bigger Rust crate and run with `cargo test`. A test passes when it returns `SUCCESS`; any other string fails with that message.
