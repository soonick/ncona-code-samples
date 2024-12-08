build:
	@docker build -t sending-emails-from-rust-with-brevo .
.PHONY: build

run: build
	@docker run --rm -it -p 3000:3000 \
		-v $(PWD)/src:/app \
		--env-file .env \
		-w /app/ \
		sending-emails-from-rust-with-brevo \
		cargo run
.PHONY: run

ssh: build
	@docker run --rm -it -p 3000:3000 \
		-v $(PWD)/src:/app \
		--env-file .env \
		-w /app/ \
		sending-emails-from-rust-with-brevo \
		sh
.PHONY: ssh
