#!/bin/sh

BASEDIR=$(realpath $(dirname "$0"))

# Copy the config
(cd "$BASEDIR/buildroot" && cp "configs/raspberrypi0_defconfig" ".config")

CURRENT_USER=$UID
CURRENT_GROUP=$GID

sudo docker pull adamschwalm/piso:latest

# If we have previously built everything, assume we have just changed the
# piso package and reconfigure it. Otherwise, make everything.
if [ -f "$BASEDIR/buildroot/output/images/sdcard.img" ]; then
    sudo docker run -v $BASEDIR:/pISO -w /pISO/buildroot \
         --user $CURRENT_USER:$CURRENT_GROUP \
         --rm -i -t \
         adamschwalm/piso:latest /bin/bash -c "make piso-reconfigure && make"
else
    sudo docker run -v $BASEDIR:/pISO -w /pISO/buildroot \
         --user $CURRENT_USER:$CURRENT_GROUP \
         --rm -i -t \
         adamschwalm/piso:latest /bin/bash -c "make"
fi
