package com.evernym.indy.jnitest;

import android.content.Context;
import android.os.AsyncTask;
import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.system.ErrnoException;
import android.system.Os;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;

import com.sun.jna.Callback;
import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.NativeLibrary;

import java.io.BufferedInputStream;
import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.StringWriter;
import java.io.UnsupportedEncodingException;
import java.net.HttpURLConnection;
import java.net.MalformedURLException;
import java.net.ProtocolException;
import java.net.URL;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ExecutionException;

import java9.util.concurrent.CompletableFuture;

//import static com.evernym.indy.jnitest.Utils.addFuture;
//import static com.evernym.indy.jnitest.Utils.checkCallback;
//import static com.evernym.indy.jnitest.Utils.removeFuture;

public class MainActivity extends AppCompatActivity {
    private static Context context;
    private static final String TAG = "MainActivity";
    public static Callback createPoolLedgerConfigCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, int connection_handle) {
            Log.v(TAG, xcommand_handle + "-=> " + err);
        }
    };



    public interface API extends Library {
        public int vcx_init(int command_handle, String config_path, Callback cb);

        public int vcx_connection_create(int command_handle, String source_id, Callback cb);

        public String vcx_error_c_message(int error_code);
        public int vcx_reset();

        public boolean zmq_has(String capability);
        public int zmq_curve_keypair(byte[] public_key, byte[] private_key);
        public boolean zmq_test();

        public String zmq_has_test();
    }

    public static API api = null;
    public static File vcx_config;
    public static File pool_config;

    public static void initConnection(){
        VCX.init(vcx_config.getPath());
        VCX.connCreate("123");

    }



    public static String loadBinaries(){
        try {
            Os.setenv("RUST_BACKTRACE","1",true);
        } catch (ErrnoException e) {
            e.printStackTrace();
        }
        StringBuilder text = new StringBuilder();
        String line;
        try {
            BufferedReader br = new BufferedReader(new FileReader(pool_config));

            while ((line = br.readLine()) != null) {
                text.append(line);
                text.append("=newline=>");
            }
            br.close();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }
        Log.d(TAG, text.toString());
        System.loadLibrary("vcx_shim");
        api = Native.loadLibrary("vcx_shim", API.class);
        Log.d(TAG, "binaries loaded");
        byte pub[] = new byte[41];
        byte priv[] = new byte[41];
        int res = api.zmq_curve_keypair(pub,priv);
        boolean curve = api.zmq_has("curve");
        boolean ipc = api.zmq_has("ipc");
        Log.d(TAG, "curve => " + String.valueOf(curve) + " ipc => " + String.valueOf(ipc));
        Log.d(TAG, "priv => " + Native.toString(priv) + " pub => " + Native.toString(pub));
        return text.toString();
    }

    public static void createConfigs() {
        try {

            MainActivity.pool_config = File.createTempFile("pool_config", ".txn", MyApplication.getAppContext().getCacheDir());
            Log.v(TAG, MyApplication.getAppContext().getCacheDir() + "-=> is the dir");
            FileOutputStream fos = new FileOutputStream(pool_config);

            BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(fos));

//            StringBuilder sb = new StringBuilder();
            bw.write("{\"data\":{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"client_ip\":\"35.164.240.131\",\"client_port\":9702,\"node_ip\":\"35.164.240.131\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}");
            bw.newLine();
            bw.write("{\"data\":{\"alias\":\"Node2\",\"blskey\":\"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk\",\"client_ip\":\"35.164.240.131\",\"client_port\":9704,\"node_ip\":\"35.164.240.131\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}");
            bw.newLine();
            bw.write("{\"data\":{\"alias\":\"Node3\",\"blskey\":\"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5\",\"client_ip\":\"35.164.240.131\",\"client_port\":9706,\"node_ip\":\"35.164.240.131\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}");
            bw.newLine();
            bw.write("{\"data\":{\"alias\":\"Node4\",\"blskey\":\"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw\",\"client_ip\":\"35.164.240.131\",\"client_port\":9708,\"node_ip\":\"35.164.240.131\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}");
            bw.newLine();
            bw.flush();
            bw.close();
            fos.close();


            vcx_config = File.createTempFile("vcx_config",".json", MyApplication.getAppContext().getCacheDir());
            FileWriter vcx_fw = new FileWriter(vcx_config);
            vcx_fw.write("{\n" +
                    "\"agency_did\": \"dTLdJqRZLwMuWSogcKfBT\",\n" +
                    "\"agency_verkey\": \"LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH\",\n" +
                    "\"agency_endpoint\": \"https://cagency.pdev.evernym.com\",\n" +
                    "\"genesis_path\":\""+ pool_config.getAbsolutePath() +"\",\n" +
                    "\"institution_name\": \"institution\",\n" +
                    "\"institution_logo_url\": \"http://robohash.org/234\",\n" +
                    "\"institution_did\": \"EwsFhWVoc3Fwqzrwe998aQ\",\n" +
                    "\"institution_verkey\": \"8brs38hPDkw5yhtzyk2tz7zkp8ijTyWnER165zDQbpK6\",\n" +
                    "\"remote_to_sdk_did\": \"EtfeMFytvYTKnWwqTScp9D\",\n" +
                    "\"remote_to_sdk_verkey\": \"8a7hZDyJK1nNCizRCKMr4H4QbDm8Gg2vcbDRab8SVfsi\",\n" +
                    "\"sdk_to_remote_did\": \"KacwZ2ndG6396KXJ9NDDw6\",\n" +
                    "\"sdk_to_remote_verkey\": \"B8LgZGxEPcpTJfZkeqXuKNLihM1Awm8yidqsNwYi5QGc\"\n" +
                    "}");
            vcx_fw.close();
            Log.d(TAG, "File path " + vcx_config.getPath());
            Log.d(TAG, "File path " + pool_config.getPath());
        } catch (IOException e) {
            Log.d(TAG, e.getMessage());
        }
    }

    public static Context getAppContext() {
        return MainActivity.context;
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        // Example of a call to a native method
        TextView tv = (TextView) findViewById(R.id.sample_text);
        MainActivity.context = getApplicationContext();
        final Button button = findViewById(R.id.button);
        button.setOnClickListener(new View.OnClickListener() {
            public void onClick(View v) {
                MainActivity.createConfigs();

//                try {
//                    String res = new RetrieveFeedTask().execute("").get();
//                    Log.d(TAG, "http message:" + res);
//                    ((TextView) findViewById(R.id.sample_text)).setText( res);
//                } catch (InterruptedException e) {
//                    e.printStackTrace();
//                } catch (ExecutionException e) {
//                    e.printStackTrace();
//                }

            }

        });

        final Button button2 = findViewById(R.id.button2);
        button2.setOnClickListener(new View.OnClickListener() {
            public void onClick(View v) {
                // Code here executes on main thread after user presses button
                ((TextView) findViewById(R.id.sample_text)).setText( MainActivity.loadBinaries());
            }
        });

        final Button button3 = findViewById(R.id.button3);
        button3.setOnClickListener(new View.OnClickListener() {
            public void onClick(View v) {
                // Code here executes on main thread after user presses button
                MainActivity.initConnection();
            }
        });
    }


}
