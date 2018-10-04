package com.evernym.sdk.vcx;

import java.util.HashMap;
import java.util.Map;

/**
 * Enumeration of error codes returned by the vcx SDK.
 */
public enum ErrorCode {

    SUCCESS(0),
    UNKNOWN_ERROR(1001),
    CONNECTION_ERROR(1002),
    InvalidConnectionHandle(1003),
    INVALID_CONFIGURATION(1004),
    NOT_READY(1005),
    NO_ENDPOINT(1006),
    INVALID_OPTION(1007),
    INVALID_DID(1008),
    INVALID_VERKEY(1009),
    POST_MSG_FAILURE(1010),
    INVALID_NONCE(1011),
    INVALID_KEY_DELEGATE(1012),
    INVALID_URL(1013),
    NOT_BASE58(1014),
    INVALID_ISSUER_CREDENTIAL_HANDLE(1015),
    INVALID_JSON(1016),
    INVALID_PROOF_HANDLE(1017),
    INVALID_CREDENTIAL_REQUEST(1018),
    INVALID_MSGPACK(1019),
    INVALID_MESSAGES(1020),
    INVALID_ATTRIBUTES_STRUCTURE(1021),
    BIG_NUMBER_ERROR(1022),
    INVALID_PROOF(1023),
    INVALID_GENESIS_TXN_PATH(1024),
    CREATE_POOL_CONFIG_PARAMETERS(1025),
    CREATE_POOL_CONFIG(1026),
    INVALID_PROOF_CREDENTIAL_DATA(1027),
    INDY_SUBMIT_REQUEST_ERR(1028),
    BUILD_CREDENTIAL_DEF_REQ_ERR(1029),
    NO_POOL_OPEN(1030),
    INVALID_SCHEMA(1031),
    FAILED_PROOF_COMPLIANCE(1032),
    INVALID_HTTP_RESPONSE(1033),
    CREATE_CREDENTIAL_DEF_ERR(1034),
    UNKNOWN_LIBINDY_ERROR(1035),
    INVALID_CREDENTIAL_DEF_JSON(1036),
    INVALID_CREDENTIAL_DEF_HANDLE(1037),
    TIMEOUT_LIBINDY_ERROR(1038),
    CREDENTIAL_DEF_ALREADY_CREATED(1039),
    INVALID_SCHEMA_SEQ_NO(1040),
    INVALID_SCHEMA_CREATION(1041),
    INVALID_SCHEMA_HANDLE(1042),
    INVALID_MASTER_SECRET(1043),
    ALREADY_INITIALIZED(1044),
    INVALID_INVITE_DETAILS(1045),
    INVALID_SELF_ATTESTED_VAL(1046),
    INVALID_PREDICATE(1047),
    INVALID_OBJ_HANDLE(1048),
    INVALID_DISCLOSED_PROOF_HANDLE(1049),
    SERIALIZATION_ERROR(1050),
    WALLET_ALREADY_EXISTS(1051),
    WALLET_ALREADY_OPEN(1052),
    INSUFFICIENT_TOKEN_AMOUNT(1064),
    WALLET_ITEM_NOT_FOUND(212),
    WALLET_ITEM_CANNOT_ADD(213),
    INVALID_CREDENTIAL_HANDLE(1053),
    INVALID_CREDENTIAL_JSON(1054),
    CREATE_CREDENTIAL_REQUEST_ERROR(1055),
    CREATE_PROOF_ERROR(1056),
    UNIDENTIFIED_ERROR_CODE(9999); //Wrapper expects to never receive 9999 from libindy. If libindy ever reaches this number in error codes, please increment number in UNIDENTIFIED_ERROR_CODE(<new bigger number>)

    private int value;
    private static Map<Integer, ErrorCode> map = new HashMap<Integer, ErrorCode>();

    private ErrorCode(int value) {

        this.value = value;
    }

    static {

        for (ErrorCode errorCode : ErrorCode.values()) {

            map.put(errorCode.value, errorCode);
        }
    }

    /**
     * Gets the ErrorCode that corresponds to the specified int value.
     *
     * @param value The integer to get the error code for.
     * @return The ErrorCode that corresponds to the specified integer.
     */
    public static ErrorCode valueOf(int value) {
        if(map.containsKey(value)){
            return map.get(value);
        }else{
            return UNIDENTIFIED_ERROR_CODE;
        }

    }

    /**
     * Gets the integer value for a specific ErrorCode.
     *
     * @return The integer value of the ErrorCode.
     */
    public int value() {

        return this.value;
    }
}
