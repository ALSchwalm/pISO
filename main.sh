#!/bin/sh

set -e

USER_PARTITION=/dev/mmcblk0p3 #/dev/mmcblk0p2
USER_PARTITION_MNT=/mnt
CONFIGFS_HOME=/sys/kernel/config/
GADGET_ROOT=$CONFIGFS_HOME/usb_gadget/g1

mount_user_partition()
{
    echo "Mounting user partition ($USER_PARTITION)"

    # Don't assume the partition type
    mount $USER_PARTITION $USER_PARTITION_MNT
}

add_entry()
{
    ID=$1
    FILE=$2
    CD=$3
    mkdir -p $GADGET_ROOT/functions/mass_storage.$ID/lun.0/

    echo 1 > $GADGET_ROOT/functions/mass_storage.$ID/stall
    echo $CD > $GADGET_ROOT/functions/mass_storage.$ID/lun.0/cdrom
    echo $CD > $GADGET_ROOT/functions/mass_storage.$ID/lun.0/ro
    echo "$FILE" > $GADGET_ROOT/functions/mass_storage.$ID/lun.0/file

    ln -s $GADGET_ROOT/functions/mass_storage.$ID $GADGET_ROOT/configs/c.1/
}

add_entry_for_user_partition()
{
    echo "Adding function for user partition"
    add_entry "user" $USER_PARTITION 0
}

add_entries_for_folder()
{
    FOLDER=$1
    PREFIX=$2
    CD=$3
    if [ ! -d $USER_PARTITION_MNT/$FOLDER ]; then
        echo "No $FOLDER folder, skipping."
        return
    fi

    local ITEMS=$USER_PARTITION_MNT/$FOLDER/*
    local ID=0
    for item in $ITEMS
    do
        # Work around idiotic sh nonsense
        [ -e "$item" ] || continue

        echo "Adding function for '$item'"

        add_entry "$PREFIX$ID" $item $CD

        let ID=++ID
    done
}

init_configfs()
{
    echo "Initializing ConfigFS gadget root in '$GADGET_ROOT'"

    local VENDOR_ID=0x1d6b  # Linux Foundation
    local PRODUCT_ID=0x0104 # Multifunction Composite Gadget
    local DEVICE_BCD=0x0100 # v1.0.0
    local USB_BCD=0x0200    # USB2

    echo $VENDOR_ID > $GADGET_ROOT/idVendor
    echo $PRODUCT_ID > $GADGET_ROOT/idProduct
    echo $DEVICE_BCD > $GADGET_ROOT/bcdDevice
    echo $USB_BCD > $GADGET_ROOT/bcdUSB

    mkdir -p $GADGET_ROOT/strings/0x409

    local SERIAL_NUMBER="0000000000000000"
    local MANUFACTURER="Adam Schwalm"
    local PRODUCT="USB MultiTool"

    echo $SERIAL_NUMBER > $GADGET_ROOT/strings/0x409/serialnumber
    echo $MANUFACTURER > $GADGET_ROOT/strings/0x409/manufacturer
    echo $PRODUCT > $GADGET_ROOT/strings/0x409/product

    mkdir -p $GADGET_ROOT/configs/c.1/strings/0x409

    local MAX_POWER=250
    local CONFIGURATION="Config 1"

    echo $MAX_POWER > $GADGET_ROOT/configs/c.1/MaxPower
    echo $CONFIGURATION > $GADGET_ROOT/configs/c.1/strings/0x409/configuration
}

create_device()
{
    echo "Starting device"

    ls /sys/class/udc > $GADGET_ROOT/UDC
}

# Load the necessary modules
modprobe dm_thin_pool # for the lvm thin pool
modprobe dwc2
modprobe libcomposite

# Then get configfs up and running
mount none /sys/kernel/config -t configfs

mount_user_partition

mkdir -p $GADGET_ROOT

init_configfs

# add_entry_for_user_partition
add_entries_for_folder "ISOS" "iso" 1
add_entries_for_folder "HDS" "hd" 0

create_device
