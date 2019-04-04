package com.evernym.sdk.vcx;


import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.evernym.sdk.vcx.connection.InvalidConnectionHandleException;
import com.evernym.sdk.vcx.vcx.VcxApi;
import java.util.concurrent.CompletableFuture;
import org.awaitility.Awaitility;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

import static com.evernym.sdk.vcx.TestHelper._createConnection;
import static org.junit.jupiter.api.Assertions.assertNotSame;

class ConnectionApiTest {

    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }


    @Test
    @DisplayName("create a connection")
    void createConnection() throws VcxException {

        long connectionHandle = _createConnection();
        assertNotSame(null,connectionHandle);
        assertNotSame( 0,connectionHandle);
    }

    @Test
    @DisplayName("connect connection without phone number")
    void connectConnectionWithoutPhone() throws VcxException {
        String payload= "{ 'connection_type': 'SMS' }";
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(future::isDone);
        assertNotSame("",future.getNow(""));
    }

    @Test
    @DisplayName("connect connection with phone number")
    void connectConnectionWithPhone() throws VcxException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(future::isDone);
        assertNotSame("",future.getNow(""));


    }

    @Test
    @DisplayName("throw invalid connection handle exception for wrong handle")
    void throwInvalidConnectionHandleException() {

        Assertions.assertThrows(InvalidConnectionHandleException.class, ()-> {
            String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
            CompletableFuture<String> future = ConnectionApi.vcxConnectionConnect(8765,TestHelper.convertToValidJson(payload));
            Awaitility.await().until(future::isDone);
            assertNotSame("",future.getNow(""));
        });


    }

    @Test
    @DisplayName("serialize a connection")
    void serializeConnection() throws VcxException {
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.connectionSerialize(connectionHandle);
        Awaitility.await().until(future::isDone);
        String serializedJson = future.getNow("");
        System.out.println(serializedJson);
        assertNotSame("",serializedJson);
        assert(serializedJson.contains("version"));
        assert(serializedJson.contains("data"));
    }

    @Test
    @DisplayName("throw invalid connection handle exception for serializing invalid connection ")
    void serializeConnectionWithBadHandle() {
        Assertions.assertThrows(InvalidConnectionHandleException.class, ()-> {
            CompletableFuture<String> future = ConnectionApi.connectionSerialize(0);
            Awaitility.await().until(future::isDone);
        });

    }

    @Test
    @DisplayName("delete a connection")
    void deleteConnection() throws VcxException, ExecutionException, InterruptedException {
        Integer connectionHandle = _createConnection();
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        TestHelper.getResultFromFuture(ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload)));
        CompletableFuture<Integer> futureDelete= ConnectionApi.deleteConnection(connectionHandle);
        Awaitility.await().until(futureDelete::isDone);
        assert(futureDelete.get() == 0);
    }

    @Test
    @DisplayName("throw invalid connection handle exception if trying to serialize deleted connection ")
    void serlializeDeletedConnection() {

        Assertions.assertThrows(InvalidConnectionHandleException.class, ()-> {
            Integer connectionHandle = _createConnection();
            CompletableFuture<Integer> futureDelete= ConnectionApi.deleteConnection(connectionHandle);
            Awaitility.await().until(futureDelete::isDone);
            CompletableFuture<String> future = ConnectionApi.connectionSerialize(connectionHandle);
            Awaitility.await().until(future::isDone);
        });

    }

    @Test
    @DisplayName("throw invalid connection handle exception if trying to serialize released connection")
    void serlializeReleasedConnection() {
        Assertions.assertThrows(InvalidConnectionHandleException.class, ()-> {
            Integer connectionHandle = _createConnection();
            int releaseResult= ConnectionApi.connectionRelease(connectionHandle);
            assert(releaseResult == 0 );
            CompletableFuture<String> future = ConnectionApi.connectionSerialize(connectionHandle);
            Awaitility.await().until(future::isDone);
        });
    }

    @Test
    @DisplayName("release a connection")
    void releaseConnection() throws VcxException {
        Integer connectionHandle = _createConnection();
        int result= ConnectionApi.connectionRelease(connectionHandle);
        assert(result == 0 );
    }

    @Test
    @DisplayName("initialise a connction")
    void initialiseConnection() throws VcxException, ExecutionException, InterruptedException {
        Integer connectionHandle = _createConnection();
        CompletableFuture<Integer> futureUpdateState= ConnectionApi.vcxConnectionUpdateState(connectionHandle);
        Awaitility.await().until(futureUpdateState::isDone);
        int updateStateResult = futureUpdateState.get();
        assert(updateStateResult== 1 );
        CompletableFuture<Integer> futureGetState= ConnectionApi.connectionGetState(connectionHandle);
        Awaitility.await().until(futureGetState::isDone);
        assert(futureGetState.get()== updateStateResult);

    }
    @Test
    @DisplayName("send offer connection")
    void sendOfferConnection() throws VcxException, ExecutionException, InterruptedException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(future::isDone);
        CompletableFuture<Integer> futureGetState= ConnectionApi.connectionGetState(connectionHandle);
        Awaitility.await().until(futureGetState::isDone);
        int connectionState = futureGetState.get();
        assert(connectionState == 2);
    }

    @Test
    @DisplayName("get abbreviated invite detials")
    void inviteDetailsAbbreviatedConnection() throws VcxException, ExecutionException, InterruptedException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        int connectionHandle = _createConnection();
        CompletableFuture<String> acceptInvitation = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(acceptInvitation::isDone);
        CompletableFuture<String> detials = ConnectionApi.connectionInviteDetails(connectionHandle,1);
        Awaitility.await().until(detials::isDone);
        assert(detials.get().contains("dp"));

    }

    @Test
    @DisplayName("get un-abbreviated invite detials")
    void inviteDetailsUnAbbreviatedConnection() throws VcxException, ExecutionException, InterruptedException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        int connectionHandle = _createConnection();
        CompletableFuture<String> acceptInvitation = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(acceptInvitation::isDone);
        CompletableFuture<String> detials = ConnectionApi.connectionInviteDetails(connectionHandle,0);
        Awaitility.await().until(detials::isDone);
        assert(detials.get().contains("senderAgencyDetail"));

    }


}
