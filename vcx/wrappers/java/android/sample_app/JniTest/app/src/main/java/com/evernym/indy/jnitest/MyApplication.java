package com.evernym.indy.jnitest;

import android.app.Application;
import android.content.Context;

/**
 * Created by abdussami on 23/04/18.
 */

public class MyApplication extends Application {

    private static Context context;

    public void onCreate() {
        super.onCreate();
        MyApplication.context = getApplicationContext();
    }

    public static Context getAppContext() {
        return MyApplication.context;
    }
}

