import * as weak from 'weak'

export abstract class GCWatcher {
  protected abstract _handle: string
  protected abstract _releaseFn: any

  async release (): Promise<number> {
    return this._releaseFn(this._handle)
  }

  protected _setHandle (handle: string): void {
    this._handle = handle
    this._clearOnExit()
  }

  // _clearOnExit creates a callback that will release the Rust Object
  // when the node Connection object is Garbage collected
  protected _clearOnExit () {
    const weakRef = weak(this)
    const release = this._releaseFn
    const handle = this._handle
    weak.addCallback(weakRef, () => release(handle))
  }
}
