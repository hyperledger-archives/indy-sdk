package wallet

// StorageConfig represents Indy wallet storage config
type StorageConfig struct {
	Path string `json:"path"`
}

// Config represents Indy wallet config
type Config struct {
	ID            string        `json:"id"`
	StorageType   string        `json:"storage_type"`
	StorageConfig StorageConfig `json:"storage_config"`
}

// Credential represents Indy wallet credential config
type Credential struct {
	Key                   string `json:"key"`
	Rekey                 string `json:"rekey,omitempty"`
	StorageCredentials    string `json:"storage_credentials"`
	KeyDerivationMethod   string `json:"key_derivation_method,omitempty"`
	ReKeyDerivationMethod string `json:"rekey_derivation_method,omitempty"`
}

// ExportConfig represents Indy wallet export config
type ExportConfig struct {
	Path                string `json:"path"`
	Key                 string `json:"key"`
	KeyDerivationMethod string `json:"key_derivation_method,omitempty"`
}

// ImportConfig represents Indy wallet import config
type ImportConfig ExportConfig
