The `strem` tool by default requires users to opt-in to additional features of the tool that are not core to its functionality.

To install the tool with desired features, the `--features` flag should be used followed by a list of comma-separated features. For more information on `cargo` features, see [here](https://doc.rust-lang.org/cargo/commands/cargo-install.html#feature-selection). For example, the `export` feature may be enabled by running the following command when installing the tool:

```console
$ cargo install --features="export" strem
```

## Export

!!! info "Feature Identifier"

    `export`

Support exporting images of matched frames to desired directory.
