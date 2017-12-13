## CLI for Indy-SDK

This is the official command line interface for Indy SDK, which provides a distributed-ledger-based
foundation for self-sovereign identity.

##### First, install [Indy-SDK](../README.md)

### Binaries
Pre-Built binaries can be downloaded from https://repo.sovrin.org/:
* lib/apt/xenial/{master,stable,rc} - Ubuntu deb packages
* windows/indy-cli/{master,stable,rc} - Windows zip-archive with execute file and all required DLLs 

Also Ubundu deb packages can be installed from APT repository (change stable to `master` or `rc` if needed):
```
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable"
sudo apt-get update
sudo apt-get install -y indy-cli
```