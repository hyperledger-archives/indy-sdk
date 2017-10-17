
## Before you Continue

If you haven't done so already, please visit the main resource for all things "Indy" to get acquainted with the code base, helpful resources, and up-to-date information: [Hyperledger Wiki-Indy](https://wiki.hyperledger.org/projects/indy).

# Indy SDK

This is the official SDK for [Hyperledger Indy](https://www.hyperledger.org/projects),
which provides a distributed-ledger-based foundation for [self-sovereign identity](https://sovrin.org).
The major artifact of the SDK is a c-callable
library; there are also convenience wrappers for various programming languages.

All bugs, stories, and backlog for this project are managed through [Hyperledger's Jira](https://jira.hyperledger.org)
in project IS (note that regular Indy tickets are in the INDY project instead...). Also, join
us on [Hyperledger's Rocket.Chat](https://chat.hyperledger.org/) at #indy-sdk to discuss.

## Building Indy SDK

* [Ubuntu based distro (Ubuntu 16.04)](doc/ubuntu-build.md)
* [RHEL based distro (Amazon Linux 2017.03)](doc/rhel-build.md)
* [Windows](doc/windows-build.md)
* [MacOS](doc/mac-build.md)

## Wrappers documentation
* [.Net](wrappers/dotnet/README.md)
* [Java](wrappers/java/README.md)
* [Python](wrappers/python/README.md)
* [iOS](wrappers/ios/ios-build.md)

## Binaries
Builded binaries can be downloaded from https://repo.evernym.com/libindy:
* ubuntu/{master,stable,rc} - Ubuntu deb packages
* windows/{master,stable,rc} - Windows zip-archive with all required DLLs (include libindy itself) and headers
* windows/deps/ - Windows zip archive with dependencies (DLLs and headers) to build libindy from sources
* ios/stable/ - Pods for iOS
* rhel/{master,stable,rc} - RHEL rpms
