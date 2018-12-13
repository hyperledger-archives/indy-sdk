## Indy SDK for Go

This is a **work-in-progress** Go wrapper for [Indy](https://www.hyperledger.org/projects/indy). It is implemented using cgo to interact with native libindy library written in Rust.
Hyperledger Indy is the open-source codebase behind the Sovrin network for self-sovereign digital identity.

This Go wrapper is developed using golang 1.7+.

### How to install

You will need:

* C build tools
    * gcc, build-essentials
* `libindy` v1.6+ in your system library path. (i.e. `/usr/lib/libindy.so` for linux)

```sh
    go get github.com/hyperledger/indy-sdk/wrappers/golang/indysdk
```

### Testing

- You will need:
    * Dep [GoDep](https://golang.github.io/dep/) for dependency management

- Clone indy-sdk repo from https://github.com/hyperledger/indy-sdk

- Move to golang/indysdk directory
```sh
    cd wrappers/golang/indysdk
```

- Install dependencies
```sh
    dep ensure
```

- Run test:
```sh
    go test -v
```

### Usage

```go
package main
  
import (
  "fmt"

  "github.com/hyperledger/indy-sdk/wrappers/golang/indysdk"
)

func main() {
    config := indysdk.WalletConfig{
      ID: "alice",
    }
    credential := indysdk.WalletCredential{
      Key: "alice_key",
    }
    err := indysdk.CreateWallet(config, credential)
    if err != nil {
        fmt.Println("Error:", err)
    }
}
```

#### Troubleshooting
Use environment variable `RUST_LOG={info|debug|trace}` to output logs of Libindy.