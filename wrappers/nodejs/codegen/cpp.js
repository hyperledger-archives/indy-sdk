var fs = require('fs')
var path = require('path')
var apiFunctions = require('./apiFunctions')

var OUT_FILE = path.resolve(__dirname, '../src/indy_codegen.h')

var normalizeType = function (param) {
  if (param.timestamp) {
    return 'Timestamp'
  }
  switch (param.type.replace(/[^a-z0-9_*]/ig, '')) {
    case 'constchar*':
    case 'constchar*const':
      return 'String'

    case 'indy_bool_t':
      return 'Boolean'

    case 'indy_handle_t':
      return 'IndyHandle'

    case 'indy_error_t':
      return 'IndyError'

    case 'void':
    case 'indy_u32_t':
    case 'indy_i32_t':
    case 'indy_u64_t':
      return param.type

    case 'Buffer':
      return 'Buffer'
  }
  throw new Error('normalizeType doesn\'t handle: ' + param.type + ' ' + JSON.stringify(param))
}

var cpp = ''

apiFunctions.forEach(function (fn) {
  var cppReturnThrow = function (msg) {
    var errmsg = JSON.stringify(msg + ': ' + fn.humanSignature)
    return '    return Nan::ThrowError(Nan::New(' + errmsg + ').ToLocalChecked());\n'
  }

  cpp += 'void ' + fn.jsName + '_cb(indy_handle_t handle, indy_error_t xerr'
  cpp += fn.jsCbParams.map(function (arg, i) {
    if (arg.type === 'Buffer') {
      return ', const indy_u8_t* arg' + i + 'data, indy_u32_t arg' + i + 'len'
    }
    return ', ' + arg.type + ' arg' + i
  }).join('')
  cpp += ') {\n'
  cpp += '  IndyCallback* icb = IndyCallback::getCallback(handle);\n'
  cpp += '  if(icb != nullptr){\n'
  var cbArgTypes = fn.jsCbParams.map(arg => normalizeType(arg)).join('+')
  switch (cbArgTypes) {
    case '':
      cpp += '    icb->cbNone(xerr);\n'
      break
    case 'String':
      cpp += '    icb->cbString(xerr, arg0);\n'
      break
    case 'Boolean':
      cpp += '    icb->cbBoolean(xerr, arg0);\n'
      break
    case 'IndyHandle':
      cpp += '    icb->cbHandle(xerr, arg0);\n'
      break
    case 'indy_i32_t':
      cpp += '    icb->cbI32(xerr, arg0);\n'
      break
    case 'String+String':
      cpp += '    icb->cbStringString(xerr, arg0, arg1);\n'
      break
    case 'String+String+String':
      cpp += '    icb->cbStringStringString(xerr, arg0, arg1, arg2);\n'
      break
    case 'String+String+Timestamp':
      cpp += '    icb->cbStringStringTimestamp(xerr, arg0, arg1, arg2);\n'
      break
    case 'Buffer':
      cpp += '    icb->cbBuffer(xerr, arg0data, arg0len);\n'
      break
    case 'String+Buffer':
      cpp += '    icb->cbStringBuffer(xerr, arg0, arg1data, arg1len);\n'
      break
    default:
      throw new Error('Unhandled callback args type: ' + cbArgTypes + ' for ' + fn.name)
  }
  cpp += '  }\n'
  cpp += '}\n'
  cpp += 'NAN_METHOD(' + fn.jsName + ') {\n'
  cpp += '  if(info.Length() != ' + (fn.jsParams.length + 1) + '){\n'
  cpp += cppReturnThrow('Expected ' + (fn.jsParams.length + 1) + ' arguments')
  cpp += '  }\n'
  fn.jsParams.forEach(function (arg, i) {
    var type = normalizeType(arg)

    var chkType = function (isfn) {
      cpp += '  if(!info[' + i + ']->' + isfn + '()){\n'
      cpp += cppReturnThrow('Expected ' + type + ' for ' + arg.name)
      cpp += '  }\n'
    }

    switch (type) {
      case 'String':
        cpp += '  Nan::Utf8String* arg' + i + 'UTF = nullptr;\n'
        cpp += '  const char* arg' + i + ' = nullptr;\n'
        cpp += '  if(info[' + i + ']->IsString()){\n'
        cpp += '    arg' + i + 'UTF = new Nan::Utf8String(info[' + i + ']);\n'
        cpp += '    arg' + i + ' = (const char*)(**arg' + i + 'UTF);\n'
        cpp += '  } else if(!info[' + i + ']->IsNull() && !info[' + i + ']->IsUndefined()){\n'
        cpp += cppReturnThrow('Expected String or null for ' + arg.name)
        cpp += '  }\n'
        break
      case 'IndyHandle':
        chkType('IsNumber')
        cpp += '  indy_handle_t arg' + i + ' = info[' + i + ']->Int32Value();\n'
        break
      case 'indy_u32_t':
        chkType('IsUint32')
        cpp += '  indy_u32_t arg' + i + ' = info[' + i + ']->Uint32Value();\n'
        break
      case 'indy_i32_t':
        chkType('IsInt32')
        cpp += '  indy_i32_t arg' + i + ' = info[' + i + ']->Int32Value();\n'
        break
      case 'indy_u64_t':
        chkType('IsUint32')
        cpp += '  indy_u64_t arg' + i + ' = (indy_u64_t)info[' + i + ']->Uint32Value();\n'
        break
      case 'Timestamp':
        chkType('IsUint32')
        cpp += '  long long arg' + i + ' = info[' + i + ']->Uint32Value();\n'
        break
      case 'Boolean':
        chkType('IsBoolean')
        cpp += '  indy_bool_t arg' + i + ' = info[' + i + ']->IsTrue();\n'
        break
      case 'Buffer':
        chkType('IsUint8Array')
        cpp += '  const indy_u8_t* arg' + i + 'data = (indy_u8_t*)node::Buffer::Data(info[' + i + ']->ToObject());\n'
        cpp += '  indy_u32_t arg' + i + 'len = node::Buffer::Length(info[' + i + ']);\n'
        break
      default:
        throw new Error('Unhandled argument reading type: ' + type)
    }
  })
  cpp += '  if(!info[' + fn.jsParams.length + ']->IsFunction()) {\n'
  cpp += '    return Nan::ThrowError(Nan::New("' + fn.jsName + ' arg ' + fn.jsParams.length + ' expected callback Function").ToLocalChecked());\n'
  cpp += '  }\n'
  cpp += '  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[' + fn.jsParams.length + ']).ToLocalChecked());\n'
  cpp += '  indyCalled(icb, ' + fn.name + '(icb->handle'
  cpp += fn.jsParams.map(function (arg, i) {
    if (arg.type === 'Buffer') {
      return ', arg' + i + 'data, arg' + i + 'len'
    }
    return ', arg' + i
  }).join('')
  cpp += ', ' + fn.jsName + '_cb));\n'

  fn.jsParams.forEach(function (arg, i) {
    var type = normalizeType(arg)
    switch (type) {
      case 'String':
        cpp += '  delete arg' + i + 'UTF;\n'
        break
      case 'Buffer':
      case 'IndyHandle':
      case 'indy_u32_t':
      case 'indy_i32_t':
      case 'indy_u64_t':
      case 'Timestamp':
      case 'Boolean':
        break
      default:
        throw new Error('Unhandled argument cleanup for type: ' + type)
    }
  })
  cpp += '}\n\n'
})

cpp += 'NAN_MODULE_INIT(InitAll) {\n'
apiFunctions.forEach(function (fn) {
  cpp += '  Nan::Export(target, "' + fn.jsName + '", ' + fn.jsName + ');\n'
})
cpp += '}\n'
cpp += 'NODE_MODULE(indynodejs, InitAll)\n'

fs.writeFileSync(OUT_FILE, cpp, 'utf8')
