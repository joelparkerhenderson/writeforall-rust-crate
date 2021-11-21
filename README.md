# WriteForAll: tips to make text better

WriteForAll is a text file style checker, that compares text documents with editorial tips to make text better.

WriteForAll is similar in ways to text spell checkers, text style checkers, and text auto-suggestion tools.

Contents:

* [](#)


## Introduction


### Beginner commands

To get help, or version, or usage:

```
$ writeforall --help
$ writeforall --version
$ writeforall --usage
```


### Create tip.json

Create a file `tip.json` with this text:

```json
{
    "search": ["men", "women"], 
    "suggest": ["people", "persons"]
}
```


### Create example.txt

Create a file `example.txt` with this text:

```md
all men are created equal
```


### Run the command with --tip and --input

To run WriteForAll with a tip file name and input file name:

```sh
writeforall --tip tip.json --input example.txt
```

The result is this output:

```html
example.txt  4  men  person, people
```

The command line option can process multiple items such as:

```sh
writeforall --tip tip1.json tip2.json --input example1.txt example2.txt
```

The command line option can process a directory recursively such as:

```sh
writeforall --tip tips --input examples
```
