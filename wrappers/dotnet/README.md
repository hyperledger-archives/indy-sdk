## Indy SDK for .NET

This is a **work-in-progress** .NET wrapper for [ Hyperledger Indy](https://www.hyperledger.org/projects/hyperledger-indy). It is implemented using PInvoke calls to a native c-callable library written in Rust. 
Hyperledger Indy is the open-source codebase behind the Sovrin network for self-sovereign digital identity.

The wrapper is designed to be platform independent and targets the .NET Standard 1.1. At present it has been tested on Windows and Ubuntu using .NET 4.5 and .NET Core 2.0.

Pull requests welcome!

### Documentation

Documentation for the .NET wrapper is available [here](http://hyperledger.github.io/indy-sdk/wrappers/dotnet/docs/index.html).

### How to build

Simply build the indy-sdk-dotnet.sln file using Visual Studio, msbuild, dotnet or whatever build system your .NET implementation and platform supports.  

The project also includes a NuGet package definition which can be built using the 'dotnet pack' command or by choosing publish on the project in Visual Studio.

### Using the Wrapper

The .NET wrapper can be used in any .NET project by referencing the NuGet package which can be built using the instructions above or obtained from the 
[nuget.org](https://www.nuget.org/packages/Hyperledger.Indy.Sdk) package repository.  Please note that the version available on nuget.org is pre-release only! 

Before attempting to use the .NET wrapper the c-callable SDK must be installed in the library/search path - see the [Binaries](../../README.md#binaries) section of the main project page to download 
the available c-callable binaries and their dependencies or the [Building Indy SDK](../../README.md#building-indy-sdk) for information on how to build your own.

Integration tests are available in the indy-sdk-dotnet-test project and can be executed once the c-callable SDK has been installed and a node pool is available.  See the information on building
the SDK for your specific platform in the [Building Indy SDK](../../README.md#building-indy-sdk) section of the main project page for information on how to set up a node pool for running integration tests.
 
### Example use

For a sample project that contains executable demo code showing various usages of the .NET SDK wrapper see the [.NET Sample](../../samples/dotnet/README.md).

#### Troubleshooting
Use environment variable `RUST_LOG={info|debug|trace}` to output logs of Libindy.
