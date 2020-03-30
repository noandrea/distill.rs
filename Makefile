GIT_DESCR = $(shell git describe --always) 
# build output folder
OUTPUTFOLDER = target
# docker image
DOCKER_REGISTRY = noandrea
DOCKER_IMAGE = distill
DOCKER_TAG = $(shell git describe --always)
# build parameters
OS = linux
ARCH = amd64

.PHONY: list
list:
	@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$' | xargs

default: build

workdir:
	mkdir -p dist

build: build-dist

build-dist: $(GOFILES)
	@echo build binary to $(OUTPUTFOLDER)
	cargo build
	@echo copy resources
	cp -r README.md LICENSE $(OUTPUTFOLDER)
	@echo done

test: test-all

test-all:
	@echo running tests 
	go test $(GOPACKAGES) -race -coverprofile=coverage.txt -covermode=atomic
	@echo tests completed

bench: bench-all

bench-all:
	@go test -bench -v $(GOPACKAGES)

lint: lint-all

lint-all:
	@golint -set_exit_status $(GOPACKAGES)

clean:
	@echo remove $(OUTPUTFOLDER) folder
	@rm -rf $(OUTPUTFOLDER)
	@echo done

docker: docker-build

docker-build:
	@echo build image
	docker build -t $(DOCKER_IMAGE) -f ./build/docker/Dockerfile .
	@echo done

docker-push: docker-build
	@echo push image
	docker tag $(DOCKER_IMAGE) $(DOCKER_REGISTRY)/$(DOCKER_IMAGE):$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/$(DOCKER_IMAGE):$(DOCKER_TAG)
	@echo done

docker-run: 
	@docker run -p 1804:1804 $(DOCKER_IMAGE) 

debug-start:
	@go run main.go -c examples/settings.yaml --debug start

gen-secret:
	@< /dev/urandom tr -dc _A-Z-a-z-0-9 | head -c40
