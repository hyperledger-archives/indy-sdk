## CLI for Indy-SDK

This is the official command line interface for Indy SDK, which provides a distributed-ledger-based
foundation for self-sovereign identity. It provides the commands to:
* Manage wallets
* Manage pool configurations
* Manage DIDs
* Sending transactions to distributed ledger

### Binaries
Pre-Built binaries can be downloaded from https://repo.sovrin.org/:
* sdk/lib/apt/xenial/{master,stable,rc} - Ubuntu deb packages. Note that it depends on [libindy](../README.md) package
* windows/indy-cli/{master,stable,rc} - Windows zip-archive with executable file and all required DLLs 

On Ubundu it is recommended to install packages with APT (change stable to `master` or `rc` if needed):
```
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable"
sudo apt-get update
sudo apt-get install -y indy-cli
```

### Execution modes
CLI supports 2 execution modes:
* Interactive. In this mode CLI reads commands from terminal interactively. To start this mode just run `indy-cli` without params.
* Batch. In this mode all commands will be read from text file or pipe and executed in series. To start this mode run `indy-cli <path-to-text-file>`. Batch mode supports the same commands as interactive mode. Note that by default if some command finishes with the error batch execution will be interrupted. To prevent this start command with `-`. For example, `-wallet create test`. In this case the result of this command will be ignored.

### Getting help
The most simple way is just start cli by `indy-cli` command and put `help` command. Also you can look to [Indy CLI Design](../doc/cli-design.md) doc that contains the list of commands and architecture overview.