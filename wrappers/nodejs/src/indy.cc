#include <nan.h>
using v8::FunctionTemplate;

NAN_METHOD(hello) {
    info.GetReturnValue().Set(Nan::New("Hello indy!").ToLocalChecked());
}

NAN_MODULE_INIT(InitAll) {
    Nan::Set(target,
        Nan::New("hello").ToLocalChecked(),
        Nan::GetFunction(Nan::New<FunctionTemplate>(hello)).ToLocalChecked()
    );
}

NODE_MODULE(indy, InitAll)
