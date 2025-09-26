run:
	@docker run -it --rm \
		-v $(PWD)/generate-url:/generate-url \
		-v $(PWD)/service-account.json:/service-account.json \
		-w /generate-url \
		-e GOOGLE_APPLICATION_CREDENTIALS="/service-account.json" \
		rust:1.90 \
		sh -c "RUST_BACKTRACE=1 cargo run"
.PHONY: run
