## 设计

两模块 A，B 之间通信，其中 A 用 C++, B 用 Go

## 准备

### 安装最新的 protoc, protocol buffer

https://grpc.io/docs/protoc-installation/
https://github.com/protocolbuffers/protobuf/releases/tag/v3.13.0

同时也需要更新 cmake 到最新
https://github.com/Kitware/CMake/releases/download/v3.18.4/cmake-3.18.4.tar.gz

### 编译 grpc

https://www.grpc.io/docs/languages/cpp/quickstart/

```bash
sudo apt install -y cmake
sudo apt install -y build-essential autoconf libtool pkg-config
git clone --recurse-submodules -b v1.45.0 --depth 1 --shallow-submodules https://github.com/grpc/grpc
cd grpc
mkdir -p cmake/build
pushd cmake/build
cmake -DgRPC_INSTALL=ON -DgRPC_BUILD_TESTS=OFF -DgRPC_PROTOBUF_PROVIDER=package -DgRPC_ZLIB_PROVIDER=package -DgRPC_CARES_PROVIDER=package -DgRPC_SSL_PROVIDER=package -DCMAKE_BUILD_TYPE=Release ../..
make -j
make install
popd
```

### 编译模块 A

模块 A 代码 examples/cpp/cpp/helloworld 直接拷自 grpc/examples/cpp/helloworld

```bash
mkdir -p cmake/build
pushd cmake/build
cmake ../..
make -j
```

### 运行 Go


```bash

pushd examples/go_greeter/internal
protoc --go_out=. --go_opt=paths=source_relative \
    --go-grpc_out=. --go-grpc_opt=paths=source_relative \
    helloworld/helloworld.proto
popd
pushd example/go_greeter
go run cmd/helloworld/greeter_server/main.go
```

## 测试

模块 A 作为客户端

```
➜  build git:(master) ./greeter_client
Greeter received: Hello world
```

模块 B 收到消息：

```
➜  go_greeter git:(master) ✗ go run cmd/helloworld/greeter_server/main.go 
2020/10/15 12:53:43 Received: world
```

## .proto 文件

examples/cpp/protos/helloworld.proto 和 examples/go_greeter/internal/helloworld/helloworld.proto 一样, 可以用 git 管理

## Rust Prost

https://github.com/tokio-rs/prost

创建 rust-with-prost 目录，进入目录，创建新项目：

```bash
cargo init rust-hello
```

在 Cargo.toml 加入

```toml
[dependencies]
prost = "0.10"
# Only necessary if using Protobuf well-known types:
prost-types = "0.10"

[build-dependencies]
prost-build = { version = "0.10" }
```

在 src 目录创建 hello.proto， 写入：

```proto
syntax = "proto3";

package hello;

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply) {}
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
}
```

在根目录创建 build.rs，写入：

```rust
use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/hello.proto"], &["src/"])?;
    Ok(())
}
```

在 src/lib.rs 写入：

```rust
pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

fn main() {

}
```

运行 `cargo build`，找到 hello.rs

```bash
➜  rust-hello git:(master) ✗ find . -name "hello.rs"
./target/debug/build/rust-hello-7975d43850b92382/out/hello.rs
```

内容如下：

```rust
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
```

然后修改 src/main.rs:

```rust
pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

pub fn create_hello_req(name: String) -> hello::HelloRequest {
    hello::HelloRequest {
        name: name
    }
}

fn main() {
    let req = create_hello_req("you".to_string());
    dbg!(req);
}
```

结果：

```
[src/main.rs:13] req = HelloRequest {
    name: "you",
}
```

没什么用，只是将 hello.proto 翻译成 hello.rs, 没有 server 和 client 代码

## Tonic

按照 https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md 生成项目

tonic 集成了 prost, tokio 等组件

编译

```bash
cargo build
```

运行客户端：

```bash
cargo run --bin helloworld-client
```

结果：

```
➜  helloworld-tonic git:(master) ✗ cargo run --bin helloworld-client
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/helloworld-client`
RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "grpc-status": "0", "grpc-message": ""} }, message: HelloReply { message: "Hello Tonic" }, extensions: Extensions }
````

此时 go 服务端收到：

```
➜  go_greeter git:(master) ✗ go run cmd/helloworld/greeter_server/main.go 
2022/03/29 10:29:14 Received: Tonic
```

## 给 go 服务端设置一个 report timer

10 秒报一次收到 hello 的次数, 修改 examples/go_greeter/cmd/helloworld/greeter_server/main.go:61

```go
	go func() {
		for {
			select {
			case <-ticker.C:
				log.Printf("[report] received %d requests in last 10 sec.", reqcnt)
				reqcnt = 0
			case <-quit:
				ticker.Stop()
				return
			}
		}
	}()
```

运行客户端：

```bash
for i in `seq 1 1000`; do cargo run --bin helloworld-client ; done
```

服务端：

```
➜  go_greeter git:(master) ✗ go run cmd/helloworld/greeter_server/main.go

2022/03/29 10:48:15 [report] received 31 requests in last 10 sec.
2022/03/29 10:48:25 [report] received 84 requests in last 10 sec.
2022/03/29 10:48:35 [report] received 84 requests in last 10 sec.
```
