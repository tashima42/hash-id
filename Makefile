OSC_DIR = ./.osc

.PHONY: prepare-osc
prepare-osc:
	@echo "Preparing cargo vendor for osc"
	@mkdir -p $(OSC_DIR)
	cargo vendor $(OSC_DIR)/vendor
	@echo "Creating .cargo/config"
	mkdir -p .cargo
	echo '[source.crates-io]' > .cargo/config
	echo 'replace-with = "vendored-sources"' >> .cargo/config
	echo '[source.vendored-sources]' >> .cargo/config
	echo 'directory = "./$(OSC_DIR)/vendor"' >> .cargo/config
	@echo "Creating source tarball"
	tar -czf hash-id.tar.gz $(OSC_DIR)

