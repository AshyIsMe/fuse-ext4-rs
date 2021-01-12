define PARTTABLE
;
write
endef

ext4fs.img: parttable.txt
	dd if=/dev/zero bs=1M count=1000 > ext4fs.img
	echo "$$PARTTABLE" > parttable.txt
	sfdisk ext4fs.img < parttable.txt
	# TODO: use loopback device and then mkfs.ext4 the partition

export PARTTABLE
parttable.txt:
	echo "$$PARTTABLE" > parttable.txt

clean:
	rm ext4fs.img
	rm parttable.txt
