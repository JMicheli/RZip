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

You can also specify an output directory for the extracted files using the `--out-dir` option. If not specified, the files will be extracted to the current working directory.

```bash
rzip --live --out-dir=./path/to/output/directory ./path/to/target/directory
```

© 2024 Joseph W. Micheli
