
# C Wrapper

This workspace is a C wrapper for the `surrealml-core` library. This enables us to no longer need `PyO3` and we can also use this library in other languages.

## Testing

To test this C wrapper we first need to build the C lib and position it in the correct location for the Python tests to load the library. We can perform this setup with the following command:

```bash
sh ./scripts/prep_tests.sh
```

This will build the C lib in debug mode and place it in the correct location for the Python tests to load the library. We can then run the tests with the following command:

```bash
sh ./scripts/run_tests.sh
```

If you setup pycharm to put your Python tests through a debugger, you need to open pycharm in the root of this workspace and set the `tests` directory as the sources root. This will allow you to point and click on specific tests and run them through a debugger.
