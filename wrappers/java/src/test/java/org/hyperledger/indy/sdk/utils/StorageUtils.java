package org.hyperledger.indy.sdk.utils;

import org.apache.commons.io.FileUtils;

import java.io.File;


public class StorageUtils {

    private static void cleanDirectory(File path) throws Exception {
        if (path.isDirectory()) {
            FileUtils.cleanDirectory(path);
        }
    }

    public static void cleanupStorage() throws Exception {

        File tmpDir = new File(FileUtils.getTempDirectoryPath() + "/indy");
        File homeDir = new File(FileUtils.getUserDirectoryPath() + "/.indy");

        StorageUtils.cleanDirectory(tmpDir);
        StorageUtils.cleanDirectory(homeDir);
    }
}
