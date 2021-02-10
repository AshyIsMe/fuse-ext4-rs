# Fuse ext4 in Rust

## Testing

```
sudo setfacl -m u:${USER}:r /dev/sda1  # good until reboot/re-attach

cargo run /dev/sda1 ./mnt
```

## TODO

* Draw the rest of the effing Owl:
  * https://github.com/wfraser/fuse-mt/tree/master/example
  * Various fixes:
```
09:33:09        Fauxux | It definitely needs some explicit panics fixing, some maths hardening, some type conversion hardening (if you're gonna run it on
                       | non-amd64), and read is O(n*m) in the length of the file, and how many reads you make, I think, which should be fixed.
09:33:54        Fauxux | Moving ext4-rs' itself's TreeReader (which reads extents) over to positioned-io, or a fork thereof, would be a good start.
```

### Latest bug:

Walking the linux kernel git tree crashes.

```
sudo ./tests/makeimage.sh # Now grabs a kernel tarball and extracts it
cargo run ./tests/ext4fs.img ./mnt

find ./mnt/linux*
```

```
aaron@warthog:~/codebases/fuse-ext4-rs$ cargo run tests/ext4fs.img ./mnt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
         Running `target/debug/fuse-ext4 tests/ext4fs.img ./mnt`
         open_raw_or_first_partition() ok
         thread 'main' panicked at 'valid inode: assumption failed: "directory checksum mismatch: on-disk: 3be383ff, computed: 283efcd8"', src/ext4fs.rs:265:47
         note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```
