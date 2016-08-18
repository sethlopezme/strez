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
    strez [OPTIONS] --target-format <target-format> [input]

OPTIONS:
    -l, --default-language <default-language>    Set the default language. Defaults to the first defined language.
    -d, --directory <directory>                  Set the output directory. [default: .] 
    -f, --force                                  Overwrite existing files with the same output filename(s).
    -h, --help                                   Prints help information
    -n, --name <name>                            Set the name of the output file(s). Defaults to platform conventions.
        --no-default-language                    Output the default language to the appropriate language directory.
    -s, --source-format <source-format>          Set the input format. [default: csv]  [values: csv]
    -t, --target-format <target-format>          Set the output format. Accepts multiple comma-delimited values. [values: android, ios]
    -V, --version                                Prints version information
    -v, --verbose                                Display verbose output.

ARGS:
    <input>    Set the input file. Defaults to stdin.
```

### Options

#### `-l`, `--default-language`

Default: The first language column defined in left-to-right order.

Set the default language. The default language is output to the default strings
files for each platform:

- Android: `values/strings.xml`
- iOS: `Base.lproj/Localizable.strings`/`Base.lproj/Localizable.stringsdict`

#### `-d`, `--directory`

Default: `.`

Set the output directory. Localization directories will be created within the
output directory if they do not exist.

#### `-f`, `--force`

Overwrite existing files with the same output filename(s).

#### `-n`, `--name`

Default:

- Android: `strings`
- iOS: `Localizable`

Set the name of the output file(s).

#### `--no-default-language`

Output the default language to the appropriate language directory.
_See [default language](#-l---default-language)._

#### `-s`, `--source-format`

Default: `csv`

Set the input format. Valid formats are:

- `csv`

#### `-t`, `--target-format`

Set the output format. Valid formats are:

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

Using shell redirection, read string resources from stdin and output them to
`Localizable.strings` in the iOS format:

```sh
$ cat ~/strings.csv | strez -t ios

# Output:
# ./Base.lproj/Localizable.strings
```

## CSV

### Headers

Your CSV must have column headers. Data in columns with unrecognized
headers or duplicate headers will be ignored. Descriptions of headers are below.

#### Name (Required)

The name of the string resource. This name will be transformed into snake_case
for output.

#### Default (Required; See Below)

The default-language value of the string resource. Required unless there is at
least one language column.
_See [ISO 639-1 Language Headers](#iso-639-1-language-headers-optional)._

Default values are output to a `values` directory for Android and a `Base.lproj`
directory for iOS. Values within language columns are output to their respective
directories.

#### ISO 639-1 Language Headers (Required)

When defining values, use headers that correspond to the language codes in
[ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes). Multiple
translations can be defined by using a different language code header for each
value column.

Values are output to their respective language directories, i.e. `fr` values are
output to the `values-fr` (Android) and `fr.lproj` (iOS) directories.

#### Description (Optional)

The description of the string resource. This description will appear as a
comment above the resource in the output.

#### Plural (Optional)

The type of plural for the string resource, if applicable. When defining
plurals, be sure to use the same name for each plural resource.
Valid plurals are:

- zero
- one
- two
- few
- many
- other

### Examples

Basic example showing how to define string resources with a name, description,
and value.

```
Name,Description,Value
navigation_dashboard,Dashboard navigation button,Dashboard
navigation_locations,Locations navigation button,Locations
navigation_my_account,My Account navigation button,My Account
navigation_messages,Messages navigation button,Messages
navigation_messages_count,Badge text for the Messages navigation button,%1$d Unread
```

A more advanced example showing additional columns for plurals and
Spanish-language translations.

```
Name,Description,Plural,Value,es
message_list_title,Title of the message list screen,,Messages,Mensajes
message_list_header_count,Count of the unread messages in the message list,one,%1$d Unread Message,%1$d Mensaje No Leído
message_list_header_count,Count of the unread messages in the message list,other,%1$d Unread Messages,%1$d Mensajes No Leídos
message_list_row_delete,Button text for deleting a message in the message list,,Delete,Borrar
```

## License

MIT &copy; Seth Lopez