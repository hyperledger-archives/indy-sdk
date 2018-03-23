
## Before you Continue

If you haven't done so already, please visit the main resource for all things "Indy" to get acquainted with the code base, helpful resources, and up-to-date information: [Hyperledger Wiki-Indy](https://wiki.hyperledger.org/projects/indy).

# Indy SDK

This is the official SDK for [Hyperledger Indy](https://www.hyperledger.org/projects),
which provides a distributed-ledger-based foundation for [self-sovereign identity](https://sovrin.org).
The major artifact of the SDK is a c-callable
library; there are also convenience wrappers for various programming languages and Indy CLI tool.

All bugs, stories, and backlog for this project are managed through [Hyperledger's Jira](https://jira.hyperledger.org)
in project IS (note that regular Indy tickets are in the INDY project instead...). Also, join
us on [Hyperledger's Rocket.Chat](https://chat.hyperledger.org/) at #indy-sdk to discuss.

## Building Indy SDK

* [Ubuntu based distro (Ubuntu 16.04)](doc/ubuntu-build.md)
* [RHEL based distro (Amazon Linux 2017.03)](doc/rhel-build.md)
* [Windows](doc/windows-build.md)
* [MacOS](doc/mac-build.md)

## Wrappers documentation

The following wrappers are tested and complete. There is also active work
on wrappers for Go and node.js; visit #indy-sdk on Rocket.Chat for
details.

* [.Net](wrappers/dotnet/README.md) (lacks a couple recent features)
* [Java](wrappers/java/README.md)
* [Python](wrappers/python/README.md)
* [iOS](wrappers/ios/ios-build.md)

## Indy CLI documentation
* [Indy CLI](cli/README.md)

## Getting Started
This extended tutorial shows how the whole ecosystem works, and how
functions in the SDK can be used to construct rich clients. (You may also
want to look at the [older guide](https://github.com/hyperledger/indy-node/blob/stable/getting-started.md)
that explored the ecosystem via command line. That material is being
rewritten but still contains some useful ideas.)
* [Libindy Getting-Started Guide](doc/getting-started/getting-started.md)

## Binaries
Pre-Built binaries can be downloaded from https://repo.sovrin.org/:
* sdk/lib/apt/xenial/{master,stable,rc} - Ubuntu deb packages
* windows/libindy/{master,stable,rc} - Windows zip-archive with all required DLLs (include libindy itself) and headers
* windows/libindy/deps/ - Windows zip archive with dependencies (DLLs and headers) to build libindy from sources
* ios/libindy/stable/ - Pods for iOS
* rhel/libindy/{master,stable,rc} - RHEL rpms

On Ubundu it is recommended to install packages with APT (change stable to `master` or `rc` if needed):
```
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable"
sudo apt-get update
sudo apt-get install -y libindy
```
