image:
	@docker build -t esp32-ap .
.PHONY: image

build: image
	@docker run -it --rm \
		-v $(PWD)/project:/project \
		-w /project \
		esp32-ap \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py build \
		"
.PHONY: build

upload: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/project:/project \
		-w /project \
		esp32-ap \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py flash \
		"
.PHONY: upload

serial: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/project:/project \
		-w /project \
		esp32-ap \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py monitor \
		"
.PHONY: serial

ssh: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/project:/project \
		-w /project \
		esp32-ap \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			bash \
		"
.PHONY: ssh
