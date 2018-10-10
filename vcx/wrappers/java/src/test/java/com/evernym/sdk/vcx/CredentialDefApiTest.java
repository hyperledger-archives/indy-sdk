package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.credential.InvalidCredentialDefHandle;
import com.evernym.sdk.vcx.credentialDef.CredentialDefApi;
import com.evernym.sdk.vcx.vcx.VcxApi;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

public class CredentialDefApiTest {

    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }
    @Test
    @DisplayName("create a credential def")
    void createCredentialDef() throws VcxException, ExecutionException, InterruptedException {
        int credentialDef = TestHelper._createCredentialDef();
        Assertions.assertNotEquals(0,credentialDef);
    }
    @Test
    @DisplayName("serialise a credential def")
    void serialize() throws VcxException, ExecutionException, InterruptedException {
        int credentialDef = TestHelper._createCredentialDef();
        String json = TestHelper.getResultFromFuture(CredentialDefApi.credentialDefSerialize(credentialDef));
        assert(json.contains("name"));
    }

    @Test
    @DisplayName("should throw invalid credentialdef handle exception when serializing invalid credentialdef")
    void serializeCredentialShouldThrow() {
        Assertions.assertThrows(InvalidCredentialDefHandle.class, () -> {
            TestHelper.getResultFromFuture(CredentialDefApi.credentialDefSerialize(0));
        });
    }

    @Test
    @DisplayName("deserialise a credential def")
    void deserialize() throws VcxException, ExecutionException, InterruptedException {
        int credentialDef = TestHelper._createCredentialDef();
        String json = TestHelper.getResultFromFuture(CredentialDefApi.credentialDefSerialize(credentialDef));
        assert(json.contains("name"));
        int deserialisedCredDef = TestHelper.getResultFromFuture(CredentialDefApi.credentialDefDeserialize(json));
        Assertions.assertNotEquals(0,deserialisedCredDef);
    }




}
