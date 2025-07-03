build-server:
	@docker build -t tonic-server .
.PHONY: build-server

build-client:
	@docker build -t tonic-client .
.PHONY: build-client

start-server: build-server
	@docker network create tonic-network || true
	@docker run -it --rm \
		--name tonic-server \
		--network tonic-network \
		-p 50051:50051 \
		-v $(PWD)/projects:/projects \
		-w /projects/server \
		tonic-server \
		cargo run
.PHONY: start-server

run-client: build-client
	@docker run -it --rm \
		--network tonic-network \
		-v $(PWD)/projects:/projects \
		-w /projects/client \
		tonic-client \
		cargo run
.PHONY: run-client
