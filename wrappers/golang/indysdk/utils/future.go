package utils

/*
#cgo CFLAGS: -I ../../Includes
#cgo LDFLAGS: -lindy
#include <wrapper.h>
*/
import "C"

// IndyResult represents callback result from C-call to libindy
type IndyResult struct {
	Error   error
	Results []interface{}
}

var futures = make(map[C.indy_handle_t](chan IndyResult))
var count int32

// NewFutureCommand creates a new future command
func NewFutureCommand() (C.indy_handle_t, chan IndyResult) {
	commandHandle := (C.indy_handle_t)(count)
	future := make(chan IndyResult)
	futures[commandHandle] = future
	count = count + 1
	return commandHandle, future
}

// RemoveFuture removes a future from the futures map
func RemoveFuture(handle int, result IndyResult) chan IndyResult {
	future := futures[(C.indy_handle_t)(handle)]
	future <- result
	delete(futures, (C.indy_handle_t)(handle))
	return future
}
