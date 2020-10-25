package com.sktelecom.ston.demo;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Context;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.system.ErrnoException;
import android.system.Os;
import android.util.Log;
import android.view.View;
import android.widget.EditText;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.evernym.sdk.vcx.credential.CredentialApi;
import com.evernym.sdk.vcx.proof.DisclosedProofApi;
import com.evernym.sdk.vcx.utils.UtilsApi;
import com.evernym.sdk.vcx.vcx.VcxApi;
import com.google.common.io.Files;
import com.jayway.jsonpath.DocumentContext;
import com.jayway.jsonpath.JsonPath;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.concurrent.ExecutionException;

public class MainActivity extends AppCompatActivity {

    private static final String TAG = "VCX";
    private static final String CONFIG = "provision_config";

    private String connection;
    SharedPreferences sharedPref;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        //Control logs from libvcx
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "ERROR");

        try {
            Os.setenv("EXTERNAL_STORAGE", getExternalFilesDir(null).getAbsolutePath(), true);
        } catch (ErrnoException e) {
            e.printStackTrace();
        }

        if (!LibVcx.isInitialized()) {
            LibVcx.init();
        }

        sharedPref = getPreferences(Context.MODE_PRIVATE);

        //If provisioned, just init vcx with saved configuration
        if (sharedPref.contains(CONFIG)) {
            String config = sharedPref.getString(CONFIG, "");

            //Initialize libvcx with configuration
            try {
                int state = VcxApi.vcxInitWithConfig(config).get();
                Log.d(TAG, "Init with config: " + VcxApi.vcxErrorCMessage(state));
            } catch (VcxException | InterruptedException | ExecutionException e) {
                e.printStackTrace();
            }
        }

    }

    public void onProvisionClicked (View v) {

        //Provision an agent and wallet, get back configuration details
        InputStream inputStreamConfig = getResources().openRawResource(R.raw.provision_config);

        String provisionConfig = null;

        try {
            provisionConfig = convertStreamToString(inputStreamConfig);
        } catch (Exception e) {
            e.printStackTrace();
        }

        String config = UtilsApi.vcxProvisionAgent(provisionConfig);
        Log.d(TAG, "Config: " + config);

        InputStream inputStreamGenesis = getResources().openRawResource(R.raw.genesis_txn);

        File poolConfig = null;

        try {
            byte[] buffer = new byte[inputStreamGenesis.available()];
            inputStreamGenesis.read(buffer);
            poolConfig = File.createTempFile("pool_config", ".txn", this.getBaseContext().getCacheDir());
            Files.write(buffer, poolConfig);
        } catch (IOException e) {
            e.printStackTrace();
        }

        //Set some additional configuration options specific to alice
        DocumentContext ctx = JsonPath.parse(config);
        ctx.set("$.institution_name", "alice_institute");
        ctx.set("$.institution_logo_url", "http://robohash.org/234");
        ctx.set("$.genesis_path", poolConfig.getAbsolutePath());
        Log.d(TAG, "New config: " + ctx.jsonString());

        //Initialize libvcx with new configuration
        try {
            int state = VcxApi.vcxInitWithConfig(ctx.jsonString()).get();
            Log.d(TAG, "Init with config: " + VcxApi.vcxErrorCMessage(state));
        } catch (VcxException | InterruptedException | ExecutionException e) {
            e.printStackTrace();
        }

        //Save configuration details in the shared preference
        SharedPreferences.Editor editor = sharedPref.edit();
        editor.putString(CONFIG, ctx.jsonString());
        editor.commit();

    }

    public void onConnectionClicked (View v) {
        //Get invitation details from the text box
        EditText invitationEditText = (EditText) findViewById(R.id.invitation);
        String invitation = invitationEditText.getText().toString();

        try {
            //Create a connection to faber
            int connectionHandle = ConnectionApi.vcxCreateConnectionWithInvite("alice", invitation).get();

            String connectionDetails = ConnectionApi.vcxConnectionConnect(connectionHandle, "{\"use_public_did\":true}").get();
            Log.d(TAG, "Connection details: " + connectionDetails);

            int state = ConnectionApi.vcxConnectionUpdateState(connectionHandle).get();
            while(state != 4){
                try {
                    Thread.sleep(2 * 1000);
                } catch (InterruptedException ie) {
                    Thread.currentThread().interrupt();
                }

                state = ConnectionApi.vcxConnectionUpdateState(connectionHandle).get();
            }

            //Serialize the connection to use in requesting a credential and to present a proof
            connection = ConnectionApi.connectionSerialize(connectionHandle).get();
            Log.d(TAG, "Serialized connection: " + connection);

            ConnectionApi.connectionRelease(connectionHandle);
        } catch (VcxException | ExecutionException | InterruptedException e) {
            e.printStackTrace();
        }
    }

    public void onAcceptOfferClicked (View v) {
        try {
            //Deserialize a saved connection
            int connectionHandle = ConnectionApi.connectionDeserialize(connection).get();

            //Check agency for a credential offer
            String offers = CredentialApi.credentialGetOffers(connectionHandle).get();

            //Create a credential object from the credential offer
            List<String> credentialOffer = JsonPath.read(offers,"$.[0]");
            int credentialHandle = CredentialApi.credentialCreateWithOffer("1", JsonPath.parse(credentialOffer).jsonString()).get();

            //Send credential request
            CredentialApi.credentialSendRequest(credentialHandle, connectionHandle, 0).get();

            //Poll agency and accept credential offer from faber
            int state = CredentialApi.credentialUpdateState(credentialHandle).get();
            while (state != 4) {
                try {
                    Thread.sleep(1 * 1000);
                } catch (InterruptedException ie) {
                    Thread.currentThread().interrupt();
                }

                state = CredentialApi.credentialUpdateState(credentialHandle).get();
            }

            String jsonString = CredentialApi.credentialSerialize(credentialHandle).get();
            Log.d(TAG, "Serialized credential: " + jsonString);

            CredentialApi.credentialRelease(credentialHandle);

        } catch (VcxException | ExecutionException | InterruptedException e) {
            e.printStackTrace();
        }
    }

    public void onPresentProofClicked (View v) {
        try {
            int connectionHandle = ConnectionApi.connectionDeserialize(connection).get();

            //Check agency for a proof request
            String requests = DisclosedProofApi.proofGetRequests(connectionHandle).get();

            //Create a Disclosed proof object from proof request
            LinkedHashMap<String, Object> request = JsonPath.read(requests,"$.[0]");
            int proofHandle = DisclosedProofApi.proofCreateWithRequest("1", JsonPath.parse(request).jsonString()).get();

            //Query for credentials in the wallet that satisfy the proof request
            String credentials = DisclosedProofApi.proofRetrieveCredentials(proofHandle).get();

            //Use the first available credentials to satisfy the proof request
            DocumentContext ctx = JsonPath.parse(credentials);
            LinkedHashMap<String, Object> attrs = ctx.read("$.attrs");
            for(String key : attrs.keySet()){
                LinkedHashMap<String, Object> attr = JsonPath.read(attrs.get(key),"$.[0]");
                ctx.set("$.attrs." + key, JsonPath.parse("{\"credential\":null}").json());
                ctx.set("$.attrs." + key + ".credential", attr);
            }

            //Generate and send the proof
            DisclosedProofApi.proofGenerate(proofHandle, ctx.jsonString(), "{}").get();
            DisclosedProofApi.proofSend(proofHandle, connectionHandle).get();

            int state = DisclosedProofApi.proofUpdateState(proofHandle).get();
            while (state != 4) {
                try {
                    Thread.sleep(1 * 1000);
                } catch (InterruptedException ie) {
                    Thread.currentThread().interrupt();
                }

                state = DisclosedProofApi.proofUpdateState(proofHandle).get();
            }

            String serializedProof = DisclosedProofApi.proofSerialize(proofHandle).get();
            Log.d(TAG, "Serialized proof: " + serializedProof);

            DisclosedProofApi.proofRelease(proofHandle);

        } catch (VcxException | ExecutionException | InterruptedException e) {
            e.printStackTrace();
        }
    }

    private String convertStreamToString(InputStream is) throws Exception {
        BufferedReader reader = new BufferedReader(new InputStreamReader(is));
        StringBuilder sb = new StringBuilder();
        String line = null;
        while ((line = reader.readLine()) != null) {
            sb.append(line).append("\n");
        }
        reader.close();
        return sb.toString();
    }
}
