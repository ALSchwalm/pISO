#!/bin/sh

set -e

# Given a drive path (/dev/VolGroup00/volume1) and a destination,
# mount each partition of the drive and scan the partitions for ISOS.
# Each located iso is printed.

VOLUME_PATH=$1
MOUNTPOINT=$2

LOOPBACK_PATH=$(losetup -f)

losetup -fP $VOLUME_PATH

# Force (for real) a scan for partitions
partprobe $LOOPBACK_PATH

LOOPBACK_SUFFIX="p"
PARTITIONS=$(find /dev -wholename "$LOOPBACK_PATH$LOOPBACK_SUFFIX*")

for PARTITION in ${PARTITIONS}; do
    PART_MOUNT="$MOUNTPOINT/$(basename $PARTITION)"

    echo "Mounting $PARTITION to $PART_MOUNT"
    mkdir -p $PART_MOUNT
    mount $PARTITION $PART_MOUNT
done
