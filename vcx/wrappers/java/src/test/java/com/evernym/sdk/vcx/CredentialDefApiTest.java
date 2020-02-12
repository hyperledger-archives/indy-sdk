package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.credential.InvalidCredentialDefHandle;
import com.evernym.sdk.vcx.credentialDef.CredentialDefApi;
import com.evernym.sdk.vcx.credentialDef.CredentialDefPrepareForEndorserResult;
import com.evernym.sdk.vcx.vcx.VcxApi;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

public class CredentialDefApiTest {

    private CredentialDefPrepareForEndorserResult prepareCredDefForEndorser() throws VcxException, ExecutionException, InterruptedException {
        return TestHelper.getResultFromFuture(CredentialDefApi.credentialDefPrepareForEndorser(
                "testCredentialDefSourceId",
                "testCredentialDefName",
                "testCredentialDefSchemaId",
                null,
                "tag1",
                "{\"support_revocation\":false}",
                "V4SGRU86Z58d6TV7PBUe6f"
        ));
    }

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

    @Test
    @DisplayName("prepare a credentialdef for endorser")
    void prepareForEndorser() throws VcxException, ExecutionException, InterruptedException {
        CredentialDefPrepareForEndorserResult credentialdefForEndorser = prepareCredDefForEndorser();
        assert (credentialdefForEndorser.getCredentialDefHandle() != 0);
        assert (!credentialdefForEndorser.getCredDefTransaction().isEmpty());
        assert (credentialdefForEndorser.getRevocRegDefTransaction() == null);
        assert (credentialdefForEndorser.getRevocRegEntryTransaction() == null);
    }

    @Test
    @DisplayName("update schema state")
    void updateState() throws VcxException, ExecutionException, InterruptedException {
        CredentialDefPrepareForEndorserResult credentialdefForEndorser = prepareCredDefForEndorser();

        assert (credentialdefForEndorser.getCredentialDefHandle() != 0);
        assert (TestHelper.getResultFromFuture(CredentialDefApi.credentialDefGetState(credentialdefForEndorser.getCredentialDefHandle())) == 0);
        assert (TestHelper.getResultFromFuture(CredentialDefApi.credentialDefUpdateState(credentialdefForEndorser.getCredentialDefHandle())) == 1);
        assert (TestHelper.getResultFromFuture(CredentialDefApi.credentialDefGetState(credentialdefForEndorser.getCredentialDefHandle())) == 1);
    }




}
