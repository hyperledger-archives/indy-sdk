package org.hyperledger.indy.sdk.helpres;

import org.apache.commons.io.FileUtils;

import java.io.File;


public class StorageHelper {

    private static void cleanDirectory(File path) throws Exception {
        if (path.isDirectory()) {
            FileUtils.cleanDirectory(path);
        }
    }

    public static void cleanupStorage() throws Exception {

        File tmpDir = new File(FileUtils.getTempDirectoryPath() + "/indy");
        File homeDir = new File(FileUtils.getUserDirectoryPath() + "/.indy");

        StorageHelper.cleanDirectory(tmpDir);
        StorageHelper.cleanDirectory(homeDir);
    }
}
