#include <nan.h>
#include "indy_core.h"
#include <string>


indy_handle_t command_handle = 1;
Nan::Callback *callback;
struct Payload {
    indy_error_t err;
    const char* verkey;
};
uv_async_t* uvHandle = (uv_async_t*)malloc(sizeof(uv_async_t));


void freeUvHandle (uv_handle_t* handle) {
    // free(handle);
};

NAUV_WORK_CB(asyncfunctionwat) {
    Nan::HandleScope scope;

    Payload* p = (Payload*)uvHandle->data; 
    const char* response = p->verkey;
    printf("[SECOND]%s[WAT]", p->verkey);

    v8::Local<v8::Value> argv[] = {
        Nan::New<v8::Number>(p->err),
        Nan::New<v8::String>(response).ToLocalChecked()
    };
    callback->Call(2, argv);
    delete callback;

    uv_close((uv_handle_t*)uvHandle, freeUvHandle);
}

void abbreviate_verkey_cb(indy_handle_t resp_command_handle, indy_error_t resp_err, const char *const resp_verkey) {

    Payload* p = new Payload();
    p->err = resp_err;
    p->verkey = resp_verkey;
    printf("[FIRST]%s[WAT]", p->verkey);
    uvHandle->data = (void *)p;

    uv_async_send(uvHandle);
}


NAN_METHOD(hello) {
    info.GetReturnValue().Set(Nan::New("Hello indy!").ToLocalChecked());
}

NAN_METHOD(abbreviate_verkey) {

    std::string did = "VsKV7grR1BUE29mG2Fm2kX";
    std::string verkey = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";

    callback = new Nan::Callback(Nan::To<v8::Function>(info[0]).ToLocalChecked());

    uv_async_init(uv_default_loop(), uvHandle, asyncfunctionwat);

    indy_error_t res = indy_abbreviate_verkey(command_handle, did.c_str(), verkey.c_str(), abbreviate_verkey_cb);

    if(res != 0){
        v8::Local<v8::Value> argv[] = {
            Nan::New<v8::Number>(res)
        };
        callback->Call(1, argv);
        delete callback;
    }

}

NAN_MODULE_INIT(InitAll) {
    Nan::Set(target,
        Nan::New("hello").ToLocalChecked(),
        Nan::GetFunction(Nan::New<v8::FunctionTemplate>(hello)).ToLocalChecked()
    );

    Nan::Set(target,
        Nan::New("abbreviate_verkey").ToLocalChecked(),
        Nan::GetFunction(Nan::New<v8::FunctionTemplate>(abbreviate_verkey)).ToLocalChecked()
    );
}

NODE_MODULE(indy, InitAll)
