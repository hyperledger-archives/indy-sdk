package org.hyperledger.indy.sdk;

import com.sun.istack.internal.NotNull;

import java.math.BigInteger;

public class AttributeEnDe {

    private static BigInteger I32_BOUND = new BigInteger("2147483648");

    private static String STR_CODE = "1";
    private static String BOOL_CODE = "2";
    private static String BIGINT_CODE = "3";
    private static String FLOAT_CODE = "4";
    private static String NONE_CODE = "9";

    private static final String ENCODED_TRUE = BOOL_CODE + "2147483650";
    private static final String ENCODED_FALSE = BOOL_CODE + "2147483649";

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
        String hex = bytesToHex(raw_value.getBytes());
        byte[] bytes = hex.getBytes();
        BigInteger bi = new BigInteger(1, bytes);
        bi = bi.add(I32_BOUND);
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
        if ("22147483650".equals(encoded)) return Boolean.TRUE;
        if ("22147483649".equals(encoded)) return Boolean.FALSE;

        BigInteger bi = new BigInteger(encoded);
        if ((BigInteger.valueOf(Integer.MIN_VALUE).compareTo(bi) <= 0)
                && (bi.compareTo(I32_BOUND) < 0)) {
            return new Integer(bi.intValue());
        }

        String prefix = encoded.substring(0, 1);
        String value = encoded.substring(1);
        BigInteger ival = new BigInteger(value, 16);
        ival = ival.subtract(I32_BOUND);

        if (ival.compareTo(BigInteger.ZERO) == 0) {
            return "";
        }
        if (STR_CODE.equals(prefix)) {
            bi = new BigInteger(value);
            bi = bi.subtract(I32_BOUND);
            byte[] bytes = bi.toByteArray();
            if (bytes.length % 2 != 0) {
                throw new IllegalArgumentException("Encoded value does not decode to an even number of UTF-8 characters");
            }
            StringBuffer rv = new StringBuffer();
            for (int j = 0; j < bytes.length / 2; j++) { // unhexlify
                int top = Character.digit(bytes[2 * j], 16);
                int bot = Character.digit(bytes[2 * j + 1], 16);
                rv.append((char)((top << 4) + bot));
            }
            return rv.toString();
        }
        if (BIGINT_CODE.equals(prefix)) {
            return new BigInteger(value);
        }
        return null;
    }

}
