CURRENT_USER=$(shell id -u)
CURRENT_GROUP=$(shell id -g)

.PHONY: test
test:
	cd "$(CURDIR)/pISO" && cargo test

.PHONY: test-ci
test-ci:
	docker run --rm \
		-e CARGO_HOME='$(CARGO_HOME)'\
		--user $(CURRENT_USER):$(CURRENT_GROUP) \
		-v "$(PWD)":$(PWD) -w $(PWD)/pISO rust:1 cargo test

.PHONY: sdimage
sdimage: update-config
ifeq ("$(shell ./scripts/should-rebuild)","rebuild")
	sudo docker run -v $(CURDIR):/pISO -w /pISO/buildroot \
			--user $(CURRENT_USER):$(CURRENT_GROUP) \
			--rm  adamschwalm/piso:latest /bin/bash -c "make clean && make"
else
	sudo docker run -v $(CURDIR):/pISO -w /pISO/buildroot \
			--user $(CURRENT_USER):$(CURRENT_GROUP) \
			--rm  adamschwalm/piso:latest /bin/bash -c "make piso-reconfigure && make"
endif
	git submodule > buildroot/output/.cache-version

.PHONY: sdimage-ci
sdimage-ci: update-config
	chmod +x buildroot/board/piso/post-build.sh
	chmod +x buildroot/board/piso/post-image.sh
ifeq ("$(shell ./scripts/should-rebuild)","rebuild")
	docker run -v $(CURDIR):/pISO -w /pISO/buildroot \
			--user $(CURRENT_USER):$(CURRENT_GROUP) \
			--rm  adamschwalm/piso:latest /bin/bash -c "make clean && make"
else
	docker run -v $(CURDIR):/pISO -w /pISO/buildroot \
			--user $(CURRENT_USER):$(CURRENT_GROUP) \
			--rm  adamschwalm/piso:latest /bin/bash -c "make piso-reconfigure && make"
endif
	git submodule > buildroot/output/.cache-version
	tar -cvzf sdcard.img.tar.gz buildroot/output/images/sdcard.img
	cp buildroot/output/images/rootfs.squashfs .

.PHONY: update-config
update-config:
	cd "$(CURDIR)/buildroot" && cp "configs/piso_defconfig" ".config"
