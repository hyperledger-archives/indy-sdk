var api = require('./api.json')

var fixBufferParams = function (params) {
  var out = []
  var i = 0
  while (i < params.length) {
    if (params[i].type.replace(/[^a-z0-9_*]/ig, '') === 'constindy_u8_t*') {
      if (params[i + 1].type !== 'indy_u32_t' && /_len$/.test(params[i + 1].name)) {
        throw new Error('Expected buffer _len next')
      }
      out.push({
        name: params[i].name,
        type: 'Buffer'
      })
      i++
    } else {
      out.push(params[i])
    }
    i++
  }
  return out
}

var functions = []

Object.keys(api.functions).forEach(function (name) {
  if (name === 'indy_register_wallet_type') {
    return
  }

  var fn = {
    name: name,
    params: api.functions[name].params,
    ret: api.functions[name].ret,
    jsName: name.replace(/^indy_/, ''),
    jsParams: [],
    jsCbParams: []
  }

  if (fn.ret !== 'indy_error_t') {
    throw new Error('Does not return an IndyError: ' + fn.name)
  }

  fn.params.forEach(function (param, i) {
    if (i === 0) {
      if (param.type !== 'indy_handle_t' || !/command_han.le$/.test(param.name)) {
        throw new Error('Expected a command_handle as the first argument: ' + fn.name)
      }
      return
    }
    if (i === fn.params.length - 1) {
      if (!param.hasOwnProperty('params')) {
        throw new Error('Expected a callback as the as the last argument: ' + fn.name)
      }
      if (param.params[0].type !== 'indy_handle_t' || !/command_handle$/.test(param.params[0].name) || param.params[1].type !== 'indy_error_t') {
        throw new Error('Callback doesn\'t have the standard handle + err: ' + fn.name)
      }
      param.params.forEach(function (param, i) {
        if (i > 1) {
          fn.jsCbParams.push(param)
        }
      })
      return
    }
    fn.jsParams.push(param)
  })
  fn.jsParams = fixBufferParams(fn.jsParams)
  fn.jsCbParams = fixBufferParams(fn.jsCbParams)

  var humanArgs = fn.jsParams.map(arg => arg.name)
  var humanCb = 'cb(err'
  if (fn.jsCbParams.length === 1) {
    humanCb += ', ' + fn.jsCbParams[0].name
  } else if (fn.jsCbParams.length > 1) {
    humanCb += ', [' + fn.jsCbParams.map(arg => arg.name).join(', ') + ']'
  }
  humanCb += ')'
  humanArgs.push(humanCb)
  fn.humanSignature = fn.jsName + '(' + humanArgs.join(', ') + ')'

  functions.push(fn)
})

module.exports = functions
