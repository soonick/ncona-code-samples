image:
	@docker build -t svelte-internationalization .
.PHONY: image

start: image
	@docker run -it --rm \
		-p 5173:5173\
		-v $(PWD)/my-app:/my-app \
		-w /my-app \
		svelte-internationalization
.PHONY: start
