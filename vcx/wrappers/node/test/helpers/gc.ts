import * as ffi from 'ffi'
import { VCXCode } from 'src'

export interface IGCTestData {
  handle: string,
  serialize: (commandId: number, handle: string, callback: Buffer) => number,
  stopCode: VCXCode
}
export const gcTest = async ({
  handle,
  serialize,
  stopCode
}: IGCTestData) => {
  global.gc()
  let isComplete = false

  //  hold on to callbacks so they don't become garbage collected
  const callbacks: any[] = []
  while (!isComplete) {
    const data = await new Promise<string>((resolve, reject) => {
      const callback = ffi.Callback(
        'void',
        ['uint32', 'uint32', 'string'],
        (handleCb: number, errCb: number, dataCb: string) => {
          if (errCb) {
            reject(errCb)
            return
          }
          resolve(dataCb)
        }
      )
      callbacks.push(callback)
      const rc = serialize(
        0,
        handle,
        callback
      )
      if (rc === stopCode) {
        resolve('')
      }
    })
    if (!data) {
      isComplete = true
    }
  }

  // this will timeout if condition is never met
  // get_data will return "" because the connection object was released
  return isComplete
}
