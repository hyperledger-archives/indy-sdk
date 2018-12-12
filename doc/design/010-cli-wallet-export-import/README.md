# Payment Interface

This design proposes the list of commands to Indy CLI to handle export/import wallet operations.

## Goals and ideas
Indy CLI should provide ability to perform following operation:
* Allow users to export their wallets so the can do the backup or move to different device.
* Allow users to import exported wallet.

## New CLI commands

### Export wallet

Exports opened wallet to the specified file.

```indy-cli
indy> wallet export export_path=<path-to-file> export_key=[<export key>]
```

Returns:

* Success or error message

### Import wallet

Create new wallet and then import content from the specified file

```indy-cli
indy> wallet import <wallet name> key=<key> export_path=<path-to-file> export_key=<key used for export>  [storage_type=<storage_type>] [storage_config={config json}]
```

Returns:

* Success or error message