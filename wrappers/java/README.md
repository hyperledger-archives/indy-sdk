## Indy SDK for Java

This Java wrapper currently requires Java 8 (e.g. the openjdk-8-jdk package in Debian/Ubuntu).

Pull requests welcome!

**Not ready for production use! Not all commands work properly! Use at your own risk!**
### How to install
In your maven project add to pom.xml file next content:

1. Inside repositories tag block add:
    
    
    <repository>
        <id>evernym</id>
        <url>https://repo.evernym.com/artifactory/libindy-maven-local</url>
    </repository>

2. Inside dependencies tag block add:    
    
    
    <dependency>
        <groupId>org.hyperledger</groupId>
        <artifactId>indy</artifactId>
        <version>1.3.1-dev-410</version>
    </dependency>
     
Note that before you can use java wrapper you must install  c-callable SDK. 
See the section "How-to-install" in [Indy SDK](README.md)
### How to build

First, build the native "indy" library at https://github.com/hyperledger/indy-sdk:

	cargo build

Then copy the resulting `libindy.so` to `./lib/`.

Then run

    mvn clean install
