build:
	@docker build -t error-handling-in-rust .
.PHONY: build

run-index-out-of-bounds: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/index-out-of-bouds \
		error-handling-in-rust \
		cargo run
.PHONY: run-index-out-of-bounds

run-panic: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/panic \
		error-handling-in-rust \
		cargo run
.PHONY: run-panic

run-simple-result: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/simple-result \
		error-handling-in-rust \
		cargo run
.PHONY: run-simple-result

run-creating-our-results: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/creating-our-results \
		error-handling-in-rust \
		cargo run
.PHONY: run-creating-our-results

run-propagating-errors: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/propagating-errors\
		error-handling-in-rust \
		cargo run
.PHONY: run-propagating-errors

run-using-traits: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/using-traits \
		error-handling-in-rust \
		cargo run
.PHONY: run-using-traits

run-own-errors: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/own-errors \
		error-handling-in-rust \
		cargo run
.PHONY: run-own-errors

run-different-errors: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples/different-errors \
		error-handling-in-rust \
		cargo run
.PHONY: run-different-errors

ssh: build
	@docker run -it --rm \
		-v $(PWD)/examples:/examples\
		-w /examples \
		error-handling-in-rust \
		bash
.PHONY: ssh
