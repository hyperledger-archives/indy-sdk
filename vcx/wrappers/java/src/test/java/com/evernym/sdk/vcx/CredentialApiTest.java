package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.evernym.sdk.vcx.credential.CredentialApi;
import com.evernym.sdk.vcx.credential.GetCredentialCreateMsgidResult;
import com.evernym.sdk.vcx.credential.InvalidCredentialHandleException;
import com.evernym.sdk.vcx.vcx.InvalidOptionException;
import com.evernym.sdk.vcx.vcx.VcxApi;
import com.jayway.jsonpath.JsonPath;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.concurrent.ExecutionException;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotSame;


class CredentialApiTest {
    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }

    @Test
    @DisplayName("create a credential with a offer")
    void createCredential() throws VcxException, ExecutionException, InterruptedException {
        int credential = TestHelper._createCredential();
        assertNotSame(0, credential);
    }

    @Test
    @DisplayName("create a credential with invalid offer")
    void createCredentialWithInvalidOffer() {
        Assertions.assertThrows(InvalidOptionException.class, () -> {
            TestHelper.getResultFromFuture(CredentialApi.credentialCreateWithOffer("1", ""));
        });

    }

    @Test
    @DisplayName("create a credential with message id")
    void createCredentialWithMsgId() throws VcxException, ExecutionException, InterruptedException {
        int connection = TestHelper._createConnection();
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionConnect(connection,TestHelper.convertToValidJson(payload)));
        GetCredentialCreateMsgidResult result = TestHelper.getResultFromFuture(CredentialApi.credentialCreateWithMsgid("1", connection, "1"));
        assertNotSame(0, result.getCredential_handle());

        assert (!result.getOffer().isEmpty());
        int status = TestHelper.getResultFromFuture(CredentialApi.credentialGetState(result.getCredential_handle()));
        assertEquals(3, status);
    }

    @Test
    @DisplayName("serialize credential")
    void serializeCredential() throws InterruptedException, VcxException, ExecutionException {
        int credential = TestHelper._createCredential();
        assertNotSame(0, credential);
        String serializedCredential = TestHelper.getResultFromFuture(CredentialApi.credentialSerialize(credential));
        assert (serializedCredential.contains(TestHelper.address1InOffer));

    }

    @Test
    @DisplayName("should throw invalid credential handle exception when serializing invalid credential")
    void serializeCredentialShouldThrow() {
        Assertions.assertThrows(InvalidCredentialHandleException.class, () -> {
            TestHelper.getResultFromFuture(CredentialApi.credentialSerialize(0));
        });
    }

    @Test
    @DisplayName("deserialize credential")
    void deserializeCredential() throws InterruptedException, VcxException, ExecutionException {
        int credential = TestHelper._createCredential();
        assertNotSame(0, credential);
        String serializedCredential = TestHelper.getResultFromFuture(CredentialApi.credentialSerialize(credential));
        assert (serializedCredential.contains(TestHelper.address1InOffer));
        int credentialHandle = TestHelper.getResultFromFuture(CredentialApi.credentialDeserialize(serializedCredential));
        assert (credentialHandle != 0);

    }

    @Test
    @DisplayName("should throw invalid credential handle when deserializing invalid credential")
    void deserializeCredentialShouldTrow() {
        Assertions.assertThrows(InvalidOptionException.class, () -> {
            TestHelper.getResultFromFuture(CredentialApi.credentialDeserialize(""));
        });


    }

    @Test
    @DisplayName("update state of credential")
    void updateState() throws VcxException, ExecutionException, InterruptedException {
        int credential = TestHelper._createCredential();
        assertNotSame(0, credential);
        TestHelper.getResultFromFuture(CredentialApi.credentialUpdateState(credential));
        int state = TestHelper.getResultFromFuture(CredentialApi.credentialGetState(credential));
        assertEquals(3, state);
    }

    @Test
    @DisplayName("send credential request")
    void sendRequest() throws VcxException, ExecutionException, InterruptedException {
        int credential = TestHelper._createCredential();
        int connection = TestHelper._createConnection();
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionConnect(connection,TestHelper.convertToValidJson(payload)));
        assertNotSame(0, credential);
        TestHelper.getResultFromFuture(CredentialApi.credentialSendRequest(credential, connection, 0));
        int state = TestHelper.getResultFromFuture(CredentialApi.credentialGetState(credential));
        assertEquals(2, state);

    }

    @Test
    @DisplayName("send credential request with message id")
    void sendRequestWithMsgId() throws VcxException, ExecutionException, InterruptedException {
        int connection = TestHelper._createConnection();
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionConnect(connection,TestHelper.convertToValidJson(payload)));
        GetCredentialCreateMsgidResult credential = TestHelper.getResultFromFuture(CredentialApi.credentialCreateWithMsgid("1", connection, "1"));
        assertNotSame(0, credential);
        TestHelper.getResultFromFuture(CredentialApi.credentialSendRequest(credential.getCredential_handle(), connection, 0));
        int state = TestHelper.getResultFromFuture(CredentialApi.credentialGetState(credential.getCredential_handle()));
        assertEquals(2, state);

    }

    @Test
    @DisplayName("get credential offers for a connection")
    void getOffers() throws VcxException, ExecutionException, InterruptedException {
        int connection = TestHelper._createConnection();
	String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionConnect(connection,TestHelper.convertToValidJson(payload)));
        String offers = TestHelper.getResultFromFuture(CredentialApi.credentialGetOffers(connection));
        assert (offers != null);
        List<String> credentialOffer = JsonPath.read(offers,"$.[0]");
        assert (!credentialOffer.isEmpty());
        int credential = TestHelper.getResultFromFuture(CredentialApi.credentialCreateWithOffer("0", JsonPath.parse(credentialOffer).jsonString()));
        assertNotSame(0, credential);
    }


}
