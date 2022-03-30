
### Run local pool nodes with Docker

```
docker build -f ci/indy-pool.dockerfile -t indy_pool ci
docker run -it -p 9701-9708:9701-9708 indy_pool
```

### Run the Java Samples

```
mvn exec:java -Dexec.mainClass=Main
```

or, if something goes wrong, like this ...

```
mvn exec:java -Dexec.mainClass=Main \
  -Dorg.slf4j.simpleLogger.defaultLogLevel=debug \
  -Djna.library.path=$LD_LIBRARY_PATH \
  -Djna.debug_load=true
```

### Run the GettingStarted Workflow

```
mvn exec:java -Dexec.mainClass=GettingStarted \
  -Djna.library.path=$LD_LIBRARY_PATH
```


### IntelliJ

* Open the project in IntelliJ
* [Optional] Change the version number of indy to latest in pom.xml like:

```
<dependency>
    <groupId>org.hyperledger</groupId>
    <artifactId>indy</artifactId>
    <version>1.16.0</version>
</dependency>
```

* Click on Edit configuration and edit Main.config, add `DYLD_LIBRARY_PATH=<path to indy dll/so/dylib>`.
* Note, `DYLD_LIBRARY_PATH` is specific to OSX, you have to use flag `LD_LIBRARY_PATH` if you are on Linux.
* If there is an error that config already exist. Remove `~/.indy_client` folder.
