var indy = require('bindings')('indy')
var errorCodes = require('./errorCodes.json')

function IndyError (err) {
  Error.call(this)
  Error.captureStackTrace(this, this.constructor)
  this.name = this.constructor.name
  if (errorCodes['c' + err]) {
    this.message = errorCodes['c' + err]
    this.indy_code = err
    this.indy_name = errorCodes['c' + err]
  } else {
    this.message = (err + '')
  }
}
require('util').inherits(IndyError, Error)

module.exports = {}
module.exports.promise = {}

Object.keys(indy).forEach(function (name) {
  var fn = indy[name]
  if (typeof fn !== 'function') {
    return
  }
  module.exports[name] = function () {
    var args = Array.prototype.slice.call(arguments)
    var cb = args[args.length - 1]
    args[args.length - 1] = function (err, data) {
      if (err) {
        cb(new IndyError(err))
        return
      }
      cb(null, data)
    }
    fn.apply(null, args)
  }

  module.exports.promise[name] = function () {
    var args = Array.prototype.slice.call(arguments)
    return new Promise(function (resolve, reject) {
      args.push(function (err, data) {
        if (err) reject(new IndyError(err))
        else resolve(data)
      })
      fn.apply(null, args)
    })
  }
})
