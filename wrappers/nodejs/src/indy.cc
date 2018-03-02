#include <string>
#include <map>
#include <nan.h>
#include "indy_core.h"

struct IndyCallback {
    indy_handle_t command_handle;
    Nan::Callback* callback;
    uv_async_t* uvHandle;

    indy_error_t err;
    const char* verkey;
};

std::map<indy_handle_t, IndyCallback*> cbmap;

indy_handle_t next_command_handle = 0;

void freeUvHandle (uv_handle_t* handle) {
    free(handle);
};

NAUV_WORK_CB(asyncfunctionwat) {
    Nan::HandleScope scope;

    IndyCallback* icb = (struct IndyCallback*)async->data;

    v8::Local<v8::Value> argv[] = {
        Nan::New<v8::Number>(icb->err),
        Nan::New<v8::String>(icb->verkey).ToLocalChecked()
    };
    icb->callback->Call(2, argv);


    cbmap.erase(icb->command_handle);

    delete icb->callback;
    icb->callback = NULL;

    delete icb->verkey;
    icb->verkey = NULL;

    uv_close((uv_handle_t*)icb->uvHandle, freeUvHandle);
}

IndyCallback* initIndyCallback(v8::Local<v8::Value> callbackArg) {
    next_command_handle++;

    IndyCallback* icb = new IndyCallback();
    icb->command_handle = next_command_handle;
    icb->callback = new Nan::Callback(Nan::To<v8::Function>(callbackArg).ToLocalChecked());
    icb->uvHandle = (uv_async_t*)malloc(sizeof(uv_async_t));
    icb->uvHandle->data = (void *)icb;

    uv_async_init(uv_default_loop(), icb->uvHandle, asyncfunctionwat);

    cbmap[icb->command_handle] = icb;

    return icb;
}

char* copyCStr(const char* original){
    size_t len = strlen(original);
    char* tmp = new char[len];
    strncpy(tmp, original, len);
    return tmp;
}


void abbreviate_verkey_cb(indy_handle_t resp_command_handle, indy_error_t resp_err, const char *const resp_verkey) {

    IndyCallback* icb = cbmap[resp_command_handle];

    icb->err = resp_err;
    if(icb->err == 0){
        icb->verkey = copyCStr(resp_verkey);
    }

    uv_async_send(icb->uvHandle);
}

NAN_METHOD(abbreviate_verkey) {

    if(info.Length() != 3) {
        return Nan::ThrowError(Nan::New("abbreviate_verkey expected 3 args").ToLocalChecked());
    }

    if(!info[0]->IsString()) {
        return Nan::ThrowError(Nan::New("abbreviate_verkey arg 0 expected String").ToLocalChecked());
    }
    Nan::Utf8String arg0UTF(info[0]);
    const char* arg0 = (const char*)(*arg0UTF);

    if(!info[1]->IsString()) {
        return Nan::ThrowError(Nan::New("abbreviate_verkey arg 1 expected String").ToLocalChecked());
    }
    Nan::Utf8String arg1UTF(info[1]);
    const char* arg1 = (const char*)(*arg1UTF);

    if(!info[2]->IsFunction()) {
        return Nan::ThrowError(Nan::New("abbreviate_verkey arg 2 expected Function").ToLocalChecked());
    }
    IndyCallback* icb = initIndyCallback(info[2]);

    indy_error_t res = indy_abbreviate_verkey(icb->command_handle, arg0, arg1, abbreviate_verkey_cb);

    if(res != 0){
        v8::Local<v8::Value> argv[] = {
            Nan::New<v8::Number>(res)
        };
        icb->callback->Call(1, argv);
        delete icb->callback;
    }
}


////////////////////////////////////////////////////////////////////////////////
//
// Export the JS functions
//

#define EXPORT_FN(NAME) (Nan::Set(target, Nan::New(""#NAME"").ToLocalChecked(), Nan::GetFunction(Nan::New<v8::FunctionTemplate>(NAME)).ToLocalChecked()))

NAN_MODULE_INIT(InitAll) {
    EXPORT_FN(abbreviate_verkey);
}

NODE_MODULE(indy, InitAll)
