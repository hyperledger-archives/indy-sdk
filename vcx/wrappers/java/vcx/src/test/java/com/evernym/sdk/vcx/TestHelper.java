package com.evernym.sdk.vcx;


public class TestHelper {
    public static boolean vcxInitialized = false;
    public static String VCX_CONFIG_TEST_MODE = "ENABLE_TEST_MODE";
    public static String getConnectionId(){
        return "testConnectionId";
    }

    public static String convertToValidJson(String InvalidJson){
        String validJson = InvalidJson.replace("'","\"");
        return validJson;
    }
}
