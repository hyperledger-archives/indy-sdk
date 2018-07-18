# TROUBLE SHOOTING Getting - Started Guide (GSG)

If you setup the demo and encounter a 307 error recommend to take the following steps to cleanup and start over. The communication with the ledger is affected and it is not possible to run the demo. Here are some recommendations for other errors.
* 306: you already have a configured ledger. Perform clean start.
* 301: you are trying to create a ledger but it is already configured. A single failure will cause a problem when opening the ledger. Perform clean install.
* 212: wallet is not found. When this occur stop and start container: Ctrl-C, docker-compose down, docker-compose up

## Overview steps  for clean start

1. Remove existing instances
1. Reset source files
1. Perform a new build
1. Start demo

## Steps in detail

1. Make sure containers are closed

```
docker-compose down   # to make sure containers are closed
docker image ls       # find image names that need to be removed in next step
docker image rm getting-started
docker image rm indy_pool
docker volume ls      # find volume name that needs to be removed in next step
docker volume rm gettingstarted_sandbox
```

2. Reset source files

```git tag      # choose the latest version and use it as <branch_name>
git checkout <branch_name>
git reset --hard     **WARNING** : make copies of any changes you want to keep prior to taking this step
git fetch --all
git pull
```

3. Perform a new build

```
docker-compose build --no-cache    # adding no cache to make clean build
```

4. Start demo

```
docker-compose up
```
