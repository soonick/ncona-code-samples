image:
	@docker build -t esp-32-project-image .
.PHONY: image

build: image
	@docker run -it --rm \
		-v $(PWD)/project:/project \
		-w /project/ \
		esp-32-project-image \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py build \
		"
.PHONY: build

ssh: image
	@docker run -it --rm \
		-v $(PWD)/project:/project \
		-w /project/ \
		esp-32-project-image \
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
		-w /project/ \
		esp-32-project-image \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py -p /dev/ttyUSB0 flash monitor \
		"
.PHONY: upload

menuconfig: image
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/project:/project \
		-w /project/ \
		esp-32-project-image \
		sh -c " \
			. /esp-idf/export.sh && \
			idf.py set-target esp32 && \
			idf.py menuconfig \
		"
.PHONY: menuconfig
