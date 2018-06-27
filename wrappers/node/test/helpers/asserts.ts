import { assert } from 'chai'
import { IPaymentTxn, IUTXO } from 'src'

export const validateUTXO = (utxo: IUTXO) => {
  assert.equal(typeof utxo, 'object')
  assert.property(utxo, 'paymentAddress')
  assert.equal(typeof utxo.paymentAddress, 'string')
  assert.property(utxo, 'amount')
  assert.equal(typeof utxo.amount, 'number')
  if (utxo.extra) {
    assert.equal(typeof utxo.extra, 'string')
  }
  if (utxo.txo) {
    assert.equal(typeof utxo.txo, 'string')
  }
  return utxo
}

export const validatePaymentTxn = (paymentTxn: IPaymentTxn) => {
  assert.equal(typeof paymentTxn, 'object')
  assert.property(paymentTxn, 'amount')
  assert.equal(typeof paymentTxn.amount, 'number')
  assert.property(paymentTxn, 'inputs')
  assert.ok(Array.isArray(paymentTxn.inputs))
  for (const input of paymentTxn.inputs) {
    assert.ok(input)
    assert.equal(typeof input, 'string')
  }
  assert.property(paymentTxn, 'outputs')
  assert.ok(Array.isArray(paymentTxn.outputs))
  for (const utxo of paymentTxn.outputs) {
    validateUTXO(utxo)
  }
  return paymentTxn
}
