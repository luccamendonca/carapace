.PHONY: check-env
check-env:
ifndef ALLOWED_CMD_PATH
	$(error ALLOWED_CMD_PATH is undefined)
endif

.PHONY: check-deps
check-deps:
	@which rustc
	@which cargo

.PHONY: server
server: check-env
	cargo run --bin carapace-server

.PHONY: client
client:
	cargo run --bin carapace-client
