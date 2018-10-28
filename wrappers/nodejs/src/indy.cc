#include <string>
#include <map>
#include <queue>
#include <nan.h>
#include "indy_core.h"

char* copyCStr(const char* original){
    if(original == nullptr){
        return nullptr;
    }
    size_t len = strlen(original);
    char* dest = new char[len + 1];
    strncpy(dest, original, len);
    dest[len] = '\0';
    return dest;
}

v8::Local<v8::Value> toJSString(const char* str){
    if(str == nullptr){
        return Nan::Null();
    }
    return Nan::New<v8::String>(str).ToLocalChecked();
}


char* copyBuffer(const indy_u8_t* data, indy_u32_t len){
    char* dest = (char*)malloc(len * sizeof(char));
    memcpy(dest, data, len);
    return dest;
}

enum IndyCallbackType {
    CB_NONE,
    CB_STRING,
    CB_BOOLEAN,
    CB_HANDLE,
    CB_HANDLE_U32,
    CB_I32,
    CB_BUFFER,
    CB_STRING_BUFFER,
    CB_STRING_STRING,
    CB_STRING_STRING_TIMESTAMP,
    CB_STRING_STRING_STRING
};

class IndyCallback : public Nan::AsyncResource {
  public:
    IndyCallback(v8::Local<v8::Function> callback_) : Nan::AsyncResource("IndyCallback") {
        callback.Reset(callback_);
        uvHandle.data = this;
        type = CB_NONE;
        next_handle++;
        handle = next_handle;
        icbmap[handle] = this;
        uv_async_init(uv_default_loop(), &uvHandle, onMainLoopReentry);
        str0 = nullptr;
        str1 = nullptr;
        str2 = nullptr;
        buffer0data = nullptr;
    }

    ~IndyCallback() {
        callback.Reset();
        delete str0;
        delete str1;
        delete str2;
        // NOTE: do not `free(buffer0data)` b/c Nan::NewBuffer assumes ownership and node's garbage collector will free it.
    }

    void cbNone(indy_error_t xerr){
        send(xerr);
    }

    void cbString(indy_error_t xerr, const char* str){
        if(xerr == 0){
          type = CB_STRING;
          str0 = copyCStr(str);
        }
        send(xerr);
    }

    void cbStringString(indy_error_t xerr, const char* strA, const char* strB){
        if(xerr == 0){
          type = CB_STRING_STRING;
          str0 = copyCStr(strA);
          str1 = copyCStr(strB);
        }
        send(xerr);
    }

    void cbStringStringString(indy_error_t xerr, const char* strA, const char* strB, const char* strC){
        if(xerr == 0){
          type = CB_STRING_STRING_STRING;
          str0 = copyCStr(strA);
          str1 = copyCStr(strB);
          str2 = copyCStr(strC);
        }
        send(xerr);
    }

    void cbStringStringTimestamp(indy_error_t xerr, const char* strA, const char* strB, unsigned long long timestamp){
        if(xerr == 0){
          type = CB_STRING_STRING_TIMESTAMP;
          str0 = copyCStr(strA);
          str1 = copyCStr(strB);
          timestamp0 = timestamp;
        }
        send(xerr);
    }

    void cbBoolean(indy_error_t xerr, bool b){
        if(xerr == 0){
          type = CB_BOOLEAN;
          bool0 = b;
        }
        send(xerr);
    }

    void cbHandle(indy_error_t xerr, indy_handle_t h){
        if(xerr == 0){
          type = CB_HANDLE;
          handle0 = h;
        }
        send(xerr);
    }

    void cbHandleU32(indy_error_t xerr, indy_handle_t h, indy_u32_t n){
        if(xerr == 0){
          type = CB_HANDLE_U32;
          handle0 = h;
          u32int0 = n;
        }
        send(xerr);
    }

    void cbI32(indy_error_t xerr, indy_i32_t i){
        if(xerr == 0){
          type = CB_I32;
          i32int0 = i;
        }
        send(xerr);
    }

    void cbBuffer(indy_error_t xerr, const indy_u8_t* data, indy_u32_t len){
        if(xerr == 0){
            type = CB_BUFFER;
            buffer0data = copyBuffer(data, len);
            buffer0len = len;
        }
        send(xerr);
    }

    void cbStringBuffer(indy_error_t xerr, const char* str, const indy_u8_t* data, indy_u32_t len){
        if(xerr == 0){
            type = CB_STRING_BUFFER;
            str0 = copyCStr(str);
            buffer0data = copyBuffer(data, len);
            buffer0len = len;
        }
        send(xerr);
    }


    indy_handle_t handle;

    static IndyCallback* getCallback(indy_handle_t handle){
        if(icbmap.count(handle) == 0){
            return nullptr;
        }
        return icbmap[handle];
    }

  private:

