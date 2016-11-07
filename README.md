# strez

`strez` makes it easier to hand strings and translations off to developers and
keep those strings in sync. One true source of strings can be defined in one
format (CSV-only, for now) and be converted to formats readable by multiple
platforms (currently Android and iOS).

## Install

Installation instructions here.

## Usage

```
$ strez --help
strez 0.1.0
Convert strings to formats readable by the Android and iOS platforms.

USAGE:
    strez [FLAGS] [OPTIONS] <infile> <outdir>

FLAGS:
    -f, --force      Overwrites existing files
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --default-language <language>    Sets the default language
        --format <format>                Sets the input data format [default: csv]  [values: csv]
        --platform <platform>            Sets the platform(s) (comma-delimited) [default: android,ios]  [values: android, ios]

ARGS:
    <infile>    Input file path
    <outdir>    Output directory path
```

### Options

#### `--default-language`

Default: _none_

Sets the default language. Strings defined in the default language will be
output to the default strings file for each platform:

- Android: `values/strings.xml`
- iOS:
    - `Base.lproj/Localizable.strings` (singular strings)
    - `Base.lproj/Localizable.stringsdict` (quantity strings)

#### `--format`

Default: `csv`

Sets the input data format. For now, the only supported format is `csv`, but
more will be added in the future.

#### `--platform`

Default: `android,ios`

Sets the platform(s) for which `strez` will output files. Values must be
comma-delimited.

Valid values are:

- `android`
- `ios`

### Examples

Assuming only English translations, read strings from a CSV file and output them
in the current directory for both the Android and iOS platforms:

```sh
$ strez ~/strings.csv .

# Output:
# ./values-en/strings.xml
# ./en.lproj/Localizable.strings
# ./en.lproj/Localizable.stringsdict
```

Same as above, but output them for the iOS platform only:

```sh
$ strez --platform=ios ~/strings.csv .

# Output:
# ./en.lproj/Localizable.strings
# ./en.lproj/Localizable.stringsdict
```

Assuming only English translations, read strings from a CSV file and output them
in another directory for the Android platform:

```sh
$ strez --platform=android ~/strings.csv MyProject/app/src/main/res

# Output:
# ./MyProject/app/src/main/res/values-en/my_strings.xml
```

Assuming English, Spanish, and French translations, read strings from a CSV file
and output them in the current directory for the Android platform, with English
as the default language:

```sh
$ strez --platform=android --default-language=en ~/strings.csv .

Output:
# ./values/strings.xml
# ./values-es/strings.xml
# ./values-fr/strings.xml
```

## CSV

### Columns

Your CSV must have column headers. Unrecognized columns are not allowed. When
duplicate columns are found, the last value for a duplicate column is used.

Column headers are described below. Columns marked as required _must_ be present
and have a value in all rows. Columns marked as optional may be entirely
omitted, or may be present without requiring a value in any row.

#### Key (Required)

The key of the string resource. This key is what will be used to refer to the
string resource in code.

#### Comment (Optional)

A comment for a string resource. Comments may be used to describe how the string
resource is used. Comments will appear above the string resource in the output.

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

Examples of valid strez string files in CSV format can be found in
[examples/csv](examples/csv).

## License

MIT &copy; Seth Lopez 2016