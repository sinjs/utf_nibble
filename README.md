# nibbles
### a custom encoding that uses 4 bits [one nibble] per character

## Supported Characters

a, b, d, e, f, g, i, j, k, n, m, o, p, r, s, t

## Compiling

```shell
cargo build # Development mode
cargo build --release # Release mode
```

## Running

```shell
cargo run # Development mode
cargo run --release # Release mode
```

## Usage

```shell
utf_nibble [encode|decode] [file]
# Using an unix like shell, you can:
# utf_nibble encode input.txt > output.txt
# utf_nibble decode output.txt > decoded.txt
```
