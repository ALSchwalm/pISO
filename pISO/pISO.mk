################################################################################
#
# libfoo
#
################################################################################

PISO_VERSION = 0.9
PISO_SITE = /home/adam/Repos/usb-multitool/pISO
PISO_SITE_METHOD:=local
PISO_DEPENDENCIES += wiringpi

$(eval $(cmake-package))
