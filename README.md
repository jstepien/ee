## ee

dee-dee but the d is silent

### Usage

```
$ ee -h
USAGE:
    ee [OPTIONS]

OPTIONS:
    -b, --bs <BYTES>       Copy BYTES bytes at a time
    -c, --count <COUNT>    Copy COUNT times
    -h, --help             Print help information
    -i, --if <INPUT>       Read from INPUT in lieu of stdin
    -o, --of <OUTPUT>      Write to OUTPUT in lieu of stdout

$ ee -i /dev/zero -o zeroes -b 1M -c 1024
1024+0 records in
1024+0 records out
1073741824 bytes transferred in 0.575380 secs (1866143808 bytes/sec)
```

### Building

Use [Cargo](https://doc.rust-lang.org/cargo/).

```
cargo build
```

### Copying

See LICENSE
