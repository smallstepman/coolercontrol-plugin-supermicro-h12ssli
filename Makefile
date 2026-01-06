.DEFAULT_GOAL := build
plugins_dir := '/etc/coolercontrol/plugins'
executable := 'custom-device'
service_id := 'custom-device'

.PHONY: clean build install

clean:
	@-$(RM) -rf target
	@-$(RM) -rf vendor

build:
	@cargo build --locked --release

install: build
	@sudo mkdir -p $(plugins_dir)/$(service_id)/ui
	@sudo install -m755 ./target/release/$(executable) $(plugins_dir)/$(service_id)
	@sudo install -m644 ./plugin-files/manifest.toml $(plugins_dir)/$(service_id)
	@sudo install -m644 ./plugin-files/ui/index.html $(plugins_dir)/$(service_id)/ui

run: build
	@sudo ./target/release/$(executable)

uninstall:
	@-sudo $(RM) -rf $(plugins_dir)/$(service_id)