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
