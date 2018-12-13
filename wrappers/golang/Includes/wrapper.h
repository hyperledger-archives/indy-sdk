#pragma GCC diagnostic ignored "-Wincompatible-pointer-types"
#define LAMBDA(c_) ({ c_ _;})

#include "indy_core.h"

// define golang function in the service.go of each package that uses defaultCallbacks
extern void defaultCallback(indy_handle_t, indy_error_t, indy_handle_t);


static void (*get_default_callback(void)) (indy_handle_t command_handle, indy_error_t err)
{
	return LAMBDA(void _(indy_handle_t h, indy_error_t e){
		defaultCallback(h, e, 0);
	});
    
}

static void (*get_int_callback(void)) (indy_handle_t command_handle, indy_error_t err)
{
	return LAMBDA(void _(indy_handle_t h, indy_error_t e, indy_handle_t v){
		defaultCallback(h, e, v);
	});
}