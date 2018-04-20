var IndyError = require('./IndyError')

function wrapIndyCallback (cb, mapResponse) {
  var promise
  if (!cb) {
    promise = new Promise(function (resolve, reject) {
      cb = function cb (err, data) {
        if (err) reject(err)
        else resolve(data)
      }
    })
  }
  function callback (err, data) {
    if (err) {
      cb(new IndyError(err))
      return
    }
    if (mapResponse) {
      data = mapResponse(data)
    }
    cb(null, data)
  }
  callback.promise = promise
  return callback
}

module.exports = wrapIndyCallback
