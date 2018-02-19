export const createFFICallbackPromise = <T>(fn, cb) => {
  let cbRef = null
  return (new Promise<T>( (resolve, reject) => fn(resolve, reject, cbRef = cb(resolve, reject))))
        .then((res) => {
          cbRef = null
          return res
        })
        .catch((err) => {
          cbRef = null
          throw err
        })
}
