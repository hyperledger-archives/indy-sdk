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

On Ubuntu it is recommended to install packages with APT (change stable to `master` or `rc` if needed):
```
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88
sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb (xenial|bionic) stable"
sudo apt-get update
sudo apt-get install -y indy-cli
```

### Execution modes
CLI supports 2 execution modes:
* Interactive. In this mode CLI reads commands from terminal interactively. To start this mode just run `indy-cli`
without params.
* Batch. In this mode all commands will be read from text file or pipe and executed in series. To start this mode run
`indy-cli <path-to-text-file>`. Batch mode supports the same commands as interactive mode. Note that by default if some
command finishes with an error batch execution will be interrupted. To prevent this start command with `-`.
For example, `-wallet create test`. In this case the result of this command will be ignored. Comments can also be made
by beginning the line with a `#`.

### Getting help
The most simple way is just start cli by `indy-cli` command and put `help` command. Also you can look to
[Indy CLI Design](https://github.com/hyperledger/indy-sdk/tree/master/docs/design/001-cli) doc that contains the list of commands and architecture overview.

### Options
* -h and --help - Print usage.
* --logger-config - Init logger according to a config file (default no logger initialized).
* --plugins - Load plugins in Libindy (usage: <lib-1-name>:<init-func-1-name>,...,<lib-n-name>:<init-func-n-name>).
* --config - Define config file for CLI initialization. A config file can contain the following fields:
    * plugins - a list of plugins to load in Libindy (is equal to usage of "--plugins" option).
    * loggerConfig - path to a logger config file (is equal to usage of "--logger-config" option).
    * taaAcceptanceMechanism - transaction author agreement acceptance mechanism to be used when sending write transactions to the Ledger.

### Old python-based CLI migration
It is possible to import did's stored in the wallet of deprecated python-based CLI tool.
To achieve this user needs to perform the following steps:
1. Execute script on machine with installed old python-based CLI.
    ```
    indy_old_cli_export_dids [-e <env name>] -w <wallet name> [-f <path to the result file>]
    ```
    This script will export DIDs stored in specified wallet into the result file.
By default, this file creates in current folder and has the following name:
    ```
    <env name>_<wallet name>.exp_wallet
    ```
2. [Install Indy-Cli](#binaries)
3. Import generated file into libindy wallet by using Indy CLI
    * Run Indy CLI
    * Open new target wallet (create if needed) in CLI
    * Run
    ```
    did import <path to the file created on first step>
    ```


### Notes
Indy-CLI depends on `term` rust library that has a system dependency on terminfo database. 
That is why CLI Debian package additionally installs `libncursesw5-dev` library.
More about it read [here](https://crates.io/crates/term) at `Packaging and Distributing` section.



