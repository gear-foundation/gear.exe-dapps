## The **manager** program

The program workspace includes the following packages:
- `manager` is the package allowing to build WASM binary for the program and IDL file for it.  
  The package also includes integration tests for the program in the `tests` sub-folder
- `manager-app` is the package containing business logic for the program represented by the `ManagerService` structure.  
- `manager-client` is the package containing the client for the program allowing to interact with it from another program, tests, or
  off-chain client.
