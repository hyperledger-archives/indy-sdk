var fs = require('fs')
var path = require('path')

var OUT_FILE = path.resolve(__dirname, '../src/indy_codegen.h')
var README_FILE = path.resolve(__dirname, '../README.md')

var hAST = require('./hParser')

var normalizeType = function (typeSrc) {
  switch (typeSrc.replace(/[^a-z0-9_*]/ig, '')) {
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
      return typeSrc

    case 'Buffer':
      return 'Buffer'
  }
  throw new Error('normalizeType doesn\'t handle: ' + typeSrc)
}

var toHumanType = function (typeSrc) {
  switch (typeSrc.replace(/[^a-z0-9_*]/ig, '')) {
    case 'constchar*':
    case 'constchar*const':
      return 'String'

    case 'indy_bool_t':
      return 'Boolean'

    case 'indy_error_t':
      return 'IndyError'

    case 'indy_handle_t':
    case 'indy_u32_t':
    case 'indy_i32_t':
      return 'Number'

    case 'Buffer':
      return 'Buffer'
  }
  throw new Error('toHumanType doesn\'t handle: ' + typeSrc)
}

var fixBufferArgs = function (args) {
  var out = []
  var i = 0
  while (i < args.length) {
    if (args[i].type.replace(/[^a-z0-9_*]/ig, '') === 'constindy_u8_t*') {
      if (args[i + 1].type !== 'indy_u32_t' && /_len$/.test(args[i + 1].name)) {
        throw new Error('Expected buffer _len next')
      }
      out.push({
        name: args[i].name,
        type: 'Buffer'
      })
      i++
    } else {
      out.push(args[i])
    }
    i++
  }
  return out
}

var exportFunctions = []
var cpp = ''
var readme = ''

