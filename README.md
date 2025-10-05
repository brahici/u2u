# u2u: ulid to uuid

u2u is a command line tool for converting ulids to uuids, back and forth.

## Install

Either use `cargo build` or provided `Makefile`.

By default, install copies the binary to `/usr/local/bin`. Target directory can be overriden with `TARGET` variable.

```console
TARGET=~/local/.bin make install
[...]
```

## Quickstart

```console
# convert a ULID
$ u2u 01K59FJBYATAY80B0ZKHEG28B7
[L2U]:: 01K59FJBYATAY80B0ZKHEG28B7           :: 019952f9-2fca-d2bc-802c-1f9c5d012167

# convert a UUID
$ u2u 019952f9-2fca-d2bc-802c-1f9c5d012167
[U2L]:: 019952f9-2fca-d2bc-802c-1f9c5d012167 :: 01K59FJBYATAY80B0ZKHEG28B7

# multiple values can be passed
$ u2u 01K59FJBYATAY80B0ZKHEG28B7 01K5EQ3RZDG1DMKAGY42MR3BZX
[L2U]:: 01K59FJBYATAY80B0ZKHEG28B7           :: 019952f9-2fca-d2bc-802c-1f9c5d012167
[L2U]:: 01K5EQ3RZDG1DMKAGY42MR3BZX           :: 01995d71-e3ed-805b-49aa-1e20a981affd

# ULIDs and UUIDs can be mixed
$ u2u 01K59FJBYATAY80B0ZKHEG28B7 019952f9-2fca-d2bc-802c-1f9c5d012167
[L2U]:: 01K59FJBYATAY80B0ZKHEG28B7           :: 019952f9-2fca-d2bc-802c-1f9c5d012167
[U2L]:: 019952f9-2fca-d2bc-802c-1f9c5d012167 :: 01K59FJBYATAY80B0ZKHEG28B7

# failures are reported but don't break the whole processing
# u2u 01K59FJBYATAY80B0ZKHEG28B7 the_heck_is_this 01K5EQ3RZDG1DMKAGY42MR3BZX
[L2U]:: 01K59FJBYATAY80B0ZKHEG28B7           :: 019952f9-2fca-d2bc-802c-1f9c5d012167
[ERR]:: the_heck_is_this                     :: can't parse the value
[L2U]:: 01K5EQ3RZDG1DMKAGY42MR3BZX           :: 01995d71-e3ed-805b-49aa-1e20a981affd
```
