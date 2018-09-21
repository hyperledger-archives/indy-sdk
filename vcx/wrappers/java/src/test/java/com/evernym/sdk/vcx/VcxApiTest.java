package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.vcx.VcxApi;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

public class VcxApiTest {
    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }

    @Test
    @DisplayName("initialise vcx")
    void vcxInit() throws VcxException {
        assert (true); // Asserting true because the api is called and is tested in setup() function above above.
    }

    @Test
    @DisplayName("shut down and initialise vcx with a config")
    void vcxInitWithConfig() throws VcxException, ExecutionException, InterruptedException {
        //This unit test tests two apis vcxShutdown and vcxInitWithConfig
        int shutdownResult = VcxApi.vcxShutdown(false);
        assert (shutdownResult == 0);
        int result = TestHelper.getResultFromFuture(VcxApi.vcxInitWithConfig(TestHelper.VCX_CONFIG_TEST_MODE));
        assert (result == 0);
    }

    @Test
    @DisplayName("error message")
    void vcxErrorMessage() throws VcxException, ExecutionException, InterruptedException {
        String errorCMessage = VcxApi.vcxErrorCMessage(0);
        assert (errorCMessage.equals("Success"));
    }

    @Test
    @DisplayName("error message 1")
    void vcxUnknownErrorMessage() throws VcxException, ExecutionException, InterruptedException {
        String errorCMessage = VcxApi.vcxErrorCMessage(1001);
        assert (errorCMessage.equals("Unknown Error"));
    }
}
