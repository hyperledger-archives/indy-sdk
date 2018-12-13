package wallet

/*
#cgo CFLAGS: -I ../../Includes
#cgo LDFLAGS: -lindy
#include <wrapper.h>
*/
import "C"

import (
	"encoding/json"
	"errors"
	"os"

	"github.com/hyperledger/indy-sdk/wrappers/golang/indysdk/utils"
	//log "github.com/sirupsen/logrus"
)

//export defaultCallback
func defaultCallback(commandHandle C.indy_handle_t, indyError C.indy_error_t, value C.int) {
	if indyError == 0 {
		utils.RemoveFuture((int)(commandHandle), utils.IndyResult{Error: nil, Results: []interface{}{int(value)}})
	} else {
		errMsg := utils.GetIndyError(int(indyError))
		utils.RemoveFuture((int)(commandHandle), utils.IndyResult{Error: errors.New(errMsg)})
	}
}

func setDefaults(config Config, credential Credential) (Config, Credential) {
	if config.StorageType == "" {
		config.StorageType = "default"
	}
	if config.StorageConfig.Path == "" {
		config.StorageConfig.Path = os.Getenv("HOME") + "/.indy_client/wallet"
	}
	return config, credential
}

// IndyCreateWallet creates a new secure wallet with the given unique name
func IndyCreateWallet(config Config, credential Credential) chan utils.IndyResult {
	config, credential = setDefaults(config, credential)
	handle, future := utils.NewFutureCommand()

	jsonConfig, err := json.Marshal(config)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}
	jsonCredential, err := json.Marshal(credential)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}

	configString := string(jsonConfig)
	credentialString := string(jsonCredential)
	commandHandle := (C.indy_handle_t)(handle)
	res := C.indy_create_wallet(commandHandle, C.CString(configString), C.CString(credentialString), C.get_default_callback())
	if res != 0 {
		errMsg := utils.GetIndyError(int(res))
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: errors.New(errMsg)}) }()
		return future
	}

	return future
}

// IndyOpenWallet opens an existing indy wallet
func IndyOpenWallet(config Config, credential Credential) chan utils.IndyResult {
	config, credential = setDefaults(config, credential)
	handle, future := utils.NewFutureCommand()

	jsonConfig, err := json.Marshal(config)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}
	jsonCredential, err := json.Marshal(credential)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}

	configString := string(jsonConfig)
	credentialString := string(jsonCredential)
	commandHandle := (C.indy_handle_t)(handle)
	if res := C.indy_open_wallet(commandHandle, C.CString(configString), C.CString(credentialString), C.get_int_callback()); res != 0 {
		errMsg := utils.GetIndyError(int(res))
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: errors.New(errMsg)}) }()
		return future
	}

	return future
}

// IndyExportWallet exports an opened indy wallet
func IndyExportWallet(walletHandle int, config ExportConfig) chan utils.IndyResult {
	handle, future := utils.NewFutureCommand()
	if config.Path == "" {
		config.Path = os.Getenv("HOME") + "/.indy_client/exported_wallet"
	}

	jsonConfig, err := json.Marshal(config)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}

	configString := string(jsonConfig)
	commandHandle := (C.indy_handle_t)(handle)
	wh := (C.indy_handle_t)(walletHandle)
	if res := C.indy_export_wallet(commandHandle, wh, C.CString(configString), C.get_default_callback()); res != 0 {
		errMsg := utils.GetIndyError(int(res))
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: errors.New(errMsg)}) }()
		return future
	}

	return future
}

// IndyDeleteWallet deletes a created indy wallet
func IndyDeleteWallet(config Config, credential Credential) chan utils.IndyResult {
	config, credential = setDefaults(config, credential)
	handle, future := utils.NewFutureCommand()

	jsonConfig, err := json.Marshal(config)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}
	jsonCredential, err := json.Marshal(credential)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}

	configString := string(jsonConfig)
	credentialString := string(jsonCredential)
	commandHandle := (C.indy_handle_t)(handle)
	res := C.indy_delete_wallet(commandHandle, C.CString(configString), C.CString(credentialString), C.get_default_callback())
	if res != 0 {
		errMsg := utils.GetIndyError(int(res))
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: errors.New(errMsg)}) }()
		return future
	}

	return future
}

// IndyCloseWallet closes an already opened indy wallet
func IndyCloseWallet(walletHandle int) chan utils.IndyResult {
	handle, future := utils.NewFutureCommand()

	commandHandle := (C.indy_handle_t)(handle)
	wh := (C.indy_handle_t)(walletHandle)
	if res := C.indy_close_wallet(commandHandle, wh, C.get_default_callback()); res != 0 {
		errMsg := utils.GetIndyError(int(res))
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: errors.New(errMsg)}) }()
		return future
	}

	return future
}

// IndyImportWallet creates a new secure wallet and then imports its content according to fields provided in importConfig
func IndyImportWallet(config Config, credential Credential, importConfig ImportConfig) chan utils.IndyResult {
	config, credential = setDefaults(config, credential)
	handle, future := utils.NewFutureCommand()

	jsonConfig, err := json.Marshal(config)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}
	jsonCredential, err := json.Marshal(credential)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}
	jsonImport, err := json.Marshal(importConfig)
	if err != nil {
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: err}) }()
		return future
	}

	configString := string(jsonConfig)
	credentialString := string(jsonCredential)
	importString := string(jsonImport)
	commandHandle := (C.indy_handle_t)(handle)
	res := C.indy_import_wallet(commandHandle, C.CString(configString), C.CString(credentialString), C.CString(importString), C.get_default_callback())
	if res != 0 {
		errMsg := utils.GetIndyError(int(res))
		go func() { utils.RemoveFuture((int)(handle), utils.IndyResult{Error: errors.New(errMsg)}) }()
		return future
	}

	return future
}
