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

By default, the files will be extracted to the same directory as each original archive file. For example, given the folder structure:

```bash
zip1.zip
subfolder
└── zip2.zip
```

Running `rzip --live .` from this folder will extract to:

```bash
zip1.zip
zip1
├── doc1.txt
└── doc2.txt
subfolder
├── zip2.zip
└── zip2
    ├── doc3.txt
    └── doc4.txt
```

You can also specify an output directory for the extracted files using the `--out-dir` option.

```bash
rzip --live --out-dir ./path/to/output/directory ./path/to/target/directory
```

© 2024 Joseph W. Micheli, see License.txt for more information.
