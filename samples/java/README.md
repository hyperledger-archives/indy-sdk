
How to run:

__Intellij__

* Open the project in IntelliJ
* [Optional] Change the version number of indy to latest in pom.xml like:
    ```
    <dependency>
       			<groupId>org.hyperledger</groupId>
       			<artifactId>indy</artifactId>
       			<version>1.0.0</version>
     </dependency>
     ```

* Click on Edit configuration and edit Main.config, add DYLD_LIBRARY_PATH=<path to indy dll/so/dylib>. *Note: `DYLD_LIBRARY_PATH` is specific to OSX, you have to use flag `LD_LIBRARY_PATH` if you are on Linux*
* Run the Project
* If there is an error that config already exist. Remove `~/.indy_client` folder.

__Shell__

```mvn exec:java -Dexec.mainClass=Main```
