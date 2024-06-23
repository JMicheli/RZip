# R(ecursive) Zip

A recursive unzipping tool.

## Usage

By default, the tool will perform a dry run. You can use this mode to ensure that the correct directory is targeted before beginning the full unzip operation.

```bash
rzip ./path/to/target/directory
```

To actually begin an unzip operation, use the `--live` flag.

```bash
rzip --live ./path/to/target/directory
```

© 2024 Joseph W. Micheli
