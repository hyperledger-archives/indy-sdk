#include <string>
#include <map>
#include <nan.h>
#include "indy_core.h"

struct IndyCallback {
    indy_handle_t command_handle;
    Nan::Callback* callback;
    uv_async_t* uvHandle;

    indy_error_t err;
    const char* str0;
};

std::map<indy_handle_t, IndyCallback*> cbmap;

indy_handle_t next_command_handle = 0;
indy_handle_t getCommandHandle(){
    next_command_handle++;
    return next_command_handle;
}

void freeUvHandle (uv_handle_t* handle) {
    free(handle);
};

NAUV_WORK_CB(mainLoopReentry) {
    Nan::HandleScope scope;

    IndyCallback* icb = (struct IndyCallback*)async->data;

    v8::Local<v8::Value> argv[] = {
        Nan::New<v8::Number>(icb->err),
        Nan::New<v8::String>(icb->str0).ToLocalChecked()
    };
    icb->callback->Call(2, argv);


    cbmap.erase(icb->command_handle);

    delete icb->callback;
    icb->callback = NULL;

    delete icb->str0;
    icb->str0 = NULL;

    uv_close((uv_handle_t*)icb->uvHandle, freeUvHandle);
}

void indyCalled(indy_handle_t ch, Nan::Callback* callback, indy_error_t res) {

    if(res != 0){
        v8::Local<v8::Value> argv[] = {
            Nan::New<v8::Number>(res)
        };
        callback->Call(1, argv);
        delete callback;
        return;
    }

    IndyCallback* icb = new IndyCallback();
    icb->command_handle = ch;
    icb->callback = callback;
    icb->uvHandle = (uv_async_t*)malloc(sizeof(uv_async_t));
    icb->uvHandle->data = (void *)icb;

    uv_async_init(uv_default_loop(), icb->uvHandle, mainLoopReentry);

    cbmap[icb->command_handle] = icb;
}

char* copyCStr(const char* original){
    size_t len = strlen(original);
    char* tmp = new char[len];
    strncpy(tmp, original, len);
    return tmp;
}

// Now inject the generated C++ code (see /codegen/index.js)
#include "indy_codegen.h"
