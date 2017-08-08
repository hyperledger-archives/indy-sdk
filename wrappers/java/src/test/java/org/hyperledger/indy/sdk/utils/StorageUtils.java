package org.hyperledger.indy.sdk.utils;

import org.apache.commons.io.FileUtils;

import java.io.File;
import java.io.IOException;


public class StorageUtils {

	private static void cleanDirectory(File path) throws IOException {
		if (path.isDirectory()) {
			FileUtils.cleanDirectory(path);
		}
	}

	public static void cleanupStorage() throws IOException {
		System.out.println("cleanupStorage >>");
		Thread.dumpStack();

		File tmpDir = new File(getTmpPath());
		File homeDir = new File(getIndyHomePath());

		StorageUtils.cleanDirectory(tmpDir);
		StorageUtils.cleanDirectory(homeDir);
		System.out.println("cleanupStorage <<");
	}

	public static String getIndyHomePath() {
		return FileUtils.getUserDirectoryPath() + "/.indy/";
	}

	public static String getIndyHomePath(String filename) {
		return getIndyHomePath() + filename;
	}

	public static String getTmpPath() {
		return FileUtils.getTempDirectoryPath() + "/indy/";
	}

	public static String getTmpPath(String filename) {
		return getTmpPath() + filename;
	}
}
