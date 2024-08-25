image:
	@docker build -t aligned-and-packed-data .
.PHONY: image

run: image
	@docker run -it --rm \
		-v $(PWD)/src:/src \
		-w /src \
		aligned-and-packed-data \
		sh -c "g++ main.cpp && ./a.out"
.PHONY: run
