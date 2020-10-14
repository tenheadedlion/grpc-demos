## 设计

两模块 A，B 之间通信，其中 A 用 cpp, B 用 go

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

### 下载 grpc-go

git clone https://github.com/grpc/grpc-go

将以下设置加到 .zshrc 或 .bashrc 设置 go 代理

```bash
export GO111MODULE=on
export GOPROXY=https://goproxy.cn
```

将依赖下载至本地

```bash
go get github.com/golang/protobuf/protoc-gen-go \
         google.golang.org/grpc/cmd/protoc-gen-go-grpc
```









