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

class IndyCallback : public Nan::AsyncResource {
  public:
    IndyCallback(v8::Local<v8::Function> callback_) : Nan::AsyncResource("IndyCallback") {
        callback.Reset(callback_);
        uvHandle.data = this;
        type = CB_NONE;
        next_command_handle++;
        command_handle = next_command_handle;
        icbmap[command_handle] = this;
        uv_async_init(uv_default_loop(), &uvHandle, mainLoopReentry);
    }

    ~IndyCallback() {
        callback.Reset();
        switch(type){
            case CB_STRING:
                delete str0;
                break;
            case CB_BUFFER:
                delete buffer0data;
                break;
            case CB_STRING_BUFFER:
                delete str0;
                delete buffer0data;
                break;
            case CB_STRING_STRING:
                delete str0;
                delete str1;
                break;
            case CB_NONE:
            case CB_BOOLEAN:
            case CB_HANDLE:
                break;
        }
    }


    indy_handle_t command_handle;
    Nan::Persistent<v8::Function> callback;
    uv_async_t uvHandle;

    IndyCallbackType type;
    indy_error_t err;

    const char* str0;
    const char* str1;

    bool bool0;

    indy_handle_t handle0;

    char*    buffer0data;
    uint32_t buffer0size;

    static IndyCallback* getCallback(indy_handle_t handle){
        if(icbmap.count(handle) == 0){
            return nullptr;
        }
        return icbmap[handle];
    }

  private:

    static indy_handle_t next_command_handle;

    static std::map<indy_handle_t, IndyCallback*> icbmap;

    inline static NAUV_WORK_CB(mainLoopReentry) {
        Nan::HandleScope scope;

        IndyCallback* icb = static_cast<IndyCallback*>(async->data);
        icbmap.erase(icb->command_handle);

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

    inline static void freeUvHandle(uv_handle_t* handle) {
        Nan::HandleScope scope;
        IndyCallback* icb = static_cast<IndyCallback*>(handle->data);
        delete icb;
    }
};

std::map<indy_handle_t, IndyCallback*> IndyCallback::icbmap;
indy_handle_t IndyCallback::next_command_handle = 0;

void indyCalled(IndyCallback* icb, indy_error_t res) {
    if(res == 0) {
        return;
    }
    icb->err = res;
    uv_async_send(&icb->uvHandle);
}

char* copyCStr(const char* original){
    size_t len = strlen(original);
    char* tmp = new char[len + 1];
    strncpy(tmp, original, len);
    tmp[len] = '\0';
    return tmp;
}

// Now inject the generated C++ code (see /codegen/index.js)
#include "indy_codegen.h"
