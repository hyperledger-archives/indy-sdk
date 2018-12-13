## Go Wrapper wallet

Structs Fields:

- WalletStorageConfig:
    * Path: string

- WalletConfig:
    * ID: string
    * StorageType: string (optional)
    * StorageConfig: WalletStorageConfig (optional)

- WalletCredential:
    * Key: string
    * Rekey: string (optional)
    * StorageCredentials: string (optional)
    * KeyDerivationMethod: string (optional)
    * ReKeyDerivationMethod: string (optional)

- WalletExportConfig:
    * Path: string
    * Key: string
    * KeyDerivationMethod: string (optional)

- WalletImportConfig:
    * Path: string
    * Key: string
    * KeyDerivationMethod: string (optional)


Functions:

* CreateWallet(config WalletConfig, credential WalletCredential) error

* OpenWallet(config WalletConfig, credential WalletCredential) (int, error)

* CloseWallet(walletHandle int) error

* DeleteWallet(config WalletConfig, credential WalletCredential) error

* ExportWallet(walletHandle int, config WalletExportConfig) error

* ImportWallet(config WalletConfig, credential WalletCredential, importConfig WalletImportConfig) error