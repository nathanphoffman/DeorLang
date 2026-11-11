# Deor test framework plan

## Decisions so far

- Test files are named `name.test.deor` (example: `temperature.test.deor`).
- A test is a function named `test_*` that returns a string and takes no arguments.
  - Returns `SUCCESS` (empty string) on pass.
  - Returns any other string on failure; the string is the message shown.
  - `FAILURE` is a shortcut for "failed, no message". Its value is `"(No Message)"`, so it prints as `FAIL: name - (No Message)`.
- `SUCCESS` and `FAILURE` are turned into the string literals `""` and `"(No Message)"` by the lexer. (A Rust global would not work: Deor strings are `String`, which a `const` cannot hold, and Deor consts cannot be exported.)
- Tests are plain `if` + `return "message"`. No `check_equal` and no assertion library. Deor's rule that calls with 2 or 3 arguments need variables stays unchanged.
- All tests run in one binary. The generated runner catches panics so a crashing test reports `FAIL` and the rest keep running.
- The runner always runs every test, prints a summary, and exits non-zero if any failed.
- Test files may `import` any code in the project. Test files have no `main`; the runner generates it.
- Entry points: `just test` and `deor test`. Both scan the current directory and its children for `*.test.deor`, skipping gitignored paths.

Output:

```
temperature.test.deor
  SUCCESS: check_freezing_point
  FAIL: check_absolute_zero - Temperature cannot be below absolute zero
2 ran, 1 failed
```

## Phase 1: user test framework (DONE)

1. Check how `crash()` is implemented. Confirm it is a real panic that `catch_unwind` can catch (not an immediate exit).
2. Decide how `SUCCESS` / `FAILURE` are injected as Rust globals, and what the transpiler needs for it.
3. Generate a runner from a `.test.deor` file: find `test_*` functions, call each inside `catch_unwind`, collect results.
4. Validate: a `test_*` function must return a string and take no arguments. Otherwise a clear transpiler error.
5. Print results in the format above, print the summary, set the exit code.
6. Discovery: find `*.test.deor` files from the current directory down, skipping gitignored paths.
7. Add `just test` and `deor test`.
8. Write the first real tests with it, including a deliberately failing one and a deliberately crashing one to prove the runner survives both.
9. Document it in the spec.

## Phase 2: compiler-specific tests (DONE for validation errors)

The compiler's tests are not function tests: they compile a file and check the result.

1. Stable error codes: every validation message ends with its code in brackets, such as `[max_params]`. The code is the rule's name in `define_errors.deor`, so there is no number list to keep in sync. (Chosen over `E012` because the name says what it is.)
2. No separate JSON mode. The bracketed code at the end of the normal message is what tests read.
3. Inline markers: `print(message)  # error: use_after_move` (several codes on one line: `# error: a, b`).
   - Passes if the set of (line, code) pairs matches the set the transpiler reports.
   - An unmarked error, a marker with no error, or an error with no code all fail the test, with both lists printed.
   - Inserting lines above never breaks a test, because the marker moves with its line.
4. `tests/unit_tester.deor` gets a `markers` mode (a file with no `# expect` block but with markers); helpers are in `tests/error_markers.deor`.
5. Output tests (`# expect:`), stdin, args and panic tests stay as comment blocks.

Known gap: errors from the importer (`[error] ...`, such as duplicate declarations and missing imports) have no code yet, so tests for those stay as `# expect-error:` text.

## Phase 3: convert the existing tests (DONE)

1. All 69 `expect-error` tests now use inline markers (`# error: code` and `# error-elsewhere: code`). To make that possible, the importer and macro errors also got bracketed codes.
2. The 4 `expect-panic` tests stay as they are.
3. The 79 `expect:` output tests stay as they are.
4. No renaming to `.test.deor`: that suffix is for files `deor test` runs, and these files contain Deor that is meant not to compile, so `deor test` would report them as broken.
5. `tests/unit_tester.deor` stays, with the unused `# expect-error:` and `# expect-rustc-error:` modes removed. It now handles `# expect:`, `# expect-panic:` and the error markers.
