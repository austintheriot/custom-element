# Contributing

### Set up

For getting your dev environment set up for compiling this repo locally, see [./SETUP.md](./SETUP.md)

## Scripts

See all available scripts at [./Makefile.toml](./Makefile.toml). These scripts are all run using the `cargo make` tool. To run the `test` task, for example, you would run the following from the root directory of this project in your shell

```sh
cargo make test
```

## Browser tests

The file [./scripts/test-browser.sh](./scripts/test-browser.sh) holds the browser test runner. The `cargo make` test tasks call that script. Run the script directly to test one browser and profile. See [./SETUP.md](./SETUP.md) for the options.
