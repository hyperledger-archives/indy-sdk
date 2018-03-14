const maxTimeout = 2147483647

export const createFFICallbackPromise = <T>(fn, cb) => {
  let cbRef = null
  // TODO: Research why registering a callback doesn't keep parent thread alive https://github.com/node-ffi/node-ffi
  const processKeepAliveTimer = setTimeout(() => undefined, maxTimeout)
  return (new Promise<T>( (resolve, reject) => fn(resolve, reject, cbRef = cb(resolve, reject))))
        .then((res) => {
          cbRef = null
          clearTimeout(processKeepAliveTimer)
          return res
        })
        .catch((err) => {
          cbRef = null
          clearTimeout(processKeepAliveTimer)
          throw err
        })
}
