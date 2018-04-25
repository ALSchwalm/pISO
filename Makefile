CURRENT_USER=$(shell id -u)
CURRENT_GROUP=$(shell id -g)

test:
	cd "$(CURDIR)/pISO" && cargo test

test-ci:
	docker run --rm \
     -e CARGO_HOME='$(CARGO_HOME)'\
     --user $(CURRENT_USER):$(CURRENT_GROUP) \
     -v "$(PWD)":$(PWD) -w $(PWD)/pISO rust:1 cargo test

sdimage: update-config
	sudo docker run -v $(CURDIR):/pISO -w /pISO/buildroot \
         --user $(CURRENT_USER):$(CURRENT_GROUP) \
         --rm adamschwalm/piso:latest make

sdimage-ci: update-config
	docker run -v $(CURDIR):/pISO -w /pISO/buildroot \
         --user $(CURRENT_USER):$(CURRENT_GROUP) \
         --rm  adamschwalm/piso:latest make

update-config:
	cd "$(CURDIR)/buildroot" && cp "configs/raspberrypi0_defconfig" ".config"
