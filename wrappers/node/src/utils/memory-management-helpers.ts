import * as weak from 'weak'
import { VCXInternalError } from '../errors'

export abstract class GCWatcher {
  protected abstract _releaseFn: any
  private _handleRef: string | null = null

  public async release (): Promise<number> {
    try {
      const rc = this._releaseFn(this._handleRef)
      if (rc) {
        throw rc
      }
      return rc
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  // Can not use setter because of https://github.com/Microsoft/TypeScript/issues/2521
  protected _setHandle (handle: string): void {
    this._handleRef = handle
    this._clearOnExit()
  }

  // _clearOnExit creates a callback that will release the Rust Object
  // when the node Connection object is Garbage collected
  protected _clearOnExit () {
    const weakRef = weak(this)
    const release = this._releaseFn
    const handle = this._handleRef
    weak.addCallback(weakRef, () => {
      try {
        const rc = release(handle)
        if (rc) {
          throw rc
        }
      } catch (err) {
        throw new VCXInternalError(err)
      }
    })
  }

  get handle () {
    // LibVCX handles invalid handles
    return this._handleRef as string
  }
}
