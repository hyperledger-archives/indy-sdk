## Null Payment Plugin for Indy SDK

This is a plugin that can be used for development of applications that use Payments API of Indy SDK.

This plugin acts like a local payment ledger -- you can create payment addresses, mint funds on them, make payments from one address to another, pay fees for your transactions and manage sources and fees.

To use this plugin you should link it to the application after and the same way as Indy SDK library. After that, you should call ```nullpay_init()``` function to register the methods of plugin to be used by libindy. Then you can call methods of libindy Payments API using ```payment_method = "null"```.

### Binaries

Pre-Built binaries can be downloaded from https://repo.sovrin.org/:
* sdk/lib/apt/xenial/master - Ubuntu deb packages. Note that it depends on [libindy](../README.md) package
* windows/libnullpay/master - Windows zip-archive with executable file and all required DLLs

On Ubuntu it is recommended to install packages with APT (change stable to `master` or `rc` if needed):
```
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88
sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb (xenial|bionic) stable"
sudo apt-get update
sudo apt-get install -y libnullpay
```

### Logs
Null Payment plugin doesn't the ability to set or configure own logger. 
It inheritances Libindy logger implementation during plugin initialization.
