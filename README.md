pnc-id
======
Simple Rust application that converts a PNC Id into either its base32 encoded
string, or its u64/long decoded form, depending on whether we can parse the id as a u64.

## Usage
```bash
# converts base32 encoded string to its u64 form
$ pnc-id BCQQO2QMV7AAA

# converts the u64 into its base32 encoded string
$ pnc-id 621786375614611456
```

## Build
```bash
$ cargo build

# if doing the final version
$ cargo build --release
```