hAST.forEach(function (fn) {
  if (fn.name === 'indy_register_wallet_type') {
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

  jsArgs = fixBufferArgs(jsArgs)
  jsCbArgs = fixBufferArgs(jsCbArgs)

  var humanArgs = jsArgs.map(arg => arg.name)
  var humanCb = 'cb(err'
  if (jsCbArgs.length === 1) {
    humanCb += ', ' + jsCbArgs[0].name
  } else if (jsCbArgs.length > 1) {
    humanCb += ', [' + jsCbArgs.map(arg => arg.name).join(', ') + ']'
  }
  humanCb += ')'
  humanArgs.push(humanCb)
  var humanDescription = jsName + '(' + humanArgs.join(', ') + ')'

  readme += '#### ' + humanDescription.replace(/_/g, '\\_') + '\n'
  var readmeArg = function (arg) {
    return '`' + arg.name + '`: ' + toHumanType(arg.type)
  }
  jsArgs.forEach(function (arg) {
    readme += '* ' + readmeArg(arg) + '\n'
  })
  if (jsCbArgs.length === 1) {
    readme += '* __->__ ' + readmeArg(jsCbArgs[0]) + '\n'
  } else if (jsCbArgs.length > 1) {
    readme += '* __->__ [' + jsCbArgs.map(readmeArg).join(', ') + ']\n'
  }
  readme += '\n'

  var cppReturnThrow = function (msg) {
    var errmsg = JSON.stringify(msg + ': ' + humanDescription)
    return '    return Nan::ThrowError(Nan::New(' + errmsg + ').ToLocalChecked());\n'
  }

  cpp += 'void ' + jsName + '_cb(indy_handle_t xcommand_handle, indy_error_t xerr'
  cpp += jsCbArgs.map(function (arg, i) {
    if (arg.type === 'Buffer') {
      return ', const indy_u8_t* arg' + i + 'data, indy_u32_t arg' + i + 'size'
    }
    return ', ' + arg.type + ' arg' + i
  }).join('')
  cpp += ') {\n'
  cpp += '  if(cbmap.count(xcommand_handle) == 0){\n'
  cpp += '    return;\n'
  cpp += '  }\n'
  cpp += '  IndyCallback* icb = cbmap[xcommand_handle];\n'
  cpp += '  icb->err = xerr;\n'
  cpp += '  if(icb->err == 0){\n'

  var cbArgTypes = jsCbArgs.map(arg => normalizeType(arg.type)).join('+')
  switch (cbArgTypes) {
    case '':
      cpp += '    // none\n'
      break
    case 'String':
      cpp += '    icb->type = CB_STRING;\n'
      cpp += '    icb->str0 = copyCStr(arg0);\n'
      break
    case 'Boolean':
      cpp += '    icb->type = CB_BOOLEAN;\n'
      cpp += '    icb->bool0 = arg0;\n'
      break
    case 'IndyHandle':
      cpp += '    icb->type = CB_HANDLE;\n'
      cpp += '    icb->handle0 = arg0;\n'
      break
    case 'String+String':
      cpp += '    icb->type = CB_STRING_STRING;\n'
      cpp += '    icb->str0 = copyCStr(arg0);\n'
      cpp += '    icb->str1 = copyCStr(arg1);\n'
      break
    case 'Buffer':
      cpp += '    icb->type = CB_BUFFER;\n'
      cpp += '    icb->buffer0data = (char*)arg0data;\n'
      cpp += '    icb->buffer0size = arg0size;\n'
      break
    case 'String+Buffer':
      cpp += '    icb->type = CB_STRING_BUFFER;\n'
      cpp += '    icb->str0 = copyCStr(arg0);\n'
      cpp += '    icb->buffer0data = (char*)arg1data;\n'
      cpp += '    icb->buffer0size = arg1size;\n'
      break
    default:
      throw new Error('Unhandled callback args type: ' + cbArgTypes)
  }
  cpp += '  }\n'
  cpp += '  uv_async_send(icb->uvHandle);\n'
  cpp += '}\n'
  cpp += 'NAN_METHOD(' + jsName + ') {\n'
  cpp += '  if(info.Length() != ' + (jsArgs.length + 1) + '){\n'
  cpp += cppReturnThrow('Expected ' + (jsArgs.length + 1) + ' arguments')
  cpp += '  }\n'
  jsArgs.forEach(function (arg, i) {
    var type = normalizeType(arg.type)

    var chkType = function (isfn) {
      cpp += '  if(!info[' + i + ']->' + isfn + '()){\n'
      cpp += cppReturnThrow('Expected ' + type + ' for arg ' + i)
      cpp += '  }\n'
    }

    switch (type) {
      case 'String':
        chkType('IsString')
        cpp += '  Nan::Utf8String arg' + i + 'UTF(info[' + i + ']);\n'
        cpp += '  const char* arg' + i + ' = (const char*)(*arg' + i + 'UTF);\n'
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
      case 'Boolean':
        chkType('IsBoolean')
        cpp += '  indy_bool_t arg' + i + ' = info[' + i + ']->IsTrue();\n'
        break
      case 'Buffer':
        chkType('IsUint8Array')
        cpp += '  const indy_u8_t* arg' + i + 'data = (indy_u8_t*)node::Buffer::Data(info[' + i + ']->ToObject());\n'
        cpp += '  indy_u32_t arg' + i + 'size = node::Buffer::Length(info[' + i + ']);\n'
        break
      default:
        throw new Error('Unhandled argument reading type: ' + type)
    }
  })
  cpp += '  if(!info[' + jsArgs.length + ']->IsFunction()) {\n'
  cpp += '    return Nan::ThrowError(Nan::New("abbreviate_verkey arg ' + jsArgs.length + ' expected Function").ToLocalChecked());\n'
  cpp += '  }\n'
  cpp += '  Nan::Callback* callback = new Nan::Callback(Nan::To<v8::Function>(info[' + jsArgs.length + ']).ToLocalChecked());\n'
  cpp += '  indy_handle_t ch = getCommandHandle();\n'
  cpp += '  indyCalled(ch, callback, ' + fn.name + '(ch'
  cpp += jsArgs.map(function (arg, i) {
    if (arg.type === 'Buffer') {
      return ', arg' + i + 'data, arg' + i + 'size'
    }
    return ', arg' + i
  }).join('')
  cpp += ', ' + jsName + '_cb));\n'
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

var readmeOut = []
var inBlock = false
fs.readFileSync(README_FILE, 'utf8').split('\n').forEach(function (line) {
  if (/CODEGEN-START/.test(line)) {
    readmeOut.push(line)
    readmeOut.push(readme)
    inBlock = true
  }
  if (/CODEGEN-END/.test(line)) {
    inBlock = false
  }
  if (!inBlock) {
    readmeOut.push(line)
  }
})

fs.writeFileSync(README_FILE, readmeOut.join('\n'), 'utf8')
