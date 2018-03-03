var indy = require('bindings')('indy')
var errorCodes = require('./errorCodes.json')

module.exports = {}
module.exports.promise = {}

Object.keys(indy).forEach(function (name) {
  var nfn = indy[name]
  if (typeof nfn !== 'function') {
    return
  }
  var fn = function () {
    var args = Array.prototype.slice.call(arguments)
    var cb = args[args.length - 1]

    args[args.length - 1] = function (err) {
      if (err) {
        if (errorCodes['c' + err]) {
          cb(errorCodes['c' + err])
        } else {
          cb(err)
        }
        return
      }
      cb.apply(null, arguments)
    }

    nfn.apply(null, args)
  }

  module.exports[name] = fn

  module.exports.promise[name] = function () {
    var args = Array.prototype.slice.call(arguments)
    return new Promise(function (resolve, reject) {
      args.push(function (err, data) {
        if (err) reject(err)
        else resolve(data)
      })
      fn.apply(null, args)
    })
  }
})
