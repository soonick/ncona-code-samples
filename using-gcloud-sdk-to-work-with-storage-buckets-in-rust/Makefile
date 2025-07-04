build:
	@docker build -t gcloud-storage-example .
.PHONY: client

run: build
	@docker run -it --rm \
		-e GOOGLE_APPLICATION_CREDENTIALS=/example/key.json \
		-v $(PWD):/example \
		-w /example \
		gcloud-storage-example \
		cargo run
.PHONY: run
