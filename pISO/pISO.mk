################################################################################
#
# libfoo
#
################################################################################

PISO_VERSION = 0.9
PISO_SITE = /home/adam/repos/pISO/pISO
PISO_SITE_METHOD:=local
PISO_DEPENDENCIES += wiringpi

$(eval $(cmake-package))
