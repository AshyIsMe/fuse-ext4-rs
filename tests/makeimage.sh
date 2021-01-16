
dd if=/dev/zero bs=1M count=1000 > ext4fs.img
# use loopback device and then mkfs.ext4 the "device"
LOOPDEV=$(losetup --show --find ext4fs.img)
mkfs -t ext4 "$LOOPDEV"

TMPDIR=$(mktemp -d)
mount "$LOOPDEV" "$TMPDIR"

echo "$TMPDIR"

# Create some test files
cd "$TMPDIR"
uname -a > uname.txt
iostat > iostat.txt
cd -

umount "$TMPDIR"
losetup -d "$LOOPDEV"
