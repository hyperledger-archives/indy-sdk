package indysdk

/*
#cgo CFLAGS: -I ../Includes
#cgo LDFLAGS: -lindy
#include <wrapper.h>
*/
import "C"
import (
	"github.com/hyperledger/indy-sdk/wrappers/golang/indysdk/wallet"
)

// CreateWallet creates a new secure wallet with the given unique name
func CreateWallet(config WalletConfig, credential WalletCredential) error {
	channel := wallet.IndyCreateWallet((wallet.Config)(config), (wallet.Credential)(credential))
	result := <-channel
	return result.Error
}

// OpenWallet opens an existing wallet
func OpenWallet(config WalletConfig, credential WalletCredential) (int, error) {
	channel := wallet.IndyOpenWallet((wallet.Config)(config), (wallet.Credential)(credential))
	result := <-channel
	if result.Error != nil {
		return 0, result.Error
	}
	return result.Results[0].(int), result.Error
}

// ExportWallet exports an opened wallet
func ExportWallet(walletHandle int, config WalletExportConfig) error {
	channel := wallet.IndyExportWallet(walletHandle, (wallet.ExportConfig)(config))
	result := <-channel
	return result.Error
}

// DeleteWallet deletes an existing wallet
func DeleteWallet(config WalletConfig, credential WalletCredential) error {
	channel := wallet.IndyDeleteWallet((wallet.Config)(config), (wallet.Credential)(credential))
	result := <-channel
	return result.Error
}

// CloseWallet closes an opened wallet
func CloseWallet(walletHandle int) error {
	channel := wallet.IndyCloseWallet(walletHandle)
	result := <-channel
	return result.Error
}

// ImportWallet imports contents from path in import config into the new created wallet
func ImportWallet(config WalletConfig, credential WalletCredential, importConfig WalletImportConfig) error {
	channel := wallet.IndyImportWallet((wallet.Config)(config), (wallet.Credential)(credential), (wallet.ImportConfig)(importConfig))
	result := <-channel
	return result.Error
}
