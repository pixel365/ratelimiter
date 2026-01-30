VERSION ?= 0.0.1
IMAGE_REPOSITORY ?= pixel365
IMAGE_NAME ?= ratelimiter
DOCKERFILE ?= ./Dockerfile
CI_BUILD_ARGS :=

.PHONY: build push

build:
	docker $@ $(CI_BUILD_ARGS) -f ${DOCKERFILE} -t ${IMAGE_REPOSITORY}/${IMAGE_NAME}:${VERSION} .

push: build
	docker $@ ${IMAGE_REPOSITORY}/${IMAGE_NAME}:${VERSION}
