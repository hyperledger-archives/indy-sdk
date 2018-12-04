package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.token.TokenApi;
import com.evernym.sdk.vcx.vcx.VcxApi;
import com.jayway.jsonpath.DocumentContext;
import com.jayway.jsonpath.JsonPath;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

public class TokenApiTest {
    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }
    String seed = "0000000000000000000WHATEVER00000";
    @Test
    @DisplayName("get token info")
    void getTokenInfo() throws VcxException, ExecutionException, InterruptedException {
        String tokenInfo = TestHelper.getResultFromFuture(TokenApi.getTokenInfo(0));
        DocumentContext jsonObject = JsonPath.parse(tokenInfo);
        int balance = jsonObject.read("$.balance");
        assert (balance != 0);
    }

    @Test
    @DisplayName("send tokens")
    void sendTokens() throws VcxException, ExecutionException, InterruptedException {
        String tokenInfo = TestHelper.getResultFromFuture(TokenApi.getTokenInfo(0));
        String receipt = TestHelper.getResultFromFuture(TokenApi.sendTokens(0,"1","address"));
    }

    @Test
    @DisplayName("create payment address ")
    void createPaymentAddress() throws VcxException, ExecutionException, InterruptedException {
        String paymentAddress = TestHelper.getResultFromFuture(TokenApi.createPaymentAddress(seed));
        assert(!paymentAddress.isEmpty());
    }
}
