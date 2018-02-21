# Indy Getting Started

## Running getting-started with docker-compose

### Prerequisites

`docker` and `docker-compose` should be installed.

### Run

`docker-compose up`

The command above will create `getting-started` (the jupyter notebook) and `indy_pool` (collection of the validator nodes) images if they hasn't been done yet, create containers and run them.  
The validators run by default on IP `10.0.0.2`, this can be changed by changing `pool_ip` in the `docker-compose` file.  
To get Jupyter click on the link in output (it must have following format: http://0.0.0.0:8888/?token= )

### Stop

`docker-compose down`
The command above will stop and delete created network and containers.
