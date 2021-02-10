
dd if=/dev/zero bs=1M count=10000 > ext4fs.img
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

#curl -O https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/snapshot/linux-5.11-rc7.tar.gz
cp ~/Downloads/linux-5.11-rc7.tar.gz .
tar xzvf linux-5.11-rc7.tar.gz

cd -

umount "$TMPDIR"
losetup -d "$LOOPDEV"
