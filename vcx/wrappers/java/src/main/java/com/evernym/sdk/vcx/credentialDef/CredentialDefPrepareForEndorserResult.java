package com.evernym.sdk.vcx.credentialDef;

public class CredentialDefPrepareForEndorserResult {
	public CredentialDefPrepareForEndorserResult(int handle, String credentialDefTransaction,
	                                             String revocRegDefTransaction, String revocRegEntryTransaction) {
		this.credentialDefHandle = handle;
		this.credDefTransaction = credentialDefTransaction;
		this.revocRegDefTransaction = revocRegDefTransaction;
		this.revocRegEntryTransaction = revocRegEntryTransaction;
	}

	public int getCredentialDefHandle() {
		return credentialDefHandle;
	}

	public void setCredentialDefHandle(int handle) {
		this.credentialDefHandle = handle;
	}

	private int credentialDefHandle;

	private String credDefTransaction;

	private String revocRegDefTransaction;

	private String revocRegEntryTransaction;

	public String getCredDefTransaction() {
		return credDefTransaction;
	}

	public void setCredDefTransaction(String credDefTransaction) {
		this.credDefTransaction = credDefTransaction;
	}

	public String getRevocRegDefTransaction() {
		return revocRegDefTransaction;
	}

	public void setRevocRegDefTransaction(String revocRegDefTransaction) {
		this.revocRegDefTransaction = revocRegDefTransaction;
	}

	public String getRevocRegEntryTransaction() {
		return revocRegEntryTransaction;
	}

	public void setRevocRegEntryTransaction(String revocRegEntryTransaction) {
		this.revocRegEntryTransaction = revocRegEntryTransaction;
	}

}
