.PHONY: check-env
check-env:
ifndef ALLOWED_CMD_PATHS
	$(error ALLOWED_CMD_PATHS is undefined)
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

.PHONY: gen-client-js
gen-client-js:
	cd proto && \
	protoc --proto_path=. carapace_command.proto \
		--js_out=import_style=commonjs:clients/js \
		--grpc-web_out=import_style=commonjs,mode=grpcwebtext:clients/js
