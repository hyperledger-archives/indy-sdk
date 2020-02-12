package com.evernym.sdk.vcx.schema;

public class SchemaPrepareForEndorserResult {
    public SchemaPrepareForEndorserResult(int handle, String transaction) {
        this.schema_handle = handle;
        this.transaction = transaction;
    }

    public int getSchemaHandle(){
        return schema_handle;
    }

    public void setSchemaHandle(int handle){
        this.schema_handle = handle;
    }

    private int schema_handle;

    private String transaction;

    public String getTransaction(){
        return transaction;
    }

    public void setTransaction(String transaction){
        this.transaction = transaction;
    }

}
