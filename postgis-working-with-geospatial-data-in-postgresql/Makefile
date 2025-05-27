image:
	@docker build -t postgresql-geo-image .
.PHONY: image

start: image
	@docker run -it --rm \
		-e POSTGRES_PASSWORD=mypassword \
		--name postgresql-geo \
		postgresql-geo-image
.PHONY: start

connect:
	@docker exec -it postgresql-geo bash
.PHONY: connect