    static indy_handle_t next_handle;
    static std::map<indy_handle_t, IndyCallback*> icbmap;

    Nan::Persistent<v8::Function> callback;
    uv_async_t uvHandle;

    IndyCallbackType type;
    indy_error_t err;
    const char* str0;
    const char* str1;
    const char* str2;
    bool bool0;
    indy_handle_t handle0;
    indy_i32_t i32int0;
    indy_u32_t u32int0;
    unsigned long long timestamp0;
    char*    buffer0data;
    uint32_t buffer0len;

    void send(indy_error_t xerr){
        err = xerr;
        uv_async_send(&uvHandle);
    }

    inline static NAUV_WORK_CB(onMainLoopReentry) {
        Nan::HandleScope scope;

        IndyCallback* icb = static_cast<IndyCallback*>(async->data);
        icbmap.erase(icb->handle);

        v8::Local<v8::Array> tuple;
        v8::Local<v8::Value> argv[2];
        argv[0] = Nan::New<v8::Number>(icb->err);
        switch(icb->type){
            case CB_NONE:
                argv[1] = Nan::Null();
                break;
            case CB_STRING:
                argv[1] = toJSString(icb->str0);
                break;
            case CB_BOOLEAN:
                argv[1] = Nan::New<v8::Boolean>(icb->bool0);
                break;
            case CB_HANDLE:
                argv[1] = Nan::New<v8::Number>(icb->handle0);
                break;
            case CB_HANDLE_U32:
                tuple = Nan::New<v8::Array>();
                tuple->Set(0, Nan::New<v8::Number>(icb->handle0));
                tuple->Set(1, Nan::New<v8::Number>(icb->u32int0));
                argv[1] = tuple;
                break;
            case CB_I32:
                argv[1] = Nan::New<v8::Number>(icb->i32int0);
                break;
            case CB_BUFFER:
                argv[1] = Nan::NewBuffer(icb->buffer0data, icb->buffer0len).ToLocalChecked();
                break;
            case CB_STRING_BUFFER:
                tuple = Nan::New<v8::Array>();
                tuple->Set(0, toJSString(icb->str0));
                tuple->Set(1, Nan::NewBuffer(icb->buffer0data, icb->buffer0len).ToLocalChecked());
                argv[1] = tuple;
                break;
            case CB_STRING_STRING:
                tuple = Nan::New<v8::Array>();
                tuple->Set(0, toJSString(icb->str0));
                tuple->Set(1, toJSString(icb->str1));
                argv[1] = tuple;
                break;
            case CB_STRING_STRING_TIMESTAMP:
                tuple = Nan::New<v8::Array>();
                tuple->Set(0, toJSString(icb->str0));
                tuple->Set(1, toJSString(icb->str1));
                tuple->Set(2, Nan::New<v8::Number>(icb->timestamp0));
                argv[1] = tuple;
                break;
            case CB_STRING_STRING_STRING:
                tuple = Nan::New<v8::Array>();
                tuple->Set(0, toJSString(icb->str0));
                tuple->Set(1, toJSString(icb->str1));
                tuple->Set(2, toJSString(icb->str2));
                argv[1] = tuple;
                break;
        }

        v8::Local<v8::Object> target = Nan::New<v8::Object>();
        v8::Local<v8::Function> callback = Nan::New(icb->callback);
        icb->runInAsyncScope(target, callback, 2, argv);

        uv_close(reinterpret_cast<uv_handle_t*>(&icb->uvHandle), onUvHandleClose);
    }

    inline static void onUvHandleClose(uv_handle_t* async) {
        Nan::HandleScope scope;
        IndyCallback* icb = static_cast<IndyCallback*>(async->data);
        delete icb;
    }
};

std::map<indy_handle_t, IndyCallback*> IndyCallback::icbmap;
indy_handle_t IndyCallback::next_handle = 0;

