#!/bin/sh

set -e

_VERBOSE=0

verbose_echo()
{
    if [[ $_VERBOSE -eq 1 ]]; then
        echo "$@"
    fi
}

# Given a volume name and a mount point, mount each partition
# of the drive and scan the partitions for ISOS. Each located
# iso is printed.
mount_vdrive()
{
    VOLUME_PATH="/dev/VolGroup00/$1"
    MOUNTPOINT=$2

    LOOPBACK_PATH=$(losetup -f)

    verbose_echo "Creating loopback devices"
    losetup -fP $VOLUME_PATH

    # Force (for real) a scan for partitions
    partprobe $LOOPBACK_PATH

    LOOPBACK_SUFFIX="p"
    PARTITIONS=$(find /dev -wholename "$LOOPBACK_PATH$LOOPBACK_SUFFIX*")

    verbose_echo "Located partitions: $PARTITIONS"

    for PARTITION in ${PARTITIONS}; do
        PART_MOUNT="$MOUNTPOINT/$(basename $PARTITION)"

        verbose_echo "Mounting $PARTITION to $PART_MOUNT"
        mkdir -p $PART_MOUNT
        mount $PARTITION $PART_MOUNT > /dev/null 2>&1

        #TODO scan for ISOs
    done
}

unmount_vdrive()
{
    MOUNTPOINT=$1

    for PARTITION in ${MOUNTPOINT}/*; do
        verbose_echo "Unmounting $PARTITION"
        if [[ $_VERBOSE -eq 1 ]]; then
            umount $PARTITION || true
        else
            umount $PARTITION > /dev/null 2>&1 || true
        fi
    done
}

while getopts ":v" OPTION
do
    case $OPTION in
        v)
            _VERBOSE=1
            shift
            ;;
    esac
done

case $1 in
    mount)
        mount_vdrive $2 $3
        ;;
    unmount)
        unmount_vdrive $2
        ;;
    *)
        echo "Usage: vdrive (mount|unmount) [args...]"
        exit 1
        ;;
esac
