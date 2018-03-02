module.exports = require('bindings')('indy')

module.exports.promise = {}
Object.keys(module.exports).forEach(function (name) {
  var fn = module.exports[name]
  if (typeof fn !== 'function') {
    return
  }
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
