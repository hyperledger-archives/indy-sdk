Indy Getting Started

## Running getting-started with docker-compose

### Prerequisites

`docker` and `docker-compose` should be installed.

### Run

`docker-compose up`

The command above will create `pool_network` network, `jupyter` and `pool` images if it hasn't been done yet, create containers and run them.
To get Jupyter click on the link in output (it must have following format: http://0.0.0.0:8888/?token= )

### Stop

`docker-compose down`
The command above will stop and delete created network and containers.
