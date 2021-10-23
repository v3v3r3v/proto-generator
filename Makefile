PROTO_FILES=$(shell find protos -iname "*.proto")

build-common: build-csharp build-php build-go build-doc

build-all: build-common build-rust

build-csharp:
	rm -rf libs/csharp/pb/*
	protoc  --csharp_out=libs/csharp/pb --csharp_opt=base_namespace=Pb $(PROTO_FILES)

build-php:
	rm -rf libs/php/pb/*
	protoc --php_out=libs/php/pb $(PROTO_FILES)

build-go:
	rm -rf libs/go/pb/*
	protoc --go_out=libs/go --go_opt=module=github.com/zverevvaleriy/proto-generator/libs/go $(PROTO_FILES)

build-rust:
	rm -rf libs/rust/pb/*
	protoc --rust_out=libs/rust/pb $(PROTO_FILES)


protoc-gen-doc-install:
	git clone https://github.com/pseudomuto/protoc-gen-doc protoc-gen-doc-repo
	cd protoc-gen-doc-repo/cmd/protoc-gen-doc && go build && mv protoc-gen-doc ../../../protoc-gen-doc
	rm -rf ./protoc-gen-doc-repo

build-doc:
	protoc --plugin=protoc-gen-doc=./protoc-gen-doc --doc_out=doc --doc_opt=html,index.html $(PROTO_FILES)
	protoc --plugin=protoc-gen-doc=./protoc-gen-doc --doc_out=doc --doc_opt=markdown,docs.md $(PROTO_FILES)
	
clean:
	rm -rf libs/*/pb/*