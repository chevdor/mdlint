# mdlint

Port of [markdownlint](https://github.com/markdownlint/markdownlint) to Rust.

NOTE: This tool is not in a usable state yet.

## Install

```
cargo install --git https://github.com/chevdor/mdlint
```

## Rules covered

See [original rules](https://github.com/markdownlint/markdownlint/blob/master/docs/RULES.md).

| Rule ID | Completed | Tested | Description                                                   |
|---------|-----------|--------|---------------------------------------------------------------|
| MD001       |    ✅    |   ✅  | Header levels should only increment by one level at a time    |
| MD002       |    ✅    |   ✅  | First header should be a top level header                     |
| MD003       |    ✅    |   ✅  | Header style                                                  |
| MD004       |    ✅    |   ✅  | Unordered list style                                          |
| MD005       |    ❌    |   ❌  | Inconsistent indentation for list items at the same level     |
| MD006       |    ❌    |   ❌  | Consider starting bulleted lists at the beginning of the line |
| MD007       |    ❌    |   ❌  | Unordered list indentation                                    |
| MD009       |    ✅    |   ✅  | Trailing spaces Hard tabs                                     |
| MD010      |    ✅    |   ✅  | Hard tabs                                                     |
| MD011      |    ✅    |   ✅  | Reversed link syntax                                          |
| MD012      |    ✅    |   ✅  | Multiple consecutive blank lines                              |
| MD013      |    ❌    |   ❌  | Line length                                                   |
| MD014      |    ✅    |   ⚠️  | Dollar signs used before commands without showing output      |
| MD015      |    ❌    |   ❌  |                 |
| MD016      |    ❌    |   ❌  |                 |
| MD017      |    ❌    |   ❌  |                 |
| MD018      |    ✅    |   ✅  | No space after hash on atx style header                       |
| MD019      |    ❌    |   ❌  | Multiple spaces after hash on atx style header                |
| MD020      |    ❌    |   ❌  | No space inside hashes on closed atx style header             |
| MD021      |    ❌    |   ❌  | Multiple spaces inside hashes on closed atx style header      |
| MD022      |    ❌    |   ❌  | Headers should be surrounded by blank lines                   |
| MD023      |    ❌    |   ❌  | Headers must start at the beginning of the line               |
| MD024      |    ❌    |   ❌  | Multiple headers with the same content                        |
| MD025      |    ✅    |   ✅  | Multiple top level headers in the same document               |
| MD026      |    ❌    |   ❌  | Trailing punctuation in header                                |
| MD027      |    ❌    |   ❌  | Multiple spaces after blockquote symbol                       |
| MD028      |    ❌    |   ❌  | Blank line inside blockquote                                  |
| MD029      |    ❌    |   ❌  | Ordered list item prefix                                      |
| MD030      |    ❌    |   ❌  | Spaces after list markers                                     |
| MD031      |    ❌    |   ❌  | Fenced code blocks should be surrounded by blank lines        |
| MD032      |    ❌    |   ❌  | Lists should be surrounded by blank lines                     |
| MD033      |    ❌    |   ❌  | Inline HTML                                                   |
| MD034      |    ❌    |   ❌  | Bare URL used                                                 |
| MD035      |    ❌    |   ❌  | Horizontal rule style                                         |
| MD036      |    ❌    |   ❌  | Emphasis used instead of a header                             |
| MD037      |    ❌    |   ❌  | Spaces inside emphasis markers                                |
| MD038      |    ❌    |   ❌  | Spaces inside code span elements                              |
| MD039      |    ❌    |   ❌  | Spaces inside link text                                       |
| MD040      |    ❌    |   ❌  | Fenced code blocks should have a language specified           |
| MD041      |    ✅    |   ✅  | First line in file should be a top level header               |
| MD046      |    ❌    |   ❌  | Code block style                                              |
| MD047      |    ❌    |   ❌  |                                               |
| MD048      |    ❌    |   ❌  |                                               |
| MD049      |    ❌    |   ❌  |                                               |
| MD050      |    ❌    |   ❌  |                                               |
| MD051      |    ❌    |   ❌  |                                               |
| MD052      |    ❌    |   ❌  |                                               |
| MD053      |    ❌    |   ❌  |                                               |
| MD054      |    ❌    |   ❌  |                                               |
| MD055      |    ❌    |   ❌  |                                               |
| MD056      |    ❌    |   ❌  |                                               |
| MD057      |    ❌    |   ❌  |                                               |

completed: 9 / 38
