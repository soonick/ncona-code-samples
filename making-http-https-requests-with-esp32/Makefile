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

upload: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/minimal-https:/minimal-https \
		-v $(PWD)/production-https:/production-https \
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

build-prod: image
	@docker run -it --rm \
		-v $(PWD)/production-https:/production-https \
		-w /production-https \
		esp32-https \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py build \
		"
.PHONY: build-prod

upload-prod: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/production-https:/production-https \
		-w /production-https \
		esp32-https \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py flash \
		"
.PHONY: upload-prod

serial-prod: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/production-https:/production-https \
		-w /production-https \
		esp32-https \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py monitor \
		"
.PHONY: serial-prod

ssh: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/minimal-https:/minimal-https \
		-v $(PWD)/production-https:/production-https \
		-w /minimal-https \
		esp32-https \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			bash \
		"
.PHONY: ssh
