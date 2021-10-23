PROTO_FILES=$(shell find protos -iname "*.proto")

build-common: build-csharp build-php build-go

build-all: build-common build-rust

build-csharp:
	rm -rf libs/csharp/proto/*
	protoc  --csharp_out=libs/csharp/proto --csharp_opt=base_namespace=Proto $(PROTO_FILES)

build-php:
	rm -rf libs/php/proto/*
	protoc --php_out=libs/php/proto $(PROTO_FILES)

build-go:
	rm -rf libs/go/proto/*
	protoc --go_out=libs/go --go_opt=module=github.com/zverevvaleriy/proto-generator/libs/go $(PROTO_FILES)

build-rust:
	rm -rf libs/rust/proto/*
	protoc --rust_out=libs/rust/proto $(PROTO_FILES)
	
clean:
	rm -rf libs/*/proto/*