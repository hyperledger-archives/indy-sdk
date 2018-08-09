package com.evernym.sdk.vcx.proof;

public class CreateProofMsgIdResult {
    public int proofHandle;
    public String proofRequest;

    public CreateProofMsgIdResult(int proofHandle, String proofRequest) {
        this.proofHandle = proofHandle;
        this.proofRequest = proofRequest;
    }
}
