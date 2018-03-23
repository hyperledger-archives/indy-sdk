var fs = require('fs')
var path = require('path')
var apiFunctions = require('./apiFunctions')

var README_FILE = path.resolve(__dirname, '../README.md')

var toHumanType = function (param) {
  if (param.json) {
    return 'Json'
  }
  switch (param.type.replace(/[^a-z0-9_*]/ig, '')) {
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
  throw new Error('toHumanType doesn\'t handle: ' + param.type)
}

var readmeParam = function (param) {
  return '`' + param.name + '`: ' + toHumanType(param)
}

var readme = ''
apiFunctions.forEach(function (fn) {
  var signature = fn.jsName + '(' + fn.jsParams.map(arg => arg.name).join(', ') + ')' + ' -> ' + fn.humanReturnValue

  readme += '#### ' + signature.replace(/_/g, '\\_') + '\n'

  fn.jsParams.forEach(function (arg) {
    readme += '* ' + readmeParam(arg) + '\n'
  })
  if (fn.jsCbParams.length === 1) {
    readme += '* __->__ ' + readmeParam(fn.jsCbParams[0]) + '\n'
  } else if (fn.jsCbParams.length > 1) {
    readme += '* __->__ [' + fn.jsCbParams.map(readmeParam).join(', ') + ']\n'
  }
  readme += '\n'
})

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
