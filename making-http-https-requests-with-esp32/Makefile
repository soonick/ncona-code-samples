image:
	@docker build -t esp32-https .
.PHONY: image

build: image
	@docker run -it --rm \
		-v $(PWD)/minimal-https:/minimal-https \
		-w /minimal-https \
		esp32-https \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py build \
		"
.PHONY: build

ssh: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/minimal-https:/minimal-https \
		-w /minimal-https \
		esp32-https \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			bash \
		"
.PHONY: ssh

upload: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/minimal-https:/minimal-https \
		-w /minimal-https \
		esp32-https \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py flash \
		"
.PHONY: upload

serial: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/minimal-https:/minimal-https \
		-w /minimal-https \
		esp32-https \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py monitor \
		"
.PHONY: serial
