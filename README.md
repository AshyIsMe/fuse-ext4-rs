# Fuse ext4 in Rust

## Testing

```
sudo setfacl -m u:${USER}:r /dev/sda1  # good until reboot/re-attach

cargo run /dev/sda1 ./mnt
```

## TODO

* Draw the rest of the effing Owl:
  * https://github.com/wfraser/fuse-mt/tree/master/example
