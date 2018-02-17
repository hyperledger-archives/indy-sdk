#include <nan.h>
#include "indy_core.h"
#include <string>


indy_handle_t command_handle = 1;
Nan::Callback *callback;
struct Payload {
    indy_error_t err;
    std::string verkey;
};
uv_async_t* uvHandle = (uv_async_t*)malloc(sizeof(uv_async_t));


void freeUvHandle (uv_handle_t* handle) {
    free(handle);
};

NAUV_WORK_CB(asyncfunctionwat) {
    Nan::HandleScope scope;

    Payload* p = (struct Payload*)uvHandle->data;

    v8::Local<v8::Value> argv[] = {
        Nan::New<v8::Number>(p->err),
        Nan::New<v8::String>(p->verkey).ToLocalChecked()
    };
    callback->Call(2, argv);
    delete callback;
    delete p;

    uv_close((uv_handle_t*)uvHandle, freeUvHandle);
}

void abbreviate_verkey_cb(indy_handle_t resp_command_handle, indy_error_t resp_err, const char *const resp_verkey) {

    struct Payload* p = new Payload();
    p->err = resp_err;

    std::string verkey(resp_verkey);
    p->verkey = verkey;

    uvHandle->data = (void *)p;

    uv_async_send(uvHandle);
}


NAN_METHOD(hello) {
    info.GetReturnValue().Set(Nan::New("Hello indy!").ToLocalChecked());
}

NAN_METHOD(abbreviate_verkey) {

    if(info.Length() != 3) {
        return Nan::ThrowError(Nan::New("abbreviate_verkey expected 3 args").ToLocalChecked());
    }



    Nan::Utf8String didUTF(info[0]);
    const char* did = (const char*)(*didUTF);

    Nan::Utf8String verkeyUTF(info[1]);
    const char* verkey = (const char*)(*verkeyUTF);

    callback = new Nan::Callback(Nan::To<v8::Function>(info[2]).ToLocalChecked());

    uv_async_init(uv_default_loop(), uvHandle, asyncfunctionwat);

    indy_error_t res = indy_abbreviate_verkey(command_handle, did, verkey, abbreviate_verkey_cb);

    if(res != 0){
        v8::Local<v8::Value> argv[] = {
            Nan::New<v8::Number>(res)
        };
        callback->Call(1, argv);
        delete callback;
    }

}

#define EXPORT_FN(NAME) (Nan::Set(target, Nan::New(""#NAME"").ToLocalChecked(), Nan::GetFunction(Nan::New<v8::FunctionTemplate>(NAME)).ToLocalChecked()))

NAN_MODULE_INIT(InitAll) {
    EXPORT_FN(hello);
    EXPORT_FN(abbreviate_verkey);
}

NODE_MODULE(indy, InitAll)
