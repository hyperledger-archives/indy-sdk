import * as ffi from 'ffi'
import * as ref from 'ref'
import * as Struct from 'ref-struct'
import * as weak from 'weak'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig, CxsStatus, FFI_CXS_STATUS_PTR } from '../rustlib'

import {
    IConnections
} from './api'

export class Connection implements IConnections {
  public connectionHandle: ref.types.uint32
  public state: ref.types.uint32
  public statusList: any
  private RUST_API: ffi

  constructor ( path?: string ) {
    this.initRustApi(path)
  }

  create ( recipientInfo: string ): number {
    const connectionHandlePtr = ref.alloc(ref.types.uint32)
    const result = this.RUST_API.cxs_connection_create(recipientInfo, connectionHandlePtr)
    this.connectionHandle = ref.deref(connectionHandlePtr, ref.types.uint32)
    this.clearOnExit()

    return result
  }

  connect (): number {
    return this.RUST_API.cxs_connection_connect(this.connectionHandle)
  }

  getData (): string {
    return this.RUST_API.cxs_connection_get_data(this.connectionHandle)
  }

  getState (): number {
    const statusPtr = ref.alloc(ref.types.uint32)
    const result = this.RUST_API.cxs_connection_get_state(this.connectionHandle, statusPtr)
    this.state = ref.deref(statusPtr, ref.types.uint32)
    return result
  }

  release (): number {
    return this.RUST_API.cxs_connection_release(this.connectionHandle)
  }

  private initRustApi (path?) {
    this.RUST_API = new CXSRuntime(new CXSRuntimeConfig(path)).ffi
  }

  // clearOnExit creates a callback that will release the Rust Object
  // when the node Connection object is Garbage collected
  private clearOnExit () {
    const weakRef = weak(this)
    const release = this.RUST_API.cxs_connection_release
    const handle = this.connectionHandle
    weak.addCallback(weakRef, () => {
      release(handle)
    })
  }
}
