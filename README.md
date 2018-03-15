
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

## How-to-install
### Instal for Ubuntu based distro (Ubuntu 16.04)
It is recommended to install packages with APT (change stable to `master` or `rc` if needed):
    
    apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable"
    sudo apt-get update
    sudo apt-get install -y libindy
    
### Instal for Windows
1. follow to https://repo.sovrin.org/windows/libindy.
2. download last version of libindy and indy-sdk-deps.
3. unzip archives to directories, where you want to save working libraries.
4. add to PATH enviroment variable path to that directories.

### Instal for MacOS        
Now we haven't prebuild library in some shared place. You can build
library yourself. Please refer to How-to-build section. 
After build add to PATH enviroment variable path to builded library.
            
### Instal for RHEL based distro (Amazon Linux 2017.03)           
Now we haven't prebuild library in some shared place. You can build
library yourself. Please refer to How-to-build section.
After build add to PATH enviroment variable path to builded library.

## How-to-build

* [Ubuntu based distro (Ubuntu 16.04)](doc/ubuntu-build.md)
* [RHEL based distro (Amazon Linux 2017.03)](doc/rhel-build.md)
* [Windows](doc/windows-build.md)
* [MacOS](doc/mac-build.md)

## How to start local nodes pool with docker

Start local nodes pool on `127.0.0.1:9701-9708` with Docker:
     
     ```     
     docker build -f ci/indy-pool.dockerfile -t indy_pool .
     docker run -itd -p 9701-9708:9701-9708 indy_pool
     ```     
     
 Ubuntu and Amazon Linux have another approach. Dockerfile `ci/indy-pool.dockerfile` 
 supports optional pool_ip param that allows changing ip of pool nodes in generated 
 pool configuration. The following commands allow to start local nodes pool in custom 
 docker network and access this pool by custom ip in docker network:
     
     ```
     docker network create --subnet 10.0.0.0/8 indy_pool_network
     docker build --build-arg pool_ip=10.0.0.2 -f ci/indy-pool.dockerfile -t indy_pool .
     docker run -d --ip="10.0.0.2" --net=indy_pool_network indy_pool
     ```          

## Wrappers documentation
* [.Net](wrappers/dotnet/README.md)
* [Java](wrappers/java/README.md)
* [Python](wrappers/python/README.md)
* [iOS](wrappers/ios/README.md)

## Indy CLI documentation
* [Indy CLI](cli/README.md)

## Getting started
* [Libindy Getting-Started Guide](doc/getting-started/getting-started.md)

## Binaries
Pre-Built binaries can be downloaded from https://repo.sovrin.org/:
* sdk/lib/apt/xenial/{master,stable,rc} - Ubuntu deb packages
* windows/libindy/{master,stable,rc} - Windows zip-archive with all required DLLs (include libindy itself) and headers
* windows/libindy/deps/ - Windows zip archive with dependencies (DLLs and headers) to build libindy from sources
* ios/libindy/stable/ - Pods for iOS
* rhel/libindy/{master,stable,rc} - RHEL rpms