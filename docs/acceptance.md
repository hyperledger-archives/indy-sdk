# Ubuntu acceptance testing:

1. check pool versions in ci/indy-pool.dockerfile
2. `docker build -f ci/indy-pool.dockerfile -t indy_pool .` [`--build-arg pool_ip=10.0.0.2`] (all after dot is for GST)
3. `docker run -itd -p 9701-9709:9701-9709 indy_pool`
4. `docker build --build-arg indy_sdk_deb='https://repo.sovrin.org/sdk/lib/apt/xenial/rc/libindy_1.10.1~79_amd64.deb' -f ci/acceptance/ubuntu_acceptance.dockerfile  .` <<< insert your version here
5. `docker run -it -v /home/indy/indy-sdk/samples:/home/indy/samples --network=host a8c51c554afb` <<< insert image id from previous step here

* Check Java wrapper:
1. `cd samples/java`
2. set version of `indy` dependency in `pom.xml` to libindy version
3. `TEST_POOL_IP=127.0.0.1 mvn clean compile exec:java -Dexec.mainClass="Main"`
4. check results
5. `rm -rf /home/indy/.indy_client`

* Check Python wrapper:
1. `cd samples/python`
2. set version of `python3-indy` dependency in `setup.py` to libindy version
3. `python3.5 -m pip install --user -e .`
4. `TEST_POOL_IP=127.0.0.1 python3.5 src/main.py`
5. check results
6. `rm -rf /home/indy/.indy_client`

------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

# Windows acceptance testing:

1. check pool versions in ci/indy-pool.dockerfile
2. `docker build -f ci/indy-pool.dockerfile -t indy_pool . --build-arg pool_ip=192.168.99.100`
3. `docker run -itd -p 9701-9709:9701-9709 indy_pool`
4. install jdk 8 and add path to environmental variable `JAVA_HOME`
5. install maven and add path to environmental variable `Path`
6. install python 3.5 + pip and add path to environmental variable `Path`
7. download libindy.zip, unpack it and add path to environmental variable `Path`

* Check Java wrapper:
1. `cd samples/java`
2. set version of `indy` dependency in `pom.xml`
3. `TEST_POOL_IP=192.168.99.100 mvn clean compile exec:java -Dexec.mainClass="Main"`
4. check results


* Check Python wrapper:
1. `cd samples/python`
2. set version of `python3-indy` dependency in `setup.py`
3. `python -m pip install -e .`
4. `TEST_POOL_IP=192.168.99.100 python src/main.py`
5. check results
