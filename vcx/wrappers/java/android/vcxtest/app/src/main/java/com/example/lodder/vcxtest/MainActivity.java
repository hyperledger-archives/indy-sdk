package com.example.lodder.vcxtest;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.widget.TextView;

import com.sun.jna.Native;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.Map;


public class MainActivity extends AppCompatActivity {

    // Used to load the 'native-lib' library on application startup.
    private final VCXJniHandler handler = new VCXJniHandler();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        try {
            //injectEnvironmentVariable("RUST_BACKTRACE", "1");
            //Log.d("**> Environment", System.getenv().toString());
            //System.setProperty("RUST_BACKTRACE", "full");
            System.loadLibrary("crypto");
            System.loadLibrary("ssl");
            System.loadLibrary("vcxall");
            Native.register(MainActivity.class, "vcxall");
        } catch (Exception e) {
            Log.e("FAIL", e.getMessage());
        }

        // Example of a call to a native method
        TextView tv = (TextView) findViewById(R.id.sample_text);
        tv.setText(vcx_version());
        String config="{\"agency_url\": \"https://cagency.pdev.evernym.com\", \"agency_did\": \"dTLdJqRZLwMuWSogcKfBT\",\"wallet_name\":\"wallet2\",\"wallet_key\":\"wallet-key\",\"agent_seed\":null,\"enterprise_seed\":null, \"agency_verkey\": \"LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH\"}";

        int result = vcx_agent_provision_async(10, config, handler);
        Log.d("HELP", result + "");
    }

    public native String vcx_version();

    /**
     * A native method that is implemented by the 'native-lib' native library,
     * which is packaged with this application.
     */
    public native int vcx_agent_provision_async(int handle, String json, VCXJniHandler callbackHandler);



    private static void injectEnvironmentVariable(String key, String value)
            throws Exception {

        Class<?> processEnvironment = Class.forName("java.lang.ProcessEnvironment");

        Field unmodifiableMapField = getAccessibleField(processEnvironment, "theUnmodifiableEnvironment");
        Object unmodifiableMap = unmodifiableMapField.get(null);
        injectIntoUnmodifiableMap(key, value, unmodifiableMap);

        Field mapField = getAccessibleField(processEnvironment, "theEnvironment");
        Map<Object, Object> map = (Map<Object, Object>) mapField.get(null);

        Class<?> processEnvironmentVar = Class.forName("java.lang.ProcessEnvironment$Variable");
        Method varValueOf = processEnvironmentVar.getMethod("valueOf", String.class);
        Object vKey = varValueOf.invoke(null, key);

        Class<?> processEnvironmentVal = Class.forName("java.lang.ProcessEnvironment$Value");
        Method valValueOf = processEnvironmentVal.getMethod("valueOf", String.class);
        Object vValue = valValueOf.invoke(null, value);

        map.put(vKey, vValue);
    }

    private static Field getAccessibleField(Class<?> clazz, String fieldName)
            throws NoSuchFieldException {

        Field field = clazz.getDeclaredField(fieldName);
        field.setAccessible(true);
        return field;
    }

    private static void injectIntoUnmodifiableMap(String key, String value, Object map)
            throws ReflectiveOperationException {

        Class unmodifiableMap = Class.forName("java.util.Collections$UnmodifiableMap");
        Field field = getAccessibleField(unmodifiableMap, "m");
        Object obj = field.get(map);
        ((Map<String, String>) obj).put(key, value);
    }
}
