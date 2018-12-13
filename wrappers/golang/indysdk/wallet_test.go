package indysdk_test

import (
	"errors"
	"os"
	"testing"

	"github.com/hyperledger/indy-sdk/wrappers/golang/indysdk"
	"github.com/hyperledger/indy-sdk/wrappers/golang/indysdk/utils"
	"github.com/stretchr/testify/assert"
)

var config = indysdk.WalletConfig{
	ID: "test_wallet",
}
var credential = indysdk.WalletCredential{
	Key: "test_wallet",
}

var exportConfig = indysdk.WalletExportConfig{
	Path: os.Getenv("HOME") + "/.indy_client/wallet_export",
	Key:  "test_wallet",
}

var importConfig = indysdk.WalletImportConfig{
	Path: os.Getenv("HOME") + "/.indy_client/wallet_export",
	Key:  "test_wallet",
}

/*
*** Create Wallet tests
 */
func TestCreateWallet(t *testing.T) {
	assert := assert.New(t)
	myconfig := config
	myconfig.StorageType = "default"
	assert.Equal(nil, indysdk.CreateWallet(myconfig, credential))
	cleanUp(0)
}

func TestCreateWalletWithEmptyStorageType(t *testing.T) {
	assert := assert.New(t)
	assert.Equal(nil, indysdk.CreateWallet(config, credential))
	cleanUp(0)
}

func TestCreateWalletWithEmptyName(t *testing.T) {
	assert := assert.New(t)
	myconfig := config
	myconfig.ID = ""
	err := errors.New(utils.GetIndyError(113))
	assert.Equal(err, indysdk.CreateWallet(myconfig, credential))
	cleanUp(0)
}

func TestCreateWalletWithUnknownStorageType(t *testing.T) {
	assert := assert.New(t)
	myconfig := config
	myconfig.StorageType = "unknown_type"
	err := errors.New(utils.GetIndyError(201))
	assert.Equal(err, indysdk.CreateWallet(myconfig, credential))
	cleanUp(0)
}

func TestCreateWalletWithDuplicateID(t *testing.T) {
	assert := assert.New(t)
	err := errors.New(utils.GetIndyError(203))
	indysdk.CreateWallet(config, credential)
	assert.Equal(err, indysdk.CreateWallet(config, credential))
	cleanUp(0)
}

// TODO: check error response
func testCreateWalletWithEmptyKey(t *testing.T) {
	assert := assert.New(t)
	mycredential := credential
	mycredential.Key = ""
	err := errors.New(utils.GetIndyError(201))
	assert.Equal(err, indysdk.CreateWallet(config, mycredential))
}

/*
*** Open Wallet tests
 */
func TestOpenWallet(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	wh, e := indysdk.OpenWallet(config, credential)
	assert.Equal(nil, e)
	assert.Equal(1, wh)
	cleanUp(wh)
}

func TestOpenWalletWithInvalidCredential(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	mycredential := credential
	mycredential.Key = "wrong_key"
	_, e := indysdk.OpenWallet(config, mycredential)
	err := errors.New(utils.GetIndyError(207))
	assert.Equal(err, e)
	cleanUp(0)
}

func TestOpenWalletForChangingCredentials(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	mycredential := credential
	mycredential.Rekey = "new_key"
	wh, e := indysdk.OpenWallet(config, mycredential)
	indysdk.CloseWallet(wh)
	mycredential.Key = "new_key"
	mycredential.Rekey = ""
	wh, e = indysdk.OpenWallet(config, mycredential)
	assert.Equal(nil, e)
	indysdk.CloseWallet(wh)
	indysdk.DeleteWallet(config, mycredential)
}

func TestOpenWalletForNotCreatedWallet(t *testing.T) {
	assert := assert.New(t)
	_, e := indysdk.OpenWallet(config, credential)
	err := errors.New(utils.GetIndyError(204))
	assert.Equal(err, e)
	cleanUp(0)
}

func TestOpenWalletTwice(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	wh, e := indysdk.OpenWallet(config, credential)
	_, e = indysdk.OpenWallet(config, credential)
	err := errors.New(utils.GetIndyError(206))
	assert.Equal(err, e)
	cleanUp(wh)
}

/*
*** Close Wallet tests
 */
func TestCloseWallet(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	wh, _ := indysdk.OpenWallet(config, credential)
	assert.Equal(nil, indysdk.CloseWallet(wh))
	cleanUp(wh)
}

func TestCloseWalletInvalidHandle(t *testing.T) {
	assert := assert.New(t)
	err := errors.New(utils.GetIndyError(200))
	assert.Equal(err, indysdk.CloseWallet(2))
}

/*
*** Delete Wallet tests
 */
func TestDeleteWallet(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	assert.Equal(nil, indysdk.DeleteWallet(config, credential))
	cleanUp(0)
}

func TestDeleteClosedWallet(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	wh, _ := indysdk.OpenWallet(config, credential)
	indysdk.CloseWallet(wh)
	assert.Equal(nil, indysdk.DeleteWallet(config, credential))
	cleanUp(wh)
}

func TestDeleteOpenedWallet(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	wh, _ := indysdk.OpenWallet(config, credential)
	err := errors.New(utils.GetIndyError(112))
	assert.Equal(err, indysdk.DeleteWallet(config, credential))
	cleanUp(wh)
}

func TestDeleteWalletTwice(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	indysdk.DeleteWallet(config, credential)
	err := errors.New(utils.GetIndyError(204))
	assert.Equal(err, indysdk.DeleteWallet(config, credential))
	cleanUp(0)
}

func TestDeleteWalletNotCreated(t *testing.T) {
	assert := assert.New(t)
	err := errors.New(utils.GetIndyError(204))
	assert.Equal(err, indysdk.DeleteWallet(config, credential))
	cleanUp(0)
}

/*
*** Export Wallet tests
 */
func TestExportWallet(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	wh, _ := indysdk.OpenWallet(config, credential)
	assert.Equal(nil, indysdk.ExportWallet(wh, exportConfig))
	assert.NotEqual(nil, os.MkdirAll(os.Getenv("HOME")+"/.indy_client/wallet_export", 0755))
	os.Remove(os.Getenv("HOME") + "/.indy_client/wallet_export")
	cleanUp(wh)
}

func TestExportWalletForExistingPath(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	wh, _ := indysdk.OpenWallet(config, credential)
	os.MkdirAll(os.Getenv("HOME")+"/.indy_client/wallet_export", 0755)
	err := errors.New(utils.GetIndyError(114))
	assert.Equal(err, indysdk.ExportWallet(wh, exportConfig))
	os.Remove(os.Getenv("HOME") + "/.indy_client/wallet_export")
	cleanUp(wh)
}

/*
*** Export Wallet tests
 */
func TestImportWallet(t *testing.T) {
	assert := assert.New(t)
	indysdk.CreateWallet(config, credential)
	wh, _ := indysdk.OpenWallet(config, credential)
	indysdk.ExportWallet(wh, exportConfig)
	indysdk.CloseWallet(wh)
	indysdk.DeleteWallet(config, credential)
	assert.Equal(nil, indysdk.ImportWallet(config, credential, importConfig))
	os.Remove(os.Getenv("HOME") + "/.indy_client/wallet_export")
	cleanUp(wh)
}

func TestImportWalletForNotExistingPath(t *testing.T) {
	assert := assert.New(t)
	err := errors.New(utils.GetIndyError(114))
	assert.Equal(err, indysdk.ImportWallet(config, credential, importConfig))
	cleanUp(0)
}

func cleanUp(wh int) {
	indysdk.CloseWallet(wh)
	indysdk.DeleteWallet(config, credential)
}
