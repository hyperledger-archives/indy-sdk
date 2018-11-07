package com.evernym.connectme.wrappertest;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.api.vcx.Vcx;
import com.evernym.sdk.vcx.api.vcx.Utils;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileOutputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.io.OutputStreamWriter;
import java.util.concurrent.ExecutionException;

//import java.util.concurrent.ExecutionException;


public class MainActivity extends AppCompatActivity {
    private static final String TAG = "MainActivity";
    public static File vcx_config;
    public static File pool_config;
    public static final String POOL_CONFIG = "{\"data\":{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"client_ip\":\"35.164.240.131\",\"client_port\":9702,\"node_ip\":\"35.164.240.131\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}\n"
            + "{\"data\":{\"alias\":\"Node2\",\"blskey\":\"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk\",\"client_ip\":\"35.164.240.131\",\"client_port\":9704,\"node_ip\":\"35.164.240.131\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"\n}"
            + "{\"data\":{\"alias\":\"Node3\",\"blskey\":\"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5\",\"client_ip\":\"35.164.240.131\",\"client_port\":9706,\"node_ip\":\"35.164.240.131\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"\n}"
            + "{\"data\":{\"alias\":\"Node4\",\"blskey\":\"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw\",\"client_ip\":\"35.164.240.131\",\"client_port\":9708,\"node_ip\":\"35.164.240.131\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}";
    public static final String VCX_CONFIG = "{\n" +
            "\"agency_did\": \"VsKV7grR1BUE29mG2Fm2kX\",\n" +
            "\"agency_verkey\": \"Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR\",\n" +
            "\"agency_endpoint\": \"http://localhost:8080\",\n" +
            "\"genesis_path\":\"" + POOL_CONFIG + "\",\n" +
            "\"institution_name\": \"institution\",\n" +
            "\"institution_logo_url\": \"http://robohash.org/234\",\n" +
            "\"institution_did\": \"EwsFhWVoc3Fwqzrwe998aQ\",\n" +
            "\"institution_verkey\": \"8brs38hPDkw5yhtzyk2tz7zkp8ijTyWnER165zDQbpK6\",\n" +
            "\"remote_to_sdk_did\": \"EtfeMFytvYTKnWwqTScp9D\",\n" +
            "\"remote_to_sdk_verkey\": \"8a7hZDyJK1nNCizRCKMr4H4QbDm8Gg2vcbDRab8SVfsi\",\n" +
            "\"sdk_to_remote_did\": \"KacwZ2ndG6396KXJ9NDDw6\",\n" +
            "\"sdk_to_remote_verkey\": \"B8LgZGxEPcpTJfZkeqXuKNLihM1Awm8yidqsNwYi5QGc\"\n" +
            "}";
    public static final String PROVISIONING_CONFIG = "{\n" +
            "\"agency_url\": \"https://cagency.pdev.evernym.com\",\n" +
            "\"agency_did\": \"dTLdJqRZLwMuWSogcKfBT\",\n" +
            "\"agency_verkey\": \"LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH\",\n" +
            "\"wallet_name\":\"WrapperTest_Wallet\",\n" +
            "\"wallet_key\": \"test123\",\n" +
            "\"agent_seed\": \"null\",\n" +
            "\"enterprise_seed\": \"null\",\n" +
            "}";


    void createConfigFiles() {
        try {

            MainActivity.pool_config = File.createTempFile("pool_config", ".txn", this.getBaseContext().getCacheDir());
            Log.v(TAG, this.getBaseContext().getCacheDir() + "-=> is the dir");
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


            vcx_config = File.createTempFile("vcx_config", ".json", this.getBaseContext().getCacheDir());
            FileWriter vcx_fw = new FileWriter(vcx_config);
            vcx_fw.write("{\n" +
                    "\"agency_did\": \"VsKV7grR1BUE29mG2Fm2kX\",\n" +
                    "\"agency_verkey\": \"Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR\",\n" +
                    "\"agency_endpoint\": \"http://localhost:8080\",\n" +
                    "\"genesis_path\":\"" + pool_config.getAbsolutePath() + "\",\n" +
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

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        if (!LibVcx.isInitialized()) {
            LibVcx.init();
        }

        final Button provisionButton = findViewById(R.id.prov);

        provisionButton.setOnClickListener(v -> {
            String prov_res = Utils.vcxProvisionAgent(PROVISIONING_CONFIG);
            Log.d(TAG, "Prov test: " + prov_res);
        });
        final Button provisionAsyncButton = findViewById(R.id.prov_async);
        provisionAsyncButton.setOnClickListener(v -> {
            Log.d(TAG, "onClick: Provision Async clicked");
            try {
                String result = Utils.vcxProvisionAgenctAsync(PROVISIONING_CONFIG).get();
            } catch (VcxException | InterruptedException | ExecutionException e) {
                e.printStackTrace();
            }
        });
        final Button initButton = findViewById(R.id.init);
        initButton.setOnClickListener(v -> {
            try {
                int result = Vcx.vcxInitWithJsonConfig(PROVISIONING_CONFIG).get();
                Log.d(TAG, "result of init with config json: " + Vcx.vcxErrorMessage(result));
            } catch (VcxException | InterruptedException | ExecutionException e) {
                e.printStackTrace();
            }
        });
        final Button initWithConfigFileButton = findViewById(R.id.init_configFile);
        initWithConfigFileButton.setOnClickListener(v -> {
            createConfigFiles();
            try {
                int result = Vcx.vcxInit(vcx_config.getPath()).get();
                Log.d(TAG, "result of init with config file: " + Vcx.vcxErrorMessage(result));
            } catch (InterruptedException | ExecutionException | VcxException e) {
                e.printStackTrace();
            }
        });

    }
}
