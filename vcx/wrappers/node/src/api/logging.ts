import * as ref from 'ref'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'

export class Logger {

/**
 * Sets the Logger to Default
 *
 * Example:
 * ```
 * logger = Logger.setDefaultLogger('info')
 * ```
 *
 */

  public static setDefaultLogger (level: string) {
    try {
      rustAPI().vcx_set_default_logger(level)
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   *
   * Set the Logger to A Custom Logger
   *
   * Example:
   * ```
   * const logFn = ffi.Callback('void', ['pointer', 'uint32', 'string', 'string', 'string', 'string', 'uint32'],
   * (_context: any, level: number, target: string, message: string , modulePath: string,
   *     file: string, line: number) => {
   *  assert(typeof level, 'number')
   *  assert(typeof target, 'string')
   *  assert(typeof message, 'string')
   *  assert(typeof modulePath, 'string')
   *  assert(typeof file, 'string')
   *  assert(typeof line, 'number')
   * })
   * Logger.setLogger(logFn)
   * ```
   *
   */
  public static setLogger (logFn: Buffer) {
    try {
      rustAPI().vcx_set_logger(ref.NULL_POINTER, ref.NULL , logFn , ref.NULL_POINTER)
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }
}
