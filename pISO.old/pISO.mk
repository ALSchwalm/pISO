################################################################################
#
# pISO
#
################################################################################

PISO_VERSION = 0.20
PISO_SITE = /home/adam/repos/pISO/pISO-rs
PISO_SITE_METHOD:=local

HOST_CARGO_HOME = $(HOST_DIR)/share/cargo
HOST_CARGO_ENV = \
	RUSTFLAGS="-Clink-arg=-Wl,-rpath,$(HOST_DIR)/lib" \
	CARGO_HOME=$(HOST_CARGO_HOME) \
	CC=arm-buildroot-linux-gnueabihf-gcc

define PISO_BUILD_CMDS
	(cd $(@D); \
		$(HOST_MAKE_ENV) $(HOST_CARGO_ENV) \
		$(HOST_DIR)/bin/cargo build --target=$(RUST_TARGET_NAME) --release)
endef

define PISO_INSTALL_TARGET_CMDS
	$(INSTALL) -D -s --strip-program=$(HOST_DIR)/bin/arm-linux-strip \
		-m 0755 $(@D)/target/$(RUST_TARGET_NAME)/release/pISO-rs \
		$(TARGET_DIR)/usr/bin/pISO-rs
endef

$(eval $(generic-package))
