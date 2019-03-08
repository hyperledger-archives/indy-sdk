## Indy SDK for Java

This Java wrapper currently requires Java 8 (e.g. the openjdk-8-jdk package in Debian/Ubuntu).

Pull requests welcome!

**Not ready for production use! Not all commands work properly! Use at your own risk!**
### How to install
In your maven project add to pom.xml file next content:

1. Inside repositories tag block add:
    
    
    <repository>
        <id>sovrin</id>
        <url>https://repo.sovrin.org/repository/maven-public</url>
    </repository>

2. Inside dependencies tag block add:    
    
    
    <dependency>
        <groupId>org.hyperledger</groupId>
        <artifactId>indy</artifactId>
        <version>1.8.1-dev-985</version>
    </dependency>
     
Note that before you can use java wrapper you must install  c-callable SDK. 
See the section "How-to-install" in [Indy SDK](README.md)
### How to build

First, build the native "indy" library at https://github.com/hyperledger/indy-sdk:

	cargo build

Then copy the resulting `libindy.so` to `./lib/`.

Then run

    mvn clean install
  
#### Logging
The Java wrapper uses slf4j as a facade for various logging frameworks, such as java.util.logging, logback and log4j.

#### Troubleshooting
If your application that uses libindy crashes with a Null Pointer Exception then probably the libindy shared library could 
not be loaded properly. If you have build libindy from source then either put the resulting shared library where your
operating system searches for shared libraries or set appropriate environment variables to help the OS's loader to find them.

On Ubuntu either copy libindy.so to /usr/local/lib or set LD_LIBRARY_PATH to the directory that contains libindy.so.

```
export LD_LIBRARY_PATH=<path to libindy.so>
```

