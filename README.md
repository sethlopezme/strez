# strez

Take string resources in CSV format and convert them to the Android and iOS
formats. A good way to keep strings in sync between platforms and hand off
translations to developers.

## Install

Installation instructions here.

## Usage

```
$ strez --help
strez 0.1.0
Take string resources in CSV format and convert them to the Android and iOS formats.

USAGE:
    strez [OPTIONS] <input>

OPTIONS:
    -l, --default-language <default-language>    Set the default language.
    -d, --directory <directory>                  Set the output directory. [default: .] 
    -f, --force                                  Overwrite existing files with the same output filename(s).
    -h, --help                                   Prints help information
    -n, --name <name>                            Set the name of the output file(s). Defaults to platform conventions.
    -s, --source-format <source-format>          Set the input format. [default: csv]  [values: csv]
    -t, --target-format <target-format>          Set the output format. Accepts comma-delimited values. [default: android,ios]  [values: android, ios]
    -V, --version                                Prints version information
    -v, --verbose                                Display verbose output.

ARGS:
    <input>    Set the input file.
```

### Options

#### `-l`, `--default-language`

Default: _none_

Set the default language. The default language is output to the default strings
files for each platform:

- Android: `values/strings.xml`
- iOS:
    - `Base.lproj/Localizable.strings`
    - `Base.lproj/Localizable.stringsdict`

#### `-d`, `--directory`

Default: `.`

Set the output directory. Localization directories will be created within the
output directory if they do not exist.

#### `-f`, `--force`

Overwrite existing files with the same output filename(s).

#### `-n`, `--name`

Defaults:

- Android: `strings`
- iOS: `Localizable`

Set the name of the output file(s).

#### `-s`, `--source-format`

Default: `csv`

Set the input format. Valid formats are:

- `csv`

#### `-t`, `--target-format`

Default: `android,ios`

Set the output format. Accepts comma-delimited values. Valid formats are:

- `android`
- `ios`

#### `-v`, `--verbose`

Display verbose output.

### Examples

Read string resources from a CSV file and output them to the current directory
in the iOS format:

```sh
$ strez -t ios ~/strings.csv

# Output:
# ./Base.lproj/Localizable.strings
# ./Base.lproj/Localizable.stringsdict
```

Same as above, but output both iOS and Android formats:

```sh
$ strez -t android,ios ~/strings.csv

# Output:
# ./values/strings.xml
# ./Base.lproj/Localizable.strings
# ./Base.lproj/Localizable.stringsdict
```

Read string resources from a CSV file and output them as `my_strings.xml` to a
another directory in the Android format:

```sh
$ strez -t android -d MyProject/app/src/main/res -n my_strings ~/strings.csv

# Output:
# ./MyProject/app/src/main/res/values/my_strings.xml
```

## CSV

### Columns

Your CSV must have column headers. Unrecognized columns are not allowed. When
duplicate columns are found, the last value for a duplicate column is used.

Column headers are described below. Columns marked as required _must_ be present
and have a value. Columns marked as optional may be entirely omitted, or may be
present without requiring a value in any row.

#### Name (Required)

The name of the string resource. This name will be transformed into snake_case
for output.

#### Description (Optional)

The description of the string resource. This description will appear as a
comment above the resource in the output.

#### Quantity (Optional)

The quantity for the string resource, if applicable. When defining quantities
for a string resource, be sure to use the same resource name in each row. Valid
quantities are:

- zero
- one
- two
- few
- many
- other

#### ISO 639-1 Language Headers (Required)

_NOTE: Language columns should be placed after all other columns._

When defining values, use headers that correspond to the language codes in
[ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes). Multiple
translations can be defined by using a different language code header for each
value column.

Values are output to their respective language directories, i.e. `fr` values are
output to the `values-fr` (Android) and `fr.lproj` (iOS) directories.

### Examples

Examples of valid strez CSV files can be found in the
[examples/csv](examples/csv) directory.

## License

MIT &copy; Seth Lopez