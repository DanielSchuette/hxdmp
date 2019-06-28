# hxdmp
## Overview
`hxdmp` is a Rust version of `hexdump -C`, i.e. outputs are similar if both commands are run on the same file (demonstrated by `./tools/test.sh`). This program was written because `hexdump(1)` cannot display the ASCII versions of unicode characters. Rust does this well, though. No other options of `hexdump(1)` are implement but that could be done pretty easily.

## License
The code in this repository is [GPLv3](./LICENSE.md)-licensed.
