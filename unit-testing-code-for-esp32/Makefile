image:
	@docker build -t esp-idf-lib-test .
.PHONY: image

ssh: image
	@docker run -it --rm \
		-v $(PWD)/library-root:/library-root \
		-w /library-root \
		esp-idf-lib-test \
		bash
.PHONY: ssh

test: image
	@docker run -it --rm \
		-v $(PWD)/library-root:/library-root \
		-w /library-root/test/build/ \
		esp-idf-lib-test \
		sh -c " \
			cmake .. && \
			make && \
			./test \
		"
.PHONY: test
