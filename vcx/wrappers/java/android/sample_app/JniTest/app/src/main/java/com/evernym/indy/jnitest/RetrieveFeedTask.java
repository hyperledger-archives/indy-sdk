package com.evernym.indy.jnitest;

import android.os.AsyncTask;
import android.util.Log;

import org.xml.sax.InputSource;
import org.xml.sax.XMLReader;

import java.io.BufferedInputStream;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.MalformedURLException;
import java.net.ProtocolException;
import java.net.URL;

import javax.xml.parsers.SAXParser;
import javax.xml.parsers.SAXParserFactory;

/**
 * Created by abdussami on 30/04/18.
 */

class RetrieveFeedTask extends AsyncTask<String, Void, String> {

    private Exception exception;

    protected String doInBackground(String... urls) {
        URL url = null;
        String chunks = "";
        try {
            url = new URL("https://reqres.in/api/users?page=2");
        } catch (MalformedURLException e) {
            e.printStackTrace();
        }
        HttpURLConnection urlConnection = null;
        try {
            urlConnection = (HttpURLConnection)url.openConnection();
        } catch (IOException e) {
            e.printStackTrace();
        }
        try {
            urlConnection.setRequestMethod("GET");
        } catch (ProtocolException e) {
            e.printStackTrace();
        }
        int statusCode = 0;
        try {
            statusCode = urlConnection.getResponseCode();
        } catch (IOException e) {
            e.printStackTrace();
        }
        if (statusCode ==  200) {
            InputStream it = null;
            try {
                it = new BufferedInputStream(urlConnection.getInputStream());
            } catch (IOException e) {
                e.printStackTrace();
            }
            InputStreamReader read = new InputStreamReader(it);
            BufferedReader buff = new BufferedReader(read);
            StringBuilder dta = new StringBuilder();

            try {
                while((chunks = buff.readLine()) != null)
                {
                    dta.append(chunks);
                }
                Log.d("MainActivity", "http message C:" + urlConnection.getContent());
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
        else
        {
            //Handle else
            Log.d("MainActivity", "http statud C:" + statusCode);
        }
        return chunks;
    }


}