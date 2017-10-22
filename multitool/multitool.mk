################################################################################
#
# libfoo
#
################################################################################

MULTITOOL_VERSION = 0.5
MULTITOOL_SITE = /home/adam/Repos/usb-multitool/multitool
MULTITOOL_SITE_METHOD:=local

$(eval $(cmake-package))
