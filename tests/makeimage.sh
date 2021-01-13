
dd if=/dev/zero bs=1M count=1000 > ext4fs.img
# use loopback device and then mkfs.ext4 the "device"
LOOPDEV=$(losetup --show --find ext4fs.img)
mkfs -t ext4 "$LOOPDEV"
losetup -d "$LOOPDEV"

