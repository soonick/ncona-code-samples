build:
	@docker build -t introduction-to-axum .
.PHONY: build

start-hello-world: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/hello-world \
		-p 3000:3000 \
		introduction-to-axum \
		cargo run
.PHONY: start-hello-world

start-extractors: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/extractors \
		-p 3000:3000 \
		introduction-to-axum \
		cargo run
.PHONY: start-extractors

start-multiple-extractors: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/multiple-extractors \
		-p 3000:3000 \
		introduction-to-axum \
		cargo run
.PHONY: start-multiple-extractors

start-debug-handler: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/debug-handler \
		-p 3000:3000 \
		introduction-to-axum \
		cargo run
.PHONY: start-debug-handler

start-rest: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/rest \
		-p 3000:3000 \
		introduction-to-axum \
		cargo run
.PHONY: start-rest

start-errors: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/errors \
		-p 3000:3000 \
		introduction-to-axum \
		cargo run
.PHONY: start-errors
