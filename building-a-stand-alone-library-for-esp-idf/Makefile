image:
	@docker build -t esp-idf-lib .
.PHONY: image

build: image
	@docker run -it --rm \
		-v $(PWD)/library-root:/library-root \
		-w /library-root/example \
		esp-idf-lib \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py build \
		"
.PHONY: build

ssh: image
	@docker run -it --rm \
		-v $(PWD)/library-root:/library-root \
		-w /library-root/example \
		esp-idf-lib \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			bash \
		"
.PHONY: ssh
