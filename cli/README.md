## CLI for Indy-SDK

This is the official command line interface for Indy SDK, which provides a distributed-ledger-based
foundation for self-sovereign identity. It provides the commands to:
* Manage wallets
* Manage pool configurations
* Manage DIDs
* Sending transactions to distributed ledger

### Installing the Indy-CLI

#### Release channels
The Indy SDK release process defines the following release channels:

* `master` - development builds for each push to master branch.
* `rc` - release candidates.
* `stable` - stable releases.

Please refer to our [release workflow](../docs/contributors/release-workflow.md) for more details.

#### Ubuntu based distributions (Ubuntu 16.04 and 18.04)
It is recommended to install the Indy-CLI with APT:

    sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb (xenial|bionic) {release channel}"
    sudo apt-get update
    sudo apt-get install -y indy-cli
    indy-cli

* (xenial|bionic) xenial for 16.04 Ubuntu and bionic for 18.04 Ubuntu.
* {release channel} must be replaced with master, rc or stable to define corresponded release channel.

#### Windows
1. Go to https://repo.sovrin.org/windows/indy-cli/{release-channel}.
2. Download last version of indy-cli.
3. Unzip archives to the directory where you want to save working library.
4. After unzip you will get next structure of files:

* `Your working directory`
    * `indy-cli.exe`
    * `indy.dll`
    * `libeay32md.dll`
    * `libsodium.dll`
    * `libzmq.dll`
    * `ssleay32md.dll`

5. Add path to the directory to PATH environment variable.
6. Run `indy-cli.exe` to start Indy-CLI.
 
#### MacOS
1. Go to https://repo.sovrin.org/macos/indy-cli/{release-channel}.
2. Download last version of indy-cli.
3. Unzip archives to the directory where you want to save working library.
4. After unzip you will get next structure of files:
    * `Your working directory`
        * `indy-cli` executable file
5. Install Libindy
   1. Download and unzip libindy from https://repo.sovrin.org/macos/libindy/{release-channel}.
   2. After unzip you will get `lib` folder which contains libindy binary.
   3. Either add directory path to `LIBRARY_PATH` env variable or move `libindy.dylib` to `/usr/lib` folder.
6. Run `indy-cli` to start Indy-CLI.

#### Centos
1. Go to https://repo.sovrin.org/rpm/indy-cli/{release-channel}.
2. Download and unzip the last version of library.
3. Install with `rpm -i indy-cli-version.rpm`.
4. Run `indy-cli` to start Indy-CLI.

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



