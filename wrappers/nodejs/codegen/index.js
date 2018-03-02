var fs = require('fs')
var path = require('path')

var OUT_FILE = path.resolve(__dirname, '../src/indy_codegen.h')

var hAST = require('./hAST.json')

var normalizeType = function (typeSrc) {
  typeSrc = typeSrc.replace(/[^a-z0-9_*]/ig, '')
  switch (typeSrc) {
    case 'constchar*':
    case 'constchar*const':
      return 'String'

    case 'indy_bool_t':
      return 'Boolean'

    case 'constindy_u8_t*':
      return 'ByteArray'

    case 'indy_u32_t':
    case 'indy_i32_t':
      return 'Number'

    case 'indy_handle_t':
      return 'IndyHandle'

    case 'indy_error_t':
      return 'IndyError'

    case 'void':
      return 'Void'
  }
  throw new Error('normalizeType doesn\'t handle: ' + typeSrc)
}

var exportFunctions = []
var cpp = ''

hAST.forEach(function (fn) {
  if (fn.name === 'indy_register_wallet_type') {
    return
  }
  if (fn.name !== 'indy_abbreviate_verkey') {
    return
  }
  if (fn.returnType !== 'indy_error_t') {
    throw new Error('Does not return an IndyError: ' + fn.name)
  }

  var jsName = fn.name.replace(/^indy_/, '')
  var jsArgs = []
  var jsCbArgs = []

  fn.args.forEach(function (arg, i) {
    if (i === 0) {
      if (arg.type !== 'indy_handle_t' || !/command_han.le$/.test(arg.name)) {
        throw new Error('Expected a command_handle as the first argument: ' + fn.name)
      }
      return
    }
    if (i === fn.args.length - 1) {
      if (arg.type !== 'Function') {
        throw new Error('Expected a callback as the as the last argument: ' + fn.name)
      }
      if (arg.args[0].type !== 'indy_handle_t' || !/command_handle$/.test(arg.args[0].name) || arg.args[1].type !== 'indy_error_t') {
        throw new Error('Callback doesn\'t have the standard handle + err: ' + fn.name)
      }
      arg.args.forEach(function (arg, i) {
        if (i > 1) {
          jsCbArgs.push(arg)
        }
      })
      return
    }
    jsArgs.push(arg)
  })

  var humanArgs = jsArgs.map(arg => arg.name)
  humanArgs.push('cb(err, ' + jsCbArgs.map(arg => arg.name) + ')')
  var humanDescription = jsName + '(' + humanArgs.join(', ') + ')'

  var cppReturnThrow = function (msg) {
    var errmsg = JSON.stringify(msg + ': ' + humanDescription)
    return '    return Nan::ThrowError(Nan::New(' + errmsg + ').ToLocalChecked());\n'
  }

  cpp += 'void ' + jsName + '_cb(indy_handle_t xcommand_handle, indy_error_t xerr, '
  cpp += jsCbArgs.map(arg => arg.type + ' ' + arg.name).join(', ')
  cpp += ') {\n'
  cpp += '  if(cbmap.count(xcommand_handle) == 0){\n'
  cpp += '    return;\n'
  cpp += '  }\n'
  cpp += '  IndyCallback* icb = cbmap[xcommand_handle];\n'
  cpp += '  icb->err = xerr;\n'
  cpp += '  if(icb->err == 0){\n'
  jsCbArgs.forEach(function (arg, i) {
    var type = normalizeType(arg.type)
    if (type === 'String') {
      cpp += '    icb->str' + i + ' = copyCStr(verkey);\n'
      return
    }
    throw new Error('Unhandled callback arg type: ' + type)
  })
  cpp += '  }\n'
  cpp += '  uv_async_send(icb->uvHandle);\n'
  cpp += '}\n'
  cpp += 'NAN_METHOD(' + jsName + ') {\n'
  cpp += '  if(info.Length() != ' + (jsArgs.length + 1) + '){\n'
  cpp += cppReturnThrow('Expected ' + (jsArgs.length + 1) + ' arguments')
  cpp += '  }\n'
  jsArgs.forEach(function (arg, i) {
    var type = normalizeType(arg.type)
    if (type === 'String') {
      cpp += '  if(!info[' + i + ']->IsString()){\n'
      cpp += cppReturnThrow('Expected ' + type + ' for arg ' + i)
      cpp += '  }\n'
      cpp += '  Nan::Utf8String arg' + i + 'UTF(info[' + i + ']);\n'
      cpp += '  const char* arg' + i + ' = (const char*)(*arg' + i + 'UTF);\n'
      return
    }
    throw new Error('Unhandled argument reading type: ' + type)
  })
  cpp += '  if(!info[' + jsArgs.length + ']->IsFunction()) {\n'
  cpp += '    return Nan::ThrowError(Nan::New("abbreviate_verkey arg ' + jsArgs.length + ' expected Function").ToLocalChecked());\n'
  cpp += '  }\n'
  cpp += '  Nan::Callback* callback = new Nan::Callback(Nan::To<v8::Function>(info[' + jsArgs.length + ']).ToLocalChecked());\n'
  cpp += '  indy_handle_t ch = getCommandHandle();\n'
  cpp += '  indyCalled(ch, callback, ' + fn.name + '(ch, arg0, arg1, ' + jsName + '_cb));\n'
  cpp += '}\n\n'

  exportFunctions.push(jsName)
})

cpp += 'NAN_MODULE_INIT(InitAll) {\n'
exportFunctions.forEach(function (fn) {
  cpp += '  Nan::Export(target, "' + fn + '", ' + fn + ');\n'
})
cpp += '}\n'
cpp += 'NODE_MODULE(indy, InitAll)\n'

fs.writeFileSync(OUT_FILE, cpp, 'utf8')
