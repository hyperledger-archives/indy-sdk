Steps to perform acceptance testing on Ubuntu:
* Run pool (see docker network option in [ubuntu readme](doc/ubuntu-build.md))
* build docker image `docker build -f ubuntu_acceptance.dockerfile --build-arg indy_sdk_deb="URL to download appropriate version of libindy.deb" .`
* start docker container from images built on previous step `docker run -it -v <path/to/indy-sdk/samples>:/home/indy --network=<pool network name> <Image ID> /bin/bash`
* in docker container:
    * Check Java wrapper:
        * `cd java`
        * set version of `indy` dependency in `pom.xml`
        * `TEST_POOL_IP=<pool ip> mvn clean compile exec:java -Dexec.mainClass="Main"`
        * check results
        * `cd ..`
    * Check Python wrapper:
        * `cd python`
        * set version of `python3-indy` dependency in `setup.py`
        * `python3.6 -m pip install -e .`
        * `TEST_POOL_IP=<pool ip> python3.6 src/main.py`