#define INDY_ASSERT_NARGS(FNAME, N) \
  if(info.Length() != N){ \
    return Nan::ThrowError(Nan::New(""#FNAME" expects "#N" arguments").ToLocalChecked()); \
  }

#define INDY_ASSERT_STRING(FNAME, I, ARGNAME) \
  if(!info[I]->IsString() && !info[I]->IsNull() && !info[I]->IsUndefined()){ \
    return Nan::ThrowTypeError(Nan::New(""#FNAME" expects String or null for "#ARGNAME"").ToLocalChecked()); \
  }

#define INDY_ASSERT_NUMBER(FNAME, I, ARGNAME) \
  if(!info[I]->IsNumber()){ \
    return Nan::ThrowTypeError(Nan::New(""#FNAME" expects Number for "#ARGNAME"").ToLocalChecked()); \
  }

#define INDY_ASSERT_BOOLEAN(FNAME, I, ARGNAME) \
  if(!info[I]->IsBoolean()){ \
    return Nan::ThrowTypeError(Nan::New(""#FNAME" expects Boolean for "#ARGNAME"").ToLocalChecked()); \
  }

#define INDY_ASSERT_UINT8ARRAY(FNAME, I, ARGNAME) \
  if(!info[I]->IsUint8Array()){ \
    return Nan::ThrowTypeError(Nan::New(""#FNAME" expects Uint8Array for "#ARGNAME"").ToLocalChecked()); \
  }

#define INDY_ASSERT_FUNCTION(FNAME, I) \
  if(!info[I]->IsFunction()){ \
    return Nan::ThrowTypeError(Nan::New(""#FNAME" expects Function for arg "#I"").ToLocalChecked()); \
  }


char* argToCString(v8::Local<v8::Value> arg){
    char* arg1 = nullptr;
    if(arg->IsString()){
        Nan::Utf8String* arg1UTF = new Nan::Utf8String(arg);
        arg1 = copyCStr((const char*)(**arg1UTF));
        delete arg1UTF;
    }
    return arg1;
}

IndyCallback* argToIndyCb(v8::Local<v8::Value> arg){
    IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(arg).ToLocalChecked());
    return icb;
}

void indyCalled(IndyCallback* icb, indy_error_t res) {
    if(res == 0) {
        return;
    }
    icb->cbNone(res);
}

NAN_METHOD(setDefaultLogger) {
  INDY_ASSERT_NARGS(setDefaultLogger, 1)
  INDY_ASSERT_STRING(setDefaultLogger, 0, pattern)
  const char* pattern = argToCString(info[0]);
  indy_error_t res = indy_set_default_logger(pattern);
  delete pattern;
  info.GetReturnValue().Set(res);
}

struct IndyLogEntry {
    indy_u32_t level;
    const char* target;
    const char* message;
    const char* module_path;
    const char* file;
    indy_u32_t line;
};
class IndyLogger : public Nan::AsyncResource {
  public:
    IndyLogger(v8::Local<v8::Function> logFn_) : Nan::AsyncResource("IndyLogger") {
        logFn.Reset(logFn_);
        uvHandle.data = this;
        uv_async_init(uv_default_loop(), &uvHandle, onMainLoopReentry);
    }

    ~IndyLogger() {
        logFn.Reset();
    }

    void cbLog(indy_u32_t level, const char* target, const char* message, const char* module_path, const char* file, indy_u32_t line){
        IndyLogEntry* entry = new IndyLogEntry();
        entry->level = level;
        entry->target = copyCStr(target);
        entry->message = copyCStr(message);
        entry->module_path = copyCStr(module_path);
        entry->file = copyCStr(file);
        entry->line = line;
        entries.push(entry);

        uv_async_send(&uvHandle);
    }

    void cbFlush(){
        uv_async_send(&uvHandle);
    }

  private:

    Nan::Persistent<v8::Function> logFn;
    uv_async_t uvHandle;
    std::queue<IndyLogEntry*> entries;

    inline static NAUV_WORK_CB(onMainLoopReentry) {
        Nan::HandleScope scope;

        IndyLogger* il = static_cast<IndyLogger*>(async->data);

        v8::Local<v8::Object> that = Nan::New<v8::Object>();
        v8::Local<v8::Function> logFn = Nan::New(il->logFn);

        while(!il->entries.empty()){
            IndyLogEntry* entry = il->entries.front();
            il->entries.pop();
            v8::Local<v8::Value> argv[6];
            argv[0] = Nan::New<v8::Number>(entry->level);
            argv[1] = toJSString(entry->target);
            argv[2] = toJSString(entry->message);
            argv[3] = toJSString(entry->module_path);
            argv[4] = toJSString(entry->file);
            argv[5] = Nan::New<v8::Number>(entry->line);
            delete entry;

            il->runInAsyncScope(that, logFn, 6, argv);
        }
    }
};
void setLogger_logFn(const void*  context, indy_u32_t level, const char* target, const char* message, const char* module_path, const char* file, indy_u32_t line){
    IndyLogger* il = (IndyLogger*) context;
    il->cbLog(level, target, message, module_path, file, line);
}
void setLogger_flushFn(const void*  context){
    IndyLogger* il = (IndyLogger*) context;
    il->cbFlush();
}
NAN_METHOD(setLogger) {
  INDY_ASSERT_NARGS(setLogger, 1)
  INDY_ASSERT_FUNCTION(setLogger, 0)
  IndyLogger* il = new IndyLogger(Nan::To<v8::Function>(info[0]).ToLocalChecked());
  indy_error_t res = indy_set_logger(il, nullptr, setLogger_logFn, setLogger_flushFn);
  info.GetReturnValue().Set(res);
}


// Now inject the generated C++ code (see /codegen/cpp.js)
#include "indy_codegen.h"
