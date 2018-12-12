package com.evernym.sdk.vcx.proof;

/**
 * Created by abdussami on 05/06/18.
 */

public class GetProofResult {
    public GetProofResult(
            int proof_state,
            String response_data) {
        this.proof_state = proof_state;
        this.response_data = response_data;
    }

    public int getProof_state() {
        return proof_state;
    }

    public void setProof_state(int proof_state) {
        this.proof_state = proof_state;
    }

    private int proof_state;

    private String response_data;

    public String getResponse_data() {
        return response_data;
    }

    public void setResponse_data(String response_data) {
        this.response_data = response_data;
    }
}
