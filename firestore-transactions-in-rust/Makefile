run-basic:
	@docker compose -f dockerfiles/docker-compose-basic.yaml down --remove-orphans
	@docker compose -f dockerfiles/docker-compose-basic.yaml \
		up --build \
		--abort-on-container-exit \
		--exit-code-from example
.PHONY: run-basic

run-insert:
	@docker compose -f dockerfiles/docker-compose-insert.yaml down --remove-orphans
	@docker compose -f dockerfiles/docker-compose-insert.yaml \
		up --build \
		--abort-on-container-exit \
		--exit-code-from example
.PHONY: run-insert

run-conflict:
	@docker compose -f dockerfiles/docker-compose-conflict.yaml down --remove-orphans
	@docker compose -f dockerfiles/docker-compose-conflict.yaml \
		up --build \
		--abort-on-container-exit \
		--exit-code-from example
.PHONY: run-conflict
