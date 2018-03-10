var binding = require('bindings')('indy')
var api = require('./api.json')
var apiFunctions = require('./apiFunctions')

function IndyError (err) {
  Error.call(this)
  Error.captureStackTrace(this, this.constructor)
  this.name = this.constructor.name
  if (api.errors['c' + err]) {
    this.message = api.errors['c' + err]
    this.indy_code = err
    this.indy_name = api.errors['c' + err]
  } else {
    this.message = (err + '')
  }
}
require('util').inherits(IndyError, Error)

var indy = {}

apiFunctions.forEach(function (fn) {
  var nParams = fn.jsParams.length + 1

  indy[fn.jsName] = function () {
    var args = Array.prototype.slice.call(arguments)
    while (args.length < nParams) {
      args.push(null)
    }
    args = args.slice(0, nParams)

    var callback = args[nParams - 1]
    var promise
    if (typeof callback !== 'function') {
      promise = new Promise(function (resolve, reject) {
        callback = function callback (err, value) {
          if (err) reject(err)
          else resolve(value)
        }
      })
    }

    args[nParams - 1] = function (err, data) {
      if (err) callback(new IndyError(err))
      else callback(null, data)
    }
    binding[fn.jsName].apply(null, args)

    return promise
  }
})

module.exports = indy
