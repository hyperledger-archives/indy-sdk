import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'

export class Logger {

/**
 * Sets the Logger to Default
 *
 * Example:
 * ```
 * logger = Logger.setDefaultLogger({ level: 'info' })
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
}
