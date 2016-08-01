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
    strez [OPTIONS] --target_format <target_format> [input]

OPTIONS:
    -d, --directory <directory>            Set the output directory. [default: .] 
    -f, --force                            Overwrite existing files with the same output filename(s).
    -h, --help                             Prints help information
    -n, --name <name>                      Set the name of the output file(s). Defaults to platform conventions.
    -s, --source_format <source_format>    Set the input format. [default: csv]  [values: csv]
    -t, --target_format <target_format>    Set the output format. Accepts multiple comma-delimited values. [values: android, ios]
    -V, --version                          Prints version information
    -v, --verbose                          Display verbose output.

ARGS:
    <input>    Set the input file. Defaults to stdin. [default: -]
```

### Options

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
# ./Localizable.strings
# ./Localizable.stringsdict
```

Same as above, but output both iOS and Android formats:

```sh
$ strez -t android,ios ~/strings.csv

# Output:
# ./values/strings.xml
# ./Localizable.strings
# ./Localizable.stringsdict
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
# ./Localizable.strings
```

## CSV

### Headers

Your CSV should always include column headers. Data in columns with unrecognized
headers or duplicate headers will be ignored. Descriptions of headers are below.

#### Name (Required)

The name of the string resource. This name will be transformed into snake_case
for output.

#### Value (Required)

The value of the string resource. Also serves as the default language resource.
*See [ISO 639-1 Language Headers](#iso-639-1-language-headers-optional).*

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

#### ISO 639-1 Language Headers (Optional)

When providing translated strings for multiple languages, additional headers
can be defined that correspond to the language codes in
[ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).

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