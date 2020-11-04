package org.hyperledger.indy.sdk;

import java.io.File;
import java.util.HashMap;
import java.util.Map;

import com.sun.jna.*;
import com.sun.jna.ptr.PointerByReference;
import static com.sun.jna.Native.detach;

public abstract class LibIndyPlugin {
    public static final String LIBRARY_NAME = "indystrgpostgres";
    static final DefaultTypeMapper MAPPER = new DefaultTypeMapper();

    public interface API extends Library {

        // indy-sdk/experimental/plugins/postgres_storage/src/lib.rs
        public int postgresstorage_init();
        /** when using pgsql as plugin from external code
         *
         * @param config Postgress initConfig
         *               {
         *                  "url": "postgress-server-db:5432",
         *                  "wallet_scheme": "MultiWalletSingleTable"
         *               }
         *               ref : indy-sdk/experimental/plugins/postgres_storage/README.md
         * @param credentials Postgress initCredentials
         *                {
         *                  "account": "user_name",
         *                  "password": "user_name_password",
         *                  "admin_account": "admin_name",
         *                  "admin_password": "admin_name_password"
         *                 }
         */
        public int init_storagetype(String config, String credentials);
    }

    /*
     * Initialization
     */

    public static LibIndyPlugin.API api = null;

    static {
        MAPPER.addTypeConverter(IndyBool.class, IndyBool.MAPPER);

        try {

            init();
        } catch (UnsatisfiedLinkError ex) {

            // Library could not be found in standard OS locations.
            // Call init(File file) explicitly with absolute library path.
        }
    }

    /**
     * Initializes the API with the default library.
     */
    public static void init() {
        Map<String, Object> options = new HashMap<String, Object>();
        options.put(Library.OPTION_TYPE_MAPPER, MAPPER);
        api = Native.loadLibrary(LIBRARY_NAME, API.class, options);
    }

}
