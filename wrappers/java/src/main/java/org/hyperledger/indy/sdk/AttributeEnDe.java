package org.hyperledger.indy.sdk;

import com.sun.istack.internal.NotNull;

import java.math.BigDecimal;
import java.math.BigInteger;

public class AttributeEnDe {

    private static BigInteger I32_BOUND = new BigInteger("2147483648");

    private static String STR_CODE = "1";
    private static String BOOL_CODE = "2";
    private static String POSINT = "3";
    private static String NEGINT = "4";
    private static String FLOAT_CODE = "5";
    private static String JSON_CODE = "9";

    private static final String ENCODED_TRUE = BOOL_CODE + "2147483650";
    private static final String ENCODED_FALSE = BOOL_CODE + "2147483649";

    private final static char[] hexArray = "0123456789ABCDEF".toCharArray();

    /**
     * @param int number
     * @return encoded int attribute
     */
    public static String encode(int number) {
        return String.valueOf(number);
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
        if ((raw_value.compareTo(I32_BOUND) < 0) &&
                (raw_value.compareTo(BigInteger.valueOf(Integer.MIN_VALUE)) >= 0)) {
            return encode(raw_value.intValue());
        }
        if (raw_value.signum() < 0) {
            return NEGINT + raw_value.abs().toString();
        }
        return POSINT + raw_value.toString();
    }

    /**
     * @param raw_value
     * @return
     */
    public static String encode(@NotNull String raw_value) {
        // sklump special case
        if ("\u0000".equals(raw_value)) return "92475982747121758242";
        BigInteger bi = new BigInteger(1, raw_value.getBytes());
        bi = bi.add(I32_BOUND);
        return STR_CODE + bi.toString();
    }

    /**
     * @param raw_value
     * @return
     */
    public static String encode(BigDecimal raw_value) {
        byte[] bytes = raw_value.toString().getBytes();
        BigInteger bi = new BigInteger(1, bytes);
        bi = bi.add(I32_BOUND);
        return "5" + bi.toString();
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
        if ("92475982747121758242".equals(encoded)) return "\u0000";

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
            return new String(bi.toByteArray());
        }
        if (POSINT.equals(prefix)) {
            return new BigInteger(value);
        }
        if (NEGINT.equals(prefix)) {
            return new BigInteger(value).negate();
        }
        if (FLOAT_CODE.equals(prefix)) {
            bi = new BigInteger(value);
            bi = bi.subtract(I32_BOUND);
            byte[] bytes = bi.toByteArray();
            String floatStr = new String(bytes);
            return new BigDecimal(floatStr);
        }
        return null;
    }

}
