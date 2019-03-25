# Indy SDK test approach 

### Components

Indy SDK contains the following parts:
* libindy - native library that provides high-level API for development of Sovrin-based applications.
It provides methods for handling communication with Indy pool, secure wallet, agents communication,
sign/verify, encrypt/decrypt, and anoncreds protocol
* Python Wrapper - FFI based wrapper for native libindy that allows application development with python language
* Java Wrapper - FFI based wrapper for native libindy that allows application development on Java platform
* iOS Wrapper - Objective-C based wrapper for native libindy that allows application development on Java platform
* .Net Wrapper - FFI based wrapper for native libindy that allows application development on .Net platform

### Acceptance testing

The Acceptance test procedure consists of the following parts:
* Functional testing
* Installability testing
* Interoperability testing
* Documentation testing

### Functional testing

Functional part of test procedure contains the set of automated system/integration tests that CI/CD execute for each
merge commit to master or rc branch. Release artifacts will be created from the same libindy binaries that were
used for tests execution. Creation of rc package assumes that automated functional tests are passed. 
See [cd-pipeline.puml](cd-pipeline.puml) for details.
 
Note that libindy and wrappers also provide the set of unit tests, but they are mostly used to follow TDD
approach and don't follow formal test design.

In the future we expect extending functional test procedure with some manual steps for complex cases that 
will be performed after rc package is created. Mostly it is relevant for Low cases. 

##### Functional specification

Specification to API calls for now are present as comments in interface parts of source code. See:
* https://github.com/hyperledger/indy-sdk/tree/master/libindy/include/ (libindy docs)
* https://github.com/hyperledger/indy-sdk/tree/master/wrappers/python/indy/ (python wrapper docs)
* https://github.com/hyperledger/indy-sdk/tree/master/wrappers/java/src/main/java/org/hyperledger/indy/sdk/
(java wrapper docs)
* https://github.com/hyperledger/indy-sdk/blob/master/wrappers/dotnet/indy-sdk-dotnet/Wrapper/ (.Net wrapper docs)
 
##### Test groups

We define the following test groups by priority:
* High cases
* Medium cases
* Low cases

##### High cases

Successful completion of High cases tests indicates Alpha quality.

* Normal cases. Note that there can be multiple execution branches. We need to cover at each branch.
 Branches examples:
  * Entity cached in the wallet
  * Entity should be taken from the ledger
* Error cases that require an explicit recovering procedure. Examples:
  * Invalid wallet credentials
  * No entity found in the wallet
  * No entity found in the ledger
  * Transaction doesn't allow for current identity
  * Unknown crypto
  * Claim doesn't correspond to scheme, proof request doesn't correspond to claim and etc...
  * Revocation registry is full and etc...

##### Medium cases

Successful completion of High and Medium cases tests indicates Beta quality.

* Precondition checking:
  * Invalid handle
  * Wallet doesn't correspond to pool
  * Invalid json format
  * Invalid json structure (missed fields and etc...)
  * Invalid base58
  * Invalid crypto keys length and format
  * Invalid crypto primitives (bigints, points)
  * Invalid complex crypto structures (anoncreds structures mostly)
  * Invalid responses from 3d parties (Ledger, Agent)

##### Low cases

Successful completion of High, Medium and Low cases tests indicates Production quality.

* Cases that hard to test: Io errors, timeouts and etc...

##### Tests specification

Tests specification is provided as the list of test cases for each API call in code grouped by
API group, API call, test level. Also there are dedicated "demo" tests mostly intended to provide
usage examples.

For current moment we implemented High and Medium cases for libindy (except revocation part of Anoncreds). High
cases for Python, Java, iOs, and .Net wrappers.

Note that High cases for wrappers contain the same tests as libindy and we keep this tests cases synced.
Architecture of wrapper allows to claim that High cases coverage can be enough for Beta+ quality.

Note that test procedure was created by developers (not professional QA) and can require review and 
enhancements.

See:
* https://github.com/hyperledger/indy-sdk/tree/master/libindy/tests (libindy tests)
* https://github.com/hyperledger/indy-sdk/tree/master/wrappers/python/tests (python wrapper tests)
* https://github.com/hyperledger/indy-sdk/tree/master/wrappers/java/src/test/java/org/hyperledger/indy/sdk (java wrapper tests)
* https://github.com/hyperledger/indy-sdk/tree/master/wrappers/ios/libindy-pod/libindy-demoTests (iOS wrapper tests)

### Installability testing

Functional testing is performed before artifacts packaging. libindy has some runtime dependencies
and we need to be sure that package installation satisfy these dependencies.

I suggest the following:
* Create simple demo projects on C, Python and Java that will depend on libindy artifacts and move our demo tests
to these projects (in the future we need projects for iOS, .Net and NodeJS). We can try to do this in the current
sprint.
* Test libindy and wrapper installation on ubuntu and windows (in the future on macos, rhel, iOS too)
* Test that these demo projects can be built and run with rc packages

For first release these steps can be performed manually and automated in the future.

#### Ubuntu testing
* Run pool (see docker network option in [ubuntu readme](/docs/build-guides/ubuntu-build.md))
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

### Interoperability testing

The following interoperability cases are needed:

* libindy - Node
  * Interoperability with latest Node version. We test it already with functional tests.
  * Backward compatibility of Node will be tested as part of Indy Node acceptance. (See
    [these notes](release-workflow.md#compatibility-with-indy-node) for a discussion about
    how compatibility relates to branches of indy-node and indy-sdk.)
* libindy - pyindy:
  * Anoncreds protocol interoperability. It is already implemented as part of functional tests.
* libindy - libindy
  * Persistent configuration backward compatibility for Major version (Requires test development, IS-312).
  * Persistent wallet backward compatibility for Major version (Requires test development, IS-312).
  * Persistent pool cache backward compatibility for Major version (Requires test development, IS-312).
  * Anoncreds protocol backward compatibility for Major version (Requires test development, IS-312).
  * Agent communication backward compatibility for Major version (Requires test development, IS-312).
* libindy - wrappers
  * For first releases we plan to release wrappers as same package and claim only exact version interoperability.
  Current functional test procedure performs this interoperability checking with wrappers functional tests.
 
For first release we can move with existing functional tests, but future release will require creation of
dedicated interoperability tests. These tests can be automated.

### Documentation testing

* Verify Changelog
* Verify documentation update for all claimed changes 
