package org.hyperledger.indy.sdk.utils;


import org.hyperledger.indy.sdk.LibIndy;

import java.io.File;

public class InitHelper {
	public static void init() {

		if (!LibIndy.isInitialized()) LibIndy.init(new File("./lib/libindy.so"));

	}
}
