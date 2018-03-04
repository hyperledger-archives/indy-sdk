#include <string>
#include <map>
#include <nan.h>
#include "indy_core.h"

enum IndyCallbackType {
    CB_NONE,
    CB_STRING,
    CB_BOOLEAN,
    CB_HANDLE,
    CB_BUFFER,
    CB_STRING_BUFFER,
    CB_STRING_STRING
};

struct IndyCallback {
    indy_handle_t command_handle;
    Nan::Callback* callback;
    uv_async_t* uvHandle;

    indy_error_t err;

    IndyCallbackType type;

    const char* str0;
    const char* str1;

    bool bool0;

    indy_handle_t handle0;

    char* buffer0data;
    uint32_t    buffer0size;
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

    int argc = 1;
    switch(icb->type){
        case CB_NONE:
            argc = 1;
            break;
        case CB_STRING:
        case CB_BOOLEAN:
        case CB_HANDLE:
        case CB_BUFFER:
            argc = 2;
            break;
        case CB_STRING_BUFFER:
        case CB_STRING_STRING:
            argc = 3;
            break;
    }

    v8::Local<v8::Array> tuple;
    v8::Local<v8::Value> argv[argc];
    argv[0] = Nan::New<v8::Number>(icb->err);
    switch(icb->type){
        case CB_NONE:
            // nothing
            break;
        case CB_STRING:
            argv[1] = Nan::New<v8::String>(icb->str0).ToLocalChecked();
            break;
        case CB_BOOLEAN:
            argv[1] = Nan::New<v8::Boolean>(icb->bool0);
            break;
        case CB_HANDLE:
            argv[1] = Nan::New<v8::Number>(icb->handle0);
            break;
        case CB_BUFFER:
            argv[1] = Nan::NewBuffer(icb->buffer0data, icb->buffer0size).ToLocalChecked();
            break;
        case CB_STRING_BUFFER:
            tuple = Nan::New<v8::Array>();
            tuple->Set(0, Nan::New<v8::String>(icb->str0).ToLocalChecked());
            tuple->Set(0, Nan::NewBuffer(icb->buffer0data, icb->buffer0size).ToLocalChecked());
            argv[1] = tuple;
            break;
        case CB_STRING_STRING:
            tuple = Nan::New<v8::Array>();
            tuple->Set(0, Nan::New<v8::String>(icb->str0).ToLocalChecked());
            tuple->Set(1, Nan::New<v8::String>(icb->str1).ToLocalChecked());
            argv[1] = tuple;
            break;
    }

    icb->callback->Call(argc, argv);


    cbmap.erase(icb->command_handle);

    delete icb->callback;
    icb->callback = NULL;

    delete icb->str0;
    icb->str0 = NULL;

    delete icb->str1;
    icb->str1 = NULL;

    delete icb->buffer0data;
    icb->buffer0data = NULL;

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
    icb->type = CB_NONE;

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
