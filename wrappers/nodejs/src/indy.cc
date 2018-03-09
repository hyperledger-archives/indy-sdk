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

indy_handle_t next_command_handle = 0;

class IndyCallback : public Nan::AsyncResource {
  public:
    IndyCallback(v8::Local<v8::Function> callback_) : Nan::AsyncResource("IndyCallback") {
        callback.Reset(callback_);
        uvHandle.data = this;
        type = CB_NONE;
        next_command_handle++;
        command_handle = next_command_handle;
    }

    ~IndyCallback() {
        callback.Reset();

        delete str0;
        str0 = NULL;

        delete str1;
        str1 = NULL;

        delete buffer0data;
        buffer0data = NULL;
    }


    indy_handle_t command_handle;
    Nan::Persistent<v8::Function> callback;
    uv_async_t uvHandle;

    indy_error_t err;

    IndyCallbackType type;

    const char* str0;
    const char* str1;

    bool bool0;

    indy_handle_t handle0;

    char*    buffer0data;
    uint32_t buffer0size;
};

std::map<indy_handle_t, IndyCallback*> cbmap;


void freeUvHandle (uv_handle_t* handle) {
    // TODO fixme
    // IndyCallback* icb = static_cast<IndyCallback*>(handle->data);
    // delete icb;
};

NAUV_WORK_CB(mainLoopReentry) {
    Nan::HandleScope scope;

    IndyCallback* icb = static_cast<IndyCallback*>(async->data);
    cbmap.erase(icb->command_handle);

    int argc = icb->type == CB_NONE ? 1 : 2;

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

    v8::Local<v8::Object> target = Nan::New<v8::Object>();
    v8::Local<v8::Function> callback = Nan::New(icb->callback);
    icb->runInAsyncScope(target, callback, argc, argv);

    uv_close(reinterpret_cast<uv_handle_t*>(&icb->uvHandle), freeUvHandle);
}

void indyCalled(IndyCallback* icb, indy_error_t res) {
    if(res == 0) {
        return;
    }
    icb->err = res;
    uv_async_send(&icb->uvHandle);
}

char* copyCStr(const char* original){
    size_t len = strlen(original);
    char* tmp = new char[len];
    strncpy(tmp, original, len);
    return tmp;
}

// Now inject the generated C++ code (see /codegen/index.js)
#include "indy_codegen.h"
