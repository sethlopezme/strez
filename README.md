# strez

Convert string resources from CSV to Android and iOS formats. A good way to keep your strings in sync between platforms.

## Install

Installation instructions here.

## Usage

```
$ strez --help

Usage:
    strez [OPTIONS] INPUT [DIRECTORY [NAME]]
  
Options:
    -d, --directory     Set the output directory.
    -f, --force         Overwrite existing files.
    -h, --help          Display this message.
    -n, --name          Set the name of the output files.
    -s, --source-format Set the input file format.
    -t, --target-format Set the output file format.
    -V, --version       Display version info and exit.
```

### Options

#### `-d`, `--directory`

Default: `.`

Set the output directory. Localization directories will be created within the output directory if they do not exist.

#### `-f`, `--force`

Overwrite existing files with the same filename as the output file.

#### `-n`, `--name`

Default:

- Android: `strings`
- iOS: `Localizable`

Set the name of the output file.

#### `-s`, `--source-format`

Default: `csv`

Set the input file format. Valid formats are:

- `csv`

#### `-t`, `--target-format`

Set the output file format. Valid options are:

- `android`
- `ios`

### Examples

Read string resources from a CSV file and output them to the current directory in the iOS format.

```sh
$ strez -t ios ~/strings.csv

# Output:
# ./Localizable.strings
# ./Localizable.stringsdict
```

Read string resources from a CSV file and output them as `my_strings.xml` to a another directory in the Android format.

```sh
$ strez -t android ~/strings.csv MyProject/app/src/main/res my_strings

# Output:
# ./MyProject/app/src/main/res/values/my_strings.xml
```

An alternative to the above example using options.

```sh
$ strez -t android -d MyProject/app/src/main/res -n my_strings ~/strings.csv

# Output:
# ./MyProject/app/src/main/res/values/my_strings.xml
```

Using shell redirection, read string resources from stdin and output them to `Localizable.strings` in the iOS format.

```sh
$ strez -t ios < cat ~/strings.csv

# Output:
# ./Localizable.strings
```

## CSV

### Headers

Your CSV should always include column headers. If headers are not present, strez will assume the first column is the
string name and the second column is the string value.

#### Name (Required)

The name of the string resource. This name will be transformed into snake_case for output.

#### Value (Required)

The value of the string resource. Also serves as the default language resource.
*See [ISO 639-1 Language Headers](#iso-639-1-language-headers-optional).*

#### Description (Optional)

The description of the string resource. This description will appear as a comment above the resource in the output.

#### Plural (Optional)

The type of plural for the string resource, if applicable. When defining plurals, be sure to use the same name for each
plural resource. Valid plurals are:

- zero
- one
- two
- few
- many
- other

#### ISO 639-1 Language Headers (Optional)

When providing translated strings for multiple languages, additional headers can be defined that correspond to the
language codes in [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).

### Examples

Basic example showing how to define string resources with a name, description, and value.

```
Name,Description,Value
navigation_dashboard,Dashboard navigation button,Dashboard
navigation_locations,Locations navigation button,Locations
navigation_my_account,My Account navigation button,My Account
navigation_messages,Messages navigation button,Messages
navigation_messages_count,Badge text for the Messages navigation button,%1$d Unread
```

A more advanced example showing additional columns for plurals and Spanish-language translations.

```
Name,Description,Plural,Value,es
message_list_title,Title of the message list screen,,Messages,Mensajes
message_list_header_count,Count of the unread messages in the message list,one,%1$d Unread Message,%1$d Mensaje No Leído
message_list_header_count,Count of the unread messages in the message list,other,%1$d Unread Messages,%1$d Mensajes No Leídos
message_list_row_delete,Button text for deleting a message in the message list,,Delete,Borrar
```

## License

MIT &copy; Seth Lopez