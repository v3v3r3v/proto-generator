# Protobuffers specification for interservice communication

## Goals

Provide single instrument to describe contracts for microservices and affectively build for all languages

## What is included:
* Implementation-independent message description in .proto files
* Generation of docs for defined messages
* Сode generation of libraries for languages 
    - C#
    - PHP
    - Go
    - Rust

* Examples how to use generated libraries (WIP)
* Service to disover current version used in cluster (WIP)


---
## Setup [protocol buffers](https://github.com/protocolbuffers/protobuf#protocol-buffers---googles-data-interchange-format) on local machine

### Install [protocol buffers compiler](https://github.com/protocolbuffers/protobuf#protocol-compiler-installation)

*Preferred usage of latest version*

There are multiple options: 
- From [source](https://github.com/protocolbuffers/protobuf/tags)
    ```
    mkdir protoc_setup
    cd protoc_setup
    wget https://github.com/protocolbuffers/protobuf/releases/download/v3.19.0/protobuf-all-3.19.0.tar.gz
    tar -xf protobuf-all-3.19.0.tar.gz
    cd protobuf-all-3.19.0
    ./configure && make && install
    protoc --version
    ```
- Install using package manager of your OS or download and install precompiled binary. Instruction [here](https://grpc.io/docs/protoc-installation/).

---
### Setup for C#
protoc compile .proto for C# out of the box

---
### Setup for Go
- Install [Go](https://golang.org/)
- Install [protoc-gen-go plugin](https://github.com/protocolbuffers/protobuf-go):
    ```bash
    go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
    ```

---
### Setup for PHP
[Guide here](https://github.com/protocolbuffers/protobuf/tree/master/php)

---
### Setup for Rust (optional)
- Install [Rust](https://www.rust-lang.org/tools/install)

* Install [protoc-gen-rust](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen)

Protocol buffers for Rust is not officially supported by Google, but a community provided plugin is [available](https://github.com/stepancheg/rust-protobuf/).

```bash
cargo install protobuf-codegen
```

### Setup doc generation

[protoc-gen-doc](https://github.com/pseudomuto/protoc-gen-doc) plugin could be used for generate docs

For download and build tool (go required):
```
make protoc-gen-doc-install
```

## Usage

* Change protos/*.proto files
* Build sources for libraries

Make commands:

- Clean **lib/*/proto** folders
```bash
make clean
```

- Build for target {language}. Available options: csharp, php, go, rust
```bash
make build-{language}
```

- Build for csharp, php, go
```bash
make build-common
```

- Build for all available languages
```bash
make build-all
```

### Problems and solutions

* What if same .proto definition has been changed in two and more dev branches simeliounesly simultaneously

For example:

Branch main
```
message ExampleMessage {
    string name = 1;
    int32 id = 2;
    string email = 3;
    google.protobuf.Timestamp last_updated = 4;
}
```

Branch foo
```
message ExampleMessage {
    string name = 1;
    int32 id = 2;
    string email = 3;
    google.protobuf.Timestamp last_updated = 4;
+    string surname = 5;
+    google.protobuf.Timestamp date_of_birth = 6; 
}
```

Branch bar
```
message ExampleMessage {
    string name = 1;
    int32 id = 2;
    string email = 3;
    google.protobuf.Timestamp last_updated = 4;
+    enum Gender {
+        NONE = 0;
+        MALE = 1;
+        FEMALE = 2;
+    }
+    Gender gender = 5;
}
```

And then merge leads to conflict:
```
message ExampleMessage {
    string name = 1;
    int32 id = 2;
    string email = 3;
    google.protobuf.Timestamp last_updated = 4;
<<<<<< HEAD
    string surname = 5;
    google.protobuf.Timestamp date_of_birth = 6; 
======
    enum Gender {
        NONE = 0;
        MALE = 1;
        FEMALE = 2;
    }
    Gender gender = 5;
>>>>>> bar
}
```

Solution:
1. Resolve conflicts in .proto files and don't forget align [Field Numbers](https://developers.google.com/protocol-buffers/docs/proto3#assigning_field_numbers)

> Don't change position of field which is already in release!

2. Rebuild libraries

```
make build-all
```

> If multiple fields has same number make build-* fails.
```
protos/example/example.proto:28:21: Field number 5 has already been used in "Example.ExampleMessage" by field "surname".
```

3. Add changes to git index and commit
