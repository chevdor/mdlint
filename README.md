# mdlint

[![Build Status](https://travis-ci.org/robertohuertasm/mdlint.svg?branch=master)](https://travis-ci.org/robertohuertasm/mdlint)

[![Build Status](https://dev.azure.com/robertohuertasm/github-oss/_apis/build/status/robertohuertasm.mdlint)](https://dev.azure.com/robertohuertasm/github-oss/_build/latest?definitionId=1)

Port of [markdownlint](https://github.com/markdownlint/markdownlint) to Rust.

## Run the project

Before running anything, make sure you have installed `clippy` and `fmt`:

```sh
rustup component add clippy-preview --toolchain=nightly

rustup component add rustfmt-preview --toolchain nightly
```

Just run:

```sh
cargo run
```

## Running clippy

`cargo clippy --all -- -D clippy::pedantic`

## Rules covered

See [original rules](https://github.com/markdownlint/markdownlint/blob/master/docs/RULES.md).

| Rule ID | Completed | Tested | Description                                                   |
|---------|-----------|--------|---------------------------------------------------------------|
| 1       |    [x]    |   [x]  | Header levels should only increment by one level at a time    |
| 2       |    [x]    |   [x]  | First header should be a top level header                     |
| 3       |    [x]    |   [x]  | Header style                                                  |
| 4       |    [x]    |   [x]  | Unordered list style                                          |
| 5       |    [ ]    |   [ ]  | Inconsistent indentation for list items at the same level     |
| 6       |    [ ]    |   [ ]  | Consider starting bulleted lists at the beginning of the line |
| 7       |    [ ]    |   [ ]  | Unordered list indentation                                    |
| 9       |    [x]    |   [x]  | Trailing spaces Hard tabs                                     |
| 10      |    [x]    |   [x]  | Hard tabs                                                     |
| 11      |    [x]    |   [x]  | Reversed link syntax                                          |
| 12      |    [x]    |   [x]  | Multiple consecutive blank lines                              |
| 13      |    [ ]    |   [ ]  | Line length                                                   |
| 14      |    [x]    |   [x]  | Dollar signs used before commands without showing output      |
| 18      |    [x]    |   [x]  | No space after hash on atx style header                       |
| 19      |    [ ]    |   [ ]  | Multiple spaces after hash on atx style header                |
| 20      |    [ ]    |   [ ]  | No space inside hashes on closed atx style header             |
| 21      |    [ ]    |   [ ]  | Multiple spaces inside hashes on closed atx style header      |
| 22      |    [ ]    |   [ ]  | Headers should be surrounded by blank lines                   |
| 23      |    [ ]    |   [ ]  | Headers must start at the beginning of the line               |
| 24      |    [ ]    |   [ ]  | Multiple headers with the same content                        |
| 25      |    [x]    |   [x]  | Multiple top level headers in the same document               |
| 26      |    [ ]    |   [ ]  | Trailing punctuation in header                                |
| 27      |    [ ]    |   [ ]  | Multiple spaces after blockquote symbol                       |
| 28      |    [ ]    |   [ ]  | Blank line inside blockquote                                  |
| 29      |    [ ]    |   [ ]  | Ordered list item prefix                                      |
| 30      |    [ ]    |   [ ]  | Spaces after list markers                                     |
| 31      |    [ ]    |   [ ]  | Fenced code blocks should be surrounded by blank lines        |
| 32      |    [ ]    |   [ ]  | Lists should be surrounded by blank lines                     |
| 33      |    [ ]    |   [ ]  | Inline HTML                                                   |
| 34      |    [ ]    |   [ ]  | Bare URL used                                                 |
| 35      |    [ ]    |   [ ]  | Horizontal rule style                                         |
| 36      |    [ ]    |   [ ]  | Emphasis used instead of a header                             |
| 37      |    [ ]    |   [ ]  | Spaces inside emphasis markers                                |
| 38      |    [ ]    |   [ ]  | Spaces inside code span elements                              |
| 39      |    [ ]    |   [ ]  | Spaces inside link text                                       |
| 40      |    [ ]    |   [ ]  | Fenced code blocks should have a language specified           |
| 41      |    [x]    |   [x]  | First line in file should be a top level header               |
| 46      |    [ ]    |   [ ]  | Code block style                                              |

completed: 9 / 38
