import * as weak from 'weak'

export abstract class GCWatcher {
  protected abstract _releaseFn: any
  private _handleRef: string | null = null

  async release (): Promise<number> {
    return this._releaseFn(this._handleRef)
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
    weak.addCallback(weakRef, () => release(handle))
  }

  get handle () {
    // LibVCX handles invalid handles
    return this._handleRef as string
  }
}
