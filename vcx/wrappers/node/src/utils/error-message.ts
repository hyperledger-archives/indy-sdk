import { VCXCode } from '../api/common'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'

const BufferSize = 256
export const errorMessage = (errorCode: number | Error, _bufferSize = BufferSize): string => {
  const buf = Buffer.alloc(_bufferSize)
  if (errorCode instanceof VCXInternalError) {
    return errorCode.message
  }
  if (errorCode instanceof Error) {
    const message = rustAPI().vcx_error_c_message(VCXCode.UNKNOWN_ERROR, buf, _bufferSize)
    return `${message}: ${errorCode.message}`
  }
  const size = rustAPI().vcx_error_c_message(errorCode, buf, _bufferSize)
  if (size > 0) {
    const terminatingNullPos = buf.indexOf('\u0000')
    let s = buf.toString()
    if (terminatingNullPos >= 0) { s = s.substr(0, size) }
    return s
  } else {
    return 'Internal Vcx Error: Invalid buffer size'
  }
}
