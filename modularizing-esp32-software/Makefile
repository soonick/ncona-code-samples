image:
	@docker build -t modularizing-esp32-software .
.PHONY: image

build: image
	@docker run -it --rm \
		-v $(PWD)/project:/project \
		-w /project \
		modularizing-esp32-software \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py build \
		"
.PHONY: build

ssh: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/project:/project \
		-w /project \
		modularizing-esp32-software \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			bash \
		"
.PHONY: ssh

upload: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/project:/project \
		-w /project \
		modularizing-esp32-software \
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
		modularizing-esp32-software \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py monitor \
		"
.PHONY: serial
