package com.evernym.sdk.vcx.credential;

public class GetCredentialCreateMsgidResult {
    public GetCredentialCreateMsgidResult(int credential_handle, String offer) {
        this.credential_handle = credential_handle;
        this.offer = offer;
    }

    public int getCredential_handle(){
        return credential_handle;
    }

    public void setCredential_handle(int credential_handle){
        this.credential_handle = credential_handle;
    }

    private int credential_handle;

    private String offer;

    public String getOffer(){
        return offer;
    }

    public void setOffer(String offer){
        this.offer = offer;
    }

}
