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
git clone --recurse-submodules -b v1.32.0 https://github.com/grpc/grpc
cd grpc
mkdir -p cmake/build
pushd cmake/build
cmake -DgRPC_INSTALL=ON -DgRPC_BUILD_TESTS=OFF -DgRPC_PROTOBUF_PROVIDER=package -DgRPC_ZLIB_PROVIDER=package -DgRPC_CARES_PROVIDER=package -DgRPC_SSL_PROVIDER=package -DCMAKE_BUILD_TYPE=Release ../..
make -j
make install
popd
```

### 编译模块 A

模块 A 代码 examples/cpp/helloworld 直接拷自 grpc/examples/cpp/helloworld

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
go: downloading google.golang.org/grpc v0.0.0-20201014215113-7b167fd6eca1
go: downloading google.golang.org/protobuf v1.25.0
go: downloading google.golang.org/genproto v0.0.0-20200806141610-86f49bd18e98
go: downloading golang.org/x/net v0.0.0-20200707034311-ab3426394381
go: downloading golang.org/x/sys v0.0.0-20200803210538-64077c9b5642
go: downloading golang.org/x/text v0.3.3
2020/10/15 12:53:43 Received: world
```

## .proto 文件

examples/cpp/protos/helloworld.proto 和 examples/go_greeter/internal/helloworld/helloworld.proto 一样, 可以用 git 管理

