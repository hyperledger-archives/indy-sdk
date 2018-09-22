//package com.evernym.sdk.vcx;
//
//import android.util.Log;
//
//import com.evernym.sdk.vcx.connection.ConnectionApi;
//import com.evernym.sdk.vcx.connection.InvalidConnectionHandleException;
//import com.evernym.sdk.vcx.vcx.VcxApi;
//
//import org.awaitility.Awaitility;
//import org.junit.Before;
//import org.junit.Test;
//
//import java.util.concurrent.ExecutionException;
//
//import java9.util.concurrent.CompletableFuture;
//import java.util.concurrent.Callable;
//
//import org.awaitility.Awaitility.*;
//import org.awaitility.Duration.*;
//
//import java.util.concurrent.TimeUnit.*;
//
//import org.hamcrest.Matchers.*;
//import org.junit.Assert.*;
//
//import static junit.framework.Assert.*;
//
//public class ConnectionApiTest {
//
//    @Before
//    public void setup() throws VcxException, ExecutionException, InterruptedException {
//
//        if (!TestHelper.vcxInitialized) {
//            CompletableFuture<Integer> result = VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE);
//            result.get();
//            TestHelper.vcxInitialized = true;
//        }
//
//    }
//
//    private int _createConnection() throws VcxException {
//        CompletableFuture<Integer> futureResult = ConnectionApi.vcxConnectionCreate(TestHelper.getConnectionId());
//        Awaitility.await().until(futureResult::isDone);
//
//        Integer result = futureResult.getNow(-1);
//        if(result == -1){
//            throw new VcxException("Unable to create connection handle",0);
//        }else{
////            System.out.println("Connection created with connection handle => "  + result);
//            return result;
//        }
//
//    }
//
//    @Test
//    public void createConnection() throws VcxException {
//
//        long ConnectionHandle = _createConnection();
//        assertNotSame(null,ConnectionHandle);
//        assertNotSame( 0,ConnectionHandle);
//    }
//
//    @Test
//    public void createConnectionWithoutPhone() throws VcxException {
//        String payload= "{ 'connection_type': 'SMS' }";
//        Integer ConnectionHandle = _createConnection();
//        CompletableFuture<String> future = ConnectionApi.vcxAcceptInvitation(ConnectionHandle,TestHelper.convertToValidJson(payload));
//        Awaitility.await().until(future::isDone);
//        assertNotSame("",future.getNow(""));
//    }
//
//    @Test
//    public void createConnectionWithPhone() throws VcxException {
//        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
//        Integer ConnectionHandle = _createConnection();
//        CompletableFuture<String> future = ConnectionApi.vcxAcceptInvitation(ConnectionHandle,TestHelper.convertToValidJson(payload));
//        Awaitility.await().until(future::isDone);
//        assertNotSame("",future.getNow(""));
//
//
//    }
//
//    @Test(expected = InvalidConnectionHandleException.class)
//    public void throwInvalidConnectionHandleException() throws VcxException {
//        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
//        CompletableFuture<String> future = ConnectionApi.vcxAcceptInvitation(8765,TestHelper.convertToValidJson(payload));
//        Awaitility.await().until(future::isDone);
//        assertNotSame("",future.getNow(""));
//    }
//
//    @Test
//    public void serializeConnection() throws VcxException {
//        Integer ConnectionHandle = _createConnection();
//        CompletableFuture<String> future = ConnectionApi.connectionSerialize(ConnectionHandle);
//        Awaitility.await().until(future::isDone);
//        String serializedJson = future.getNow("");
//        System.out.println(serializedJson);
//        assertNotSame("",serializedJson);
//        assert(serializedJson.contains("version"));
//        assert(serializedJson.contains("data"));
//    }
//
//    @Test
//    public void deserializeConnection() throws VcxException {
//        Integer ConnectionHandle = _createConnection();
//        CompletableFuture<String> future = ConnectionApi.connectionSerialize(ConnectionHandle);
//        Awaitility.await().until(future::isDone);
//        String serializedJson = future.getNow("");
//        System.out.println(serializedJson);
//        CompletableFuture<Integer> deserializeFuture = ConnectionApi.connectionDeserialize(serializedJson);
//        Awaitility.await().until(deserializeFuture::isDone);
//        assertEquals(ConnectionHandle,deserializeFuture.getNow(-1));
//
//    }
//
//
//}
