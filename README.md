
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

## How to install
### Install for Ubuntu based distro (Ubuntu 16.04)
It is recommended to install packages with APT:
    
    apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial {release channel}"
    sudo apt-get update
    sudo apt-get install -y libindy
#### Release channels
{release channels} pointed in previous section may be one of these values: 
    
* `master` - development builds for each push to master branch.
* `rc` - release candidates.
* `stable` - stable releases.

Please refer to [release workflow](doc/release-workflow.md) for more details.  
   
### Install for Windows

1. follow to https://repo.sovrin.org/windows/libindy/{release-channel}.
2. download last version of libindy.
3. unzip archives to directory, where you want to save working library.
4. After unzip you will get next structure of files:

    
    -Your working directory
        -include
            ...
        -lib
            -indy.dll
            -libeay32md.dll
            -libsodium.dll
            -libzmq.dll
            -ssleay32md.dll
            
`include` contains c-header files which contains all necessary declarations
that may be need for your applications. 

`lib` contains all necessary binaries which contains libindy and all it's dependencies.
 `You must add to PATH environment variable path to lib`. It's necessary for dynamic linkage
 your application with libindy.       

#### Release channels
{release channels} pointed in previous section may be one of these values: 
    
* `master` - development builds for each push to master branch.
* `rc` - release candidates.
* `stable` - stable releases.

Please refer to [release workflow](doc/release-workflow.md) for more details.

### Install for MacOS        
Now we haven't prebuild library in some shared place. You can build
library yourself. Please refer to How-to-build section. 

After build add to LD_LIBRARY_PATH and to DYLD_LIBRARY_PATH 
environment variables path to builded library. It's necessary 
for dynamic linkage your application with libindy. At first dynamic linker
browse library in LD_LIBRARY_PATH, if library in your application doesn't include directory names.
If library in your application include any directory name, then dynamic linker will search library
in DYLD_LIBRARY_PATH(not LD_LIBRARY_PATH). So for reliability we recommend you set both this variables.
            
### Install for RHEL based distro (Amazon Linux 2017.03)           
Now we haven't prebuild library in some shared place. You can build
library yourself. Please refer to How-to-build section.

After build add to LD_LIBRARY_PATH environment variable path to builded library. 
It's necessary for dynamic linkage your application with libindy.

## How to build

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
     
 Dockerfile `ci/indy-pool.dockerfile` supports optional pool_ip param that allows 
 changing ip of pool nodes in generated pool configuration. The following commands 
 allow to start local nodes pool in custom docker network and access this pool 
 by custom ip in docker network:
     
     ```
     docker network create --subnet 10.0.0.0/8 indy_pool_network
     docker build --build-arg pool_ip=10.0.0.2 -f ci/indy-pool.dockerfile -t indy_pool .
     docker run -d --ip="10.0.0.2" --net=indy_pool_network indy_pool
     ``` 
 Note that for Windows and MacOS this approach have some issues. Docker for these OS run in
 their virtual environment. First command creates network for container and host can't
 get access to that network because container placed on virtual machine. You must appropriate set up 
 networking on your virtual environment.

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