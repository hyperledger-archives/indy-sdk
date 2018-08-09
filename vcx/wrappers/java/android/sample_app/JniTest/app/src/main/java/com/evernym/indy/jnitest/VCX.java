package com.evernym.indy.jnitest;

import android.util.Log;

import com.sun.jna.Callback;

import java.util.concurrent.ExecutionException;

import java9.util.concurrent.CompletableFuture;

/**
 * Created by abdussami on 10/05/18.
 */

public class VCX extends IndyJava.API {
    private static final String TAG = "MainActivity";

    public static Callback initCB = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err) {
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
            if (!checkCallback(future, err)) return;

            Void result = null;
            future.complete(result);

        }
    };

    public static Callback connCreateCB = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err,int connection_handle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(xcommand_handle);
            if (!checkCallback(future, err)) return;

            int result = connection_handle;
            future.complete(result);

        }
    };

    public static void init(String configPath) {
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = MainActivity.api.vcx_init(
                commandHandle, configPath,
                initCB);

        try {
            Log.d(TAG, String.valueOf(future.get()));
        } catch (InterruptedException e) {
            e.printStackTrace();
        } catch (ExecutionException e) {
            e.printStackTrace();
        }
    }
    public static void connCreate(String sourceId) {
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = MainActivity.api.vcx_connection_create(
                commandHandle, sourceId,
                connCreateCB);

        try {
            Log.d(TAG, "Connection handle => " + String.valueOf(future.get()));
        } catch (InterruptedException e) {
            e.printStackTrace();
        } catch (ExecutionException e) {
            e.printStackTrace();
        }
    }


}
