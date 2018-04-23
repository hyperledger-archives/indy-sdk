
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


## Installation
### Release channels
Indy SDK release process defines the following release channels:

* `master` - development builds for each push to master branch.
* `rc` - release candidates.
* `stable` - stable releases.

Please refer to [release workflow](doc/release-workflow.md) for more details.

### Ubuntu based distributions (Ubuntu 16.04)
It is recommended to install packages with APT:

    apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial {release channel}"
    sudo apt-get update
    sudo apt-get install -y libindy

{release channel} must be replaced with master, rc or stable to define corresponded release channel.
See section "Release channels" for more details.

### Windows

1. follow to https://repo.sovrin.org/windows/libindy/{release-channel}.
2. download last version of libindy.
3. unzip archives to directory, where you want to save working library.
4. After unzip you will get next structure of files:

* `Your working directory`
    * `include`
        * `...`
    * `lib`
        * `indy.dll`
        * `libeay32md.dll`
        * `libsodium.dll`
        * `libzmq.dll`
        * `ssleay32md.dll`

`include` contains c-header files which contains all necessary declarations
that may be need for your applications.

`lib` contains all necessary binaries which contains libindy and all it's dependencies.
 `You must add to PATH environment variable path to lib`. It's necessary for dynamic linkage
 your application with libindy.

{release channel} must be replaced with master, rc or stable to define corresponded release channel.
See section "Release channels" for more details.

### iOS
See [wrapper iOS install documentation](wrappers/ios/README.md "How to install").

### MacOS

Pre-built libraries are not provided for MacOS. Please look [here](doc/mac-build.md)
for details on building from source for MacOS.

After building `libindy`, add the path containing the library the `LD_LIBRARY_PATH` and
`DYLD_LIBRARY_PATH` environment variables. This is necessary for dynamically linking
your application with `libindy`. The dynamic linker will first check for the library in
`LD_LIBRARY_PATH` if the library in your application doesn't include directory names.
If the library in your application does include any directory name, then dynamic
linker will search for the library in `DYLD_LIBRARY_PATH` (not `LD_LIBRARY_PATH`)
so we recommend you set both variables to be safe.

### RHEL-based distributions (Amazon Linux 2017.03)
Pre-built libraries are not provided for RHEL-based distributions. Please look [here](doc/rhel-build.md)
for details on building from source for RHEL-based distributions.

After successfully compiling `libindy`, you will need to add the path containing `libindy.so` to the
`LD_LIBRARY_PATH` environment variable. This is required for your application to link to
`libindy`.

## How to build

* [Ubuntu based distributions (Ubuntu 16.04)](doc/ubuntu-build.md)
* [RHEL based distributions (Amazon Linux 2017.03)](doc/rhel-build.md)
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
 Note that for Windows and MacOS this approach has some issues. Docker for these OS run in
 their virtual environment. First command creates network for container and host can't
 get access to that network because container placed on virtual machine. You must appropriate set up 
 networking on your virtual environment. See the instructions for MacOS below.

### Docker port mapping on MacOS

If you use some Docker distribution based on Virtual Box you can use Virtual Box's
port forwarding future to map 9701-9709 container ports to local 9701-9709 ports.

If you use VMWare Fusion to run Docker locally, follow the instructions from
[this article](https://medium.com/@tuweizhong/how-to-setup-port-forward-at-vmware-fusion-8-for-os-x-742ad6ca1344)
and add the following lines to _/Library/Preferences/VMware Fusion/vmnet8/nat.conf_:

```
# Use these with care - anyone can enter into your VM through these...
# The format and example are as follows:
#<external port number> = <VM's IP address>:<VM's port number>
#8080 = 172.16.3.128:80
9701 = <your_docker_ip>:9701
9702 = <your_docker_ip>:9702
9703 = <your_docker_ip>:9703
9704 = <your_docker_ip>:9704
9705 = <your_docker_ip>:9705
9706 = <your_docker_ip>:9706
9707 = <your_docker_ip>:9707
9708 = <your_docker_ip>:9708
9709 = <your_docker_ip>:9709
```
where <your_docker_ip> is your Docker host IP.

Docker machine needs to be rebooted after these changes.

## Wrappers documentation

The following wrappers are tested and complete. There is also active work
on wrappers for Go and node.js; visit
[#indy-sdk on Rocket.Chat](https://chat.hyperledger.org/channel/indy-sdk) for
details.

* [.Net](wrappers/dotnet/README.md)
* [Java](wrappers/java/README.md)
* [Python](wrappers/python/README.md)
* [iOS](wrappers/ios/README.md)

## Indy CLI documentation
* [Indy CLI](cli/README.md)

## Getting started

This extended tutorial shows how the whole ecosystem works, and how
functions in the SDK can be used to construct rich clients. (You may also
want to look at the [older guide](https://github.com/hyperledger/indy-node/blob/stable/getting-started.md)
that explored the ecosystem via command line. That material is being
rewritten but still contains some useful ideas.)

* [Libindy Getting-Started Guide](doc/getting-started/getting-started.md)

## How Tos

Short, simple tutorials that demonstrate how to accomplish common tasks
are also available. See [the doc/how-tos folder](doc/how-tos).

## How to migrate
The documents that provide necessary information for Libindy migration.
* [v1.3.0 → v1.4.0](doc/migration-guide.md)
