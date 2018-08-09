package com.example.lodder.vcxtest;

import android.util.Log;

public class VCXJniHandler implements com.sun.jna.Callback {
    public void callback(int xcommand_handle, int err, String config) {
        Log.d("Callback called", "xcommand_handle=" + xcommand_handle + ", err=" + err + ", config=" + config);
    }
}
