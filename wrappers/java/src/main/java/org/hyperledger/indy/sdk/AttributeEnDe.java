package org.hyperledger.indy.sdk;

import com.sun.istack.internal.NotNull;
import org.omg.PortableInterceptor.INACTIVE;

import java.math.BigInteger;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

public class AttributeEnDe {

    private static String STR_CODE = "1";
    private static String BOOL_CODE = "2";
    private static String BIGINT_CODE = "3";
    private static String FLOAT_CODE = "4";
    private static String NONE_CODE = "9";

    private static final String ENCODED_TRUE = BOOL_CODE + String.valueOf(Integer.MAX_VALUE + 2);
    private static final String ENCODED_FALSE = BOOL_CODE + String.valueOf(Integer.MAX_VALUE + 1);

    private final static char[] hexArray = "0123456789ABCDEF".toCharArray();

    public static String bytesToHex(byte[] bytes) {
        char[] hexChars = new char[bytes.length * 2];
        for (int j = 0; j < bytes.length; j++) {
            int v = bytes[j] & 0xFF;
            hexChars[j * 2] = hexArray[v >>> 4];
            hexChars[j * 2 + 1] = hexArray[v & 0x0F];
        }
        return new String(hexChars);
    }

    /**
     * @param int number
     * @return encoded int attribute
     */
    public static String encode(int number) {
        return Integer.valueOf(number).toString();
    }

    /**
     * @param b
     * @return
     */
    public static String encode(boolean b) {
        return (b) ? ENCODED_TRUE : ENCODED_FALSE;
    }

    /**
     * @param raw_value
     * @return
     */
    public static String encode(@NotNull BigInteger raw_value) {
        if ((raw_value.compareTo(BigInteger.valueOf(Integer.MAX_VALUE)) > 0) &&
                (raw_value.compareTo(BigInteger.valueOf(Integer.MIN_VALUE)) < 0)) {
            String stringified = raw_value.toString();
            String hex = bytesToHex(stringified.getBytes());
            BigInteger bi = new BigInteger(hex, 16);
            bi.add(BigInteger.valueOf(Integer.MAX_VALUE));
            return STR_CODE + bi.toString();
        }
        return encode(raw_value.intValue());
    }

    /**
     * @param raw_value
     * @return
     */
    public static String encode(@NotNull String raw_value) {
        BigInteger bi = new BigInteger(raw_value.getBytes());
        bi = bi.add(BigInteger.valueOf(Integer.MAX_VALUE));
        return STR_CODE + bi.toString();
    }

    /**
     * @return encoded null pointer
     */
    public static String encode() {
        return Integer.valueOf(Integer.MAX_VALUE).toString();
    }

    /*
    Decode encoded credential attribute value.
    :param value: numeric string to decode
    :return: decoded value, stringified if original was neither str, bool, int, nor float
    */
    public static Object decode(String encoded) {
        if (String.valueOf(Integer.MAX_VALUE).equals(encoded)) return null;

        BigInteger bi = new BigInteger(encoded);
        if ((BigInteger.valueOf(Integer.MIN_VALUE).compareTo(bi) <= 0)
                && (bi.compareTo(BigInteger.valueOf(Integer.MAX_VALUE)) < 0)) {
            return new Integer(bi.intValue());
        }

        String prefix = encoded.substring(0, 1);
        String value = encoded.substring(1);
        BigInteger ival = new BigInteger(value, 16);
        ival = ival.subtract(BigInteger.valueOf(Integer.MAX_VALUE));

        if (ival.compareTo(BigInteger.ZERO) == 0) {
            return "";
        }
        if (BOOL_CODE.equals(prefix) && ival.compareTo(BigInteger.ONE) == 0) {
            return Boolean.valueOf(false);
        }
        if (BOOL_CODE.equals(prefix) && ival.compareTo(BigInteger.valueOf(2)) == 0) {
            return Boolean.valueOf(true);
        }
        if (STR_CODE.equals(prefix)) {
            bi = new BigInteger(value);
            bi = bi.subtract(BigInteger.valueOf(Integer.MAX_VALUE));
            return new String(bi.toByteArray());
        }
        if (BIGINT_CODE.equals(prefix)) {
            return new BigInteger(value);
        }
        return null;
    }
}
