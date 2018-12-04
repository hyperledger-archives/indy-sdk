package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.vcx.VcxApi;
import com.evernym.sdk.vcx.wallet.WalletApi;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

public class WalletApiTest {
    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }

    private String type = "test";
    private String id = "123";
    private String value = "record value";
    private String tags = "{'tagName1':'str1','tagName2':'5','tagName3':'12'}";

    @Test
    @DisplayName("create a record")
    void createRecord() throws VcxException, ExecutionException, InterruptedException {
        int recordHandle = TestHelper.getResultFromFuture(WalletApi.addRecordWallet(type,id,value));
        assert (recordHandle != 0);
    }

    @Test
    @DisplayName("get a record")
    void getRecord() throws VcxException, ExecutionException, InterruptedException {
        int recordHandle = TestHelper.getResultFromFuture(WalletApi.addRecordWallet(type,id,value));
        String recordValue = TestHelper.getResultFromFuture(WalletApi.getRecordWallet(type,id,""));
        assert (recordValue.contains(value));
    }

    @Test
    @DisplayName("update a record")
    void updateRecord() throws VcxException, ExecutionException, InterruptedException {
        int recordHandle = TestHelper.getResultFromFuture(WalletApi.addRecordWallet(type,id,value));
        int updatedRecordHandle = TestHelper.getResultFromFuture(WalletApi.updateRecordWallet(type,id,"new"));
        assert (updatedRecordHandle != 0);
    }

    @Test
    @DisplayName("delete a record")
    void deleteRecord() throws VcxException, ExecutionException, InterruptedException {
        int recordHandle = TestHelper.getResultFromFuture(WalletApi.addRecordWallet(type,id,value));
        int deleteRecordHandle = TestHelper.getResultFromFuture(WalletApi.deleteRecordWallet(type,id));
        assert (deleteRecordHandle != 0);
    }
}
