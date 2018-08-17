package org.hyperledger.indy.sdk;

import org.junit.Test;

import java.math.BigDecimal;
import java.math.BigInteger;

import static org.junit.Assert.*;

public class AttributeEnDeTest {

    @Test
    public void test() {
        BigInteger bi = new BigInteger("Python Lake".getBytes());
        assertEquals("97287619261264698714188645", bi.toString());
    }

    @Test
    public void testIntEncode() {
        String encoded = AttributeEnDe.encode(Integer.valueOf(0));
        assertEquals("0", encoded);
        encoded = AttributeEnDe.encode(Integer.valueOf(1));
        assertEquals("1", encoded);

        encoded = AttributeEnDe.encode(Integer.MAX_VALUE);
        assertEquals("2147483647", encoded);
        encoded = AttributeEnDe.encode(-2147483647);
        assertEquals("-2147483647", encoded);
        encoded = AttributeEnDe.encode(Integer.MIN_VALUE);
        assertEquals("-2147483648", encoded);

        encoded = AttributeEnDe.encode(-5);
        assertEquals("-5", encoded);

        encoded = AttributeEnDe.encode(1024);
        assertEquals("1024", encoded);
    }

    @Test
    public void testIntDecode() {
        Integer decoded = (Integer) AttributeEnDe.decode("0");
        assertEquals(Integer.valueOf(0), decoded);
        decoded = (Integer) AttributeEnDe.decode("1");
        assertEquals(Integer.valueOf(1), decoded);

        decoded = (Integer) AttributeEnDe.decode("2147483646");
        assertTrue(decoded.compareTo(Integer.MAX_VALUE - 1) == 0);

        decoded = (Integer) AttributeEnDe.decode("-2147483648");
        assertTrue(decoded.compareTo(Integer.MIN_VALUE) == 0);

        decoded = (Integer) AttributeEnDe.decode("-5");
        assertTrue(decoded.compareTo(Integer.valueOf(-5)) == 0);

        decoded = (Integer) AttributeEnDe.decode("1024");
        assertTrue(decoded.compareTo(Integer.valueOf(1024)) == 0);

        decoded = (Integer) AttributeEnDe.decode("-2147483647");
        assertTrue(decoded.compareTo(-2147483647) == 0);
    }

    @Test
    public void testNullEncode() throws Exception {
        String encoded = AttributeEnDe.encode();
        assertEquals("2147483647", encoded);
    }

    @Test
    public void testNullDecode() throws Exception {
        Object encoded = AttributeEnDe.decode("2147483647");
        assertEquals(null, encoded);
    }

    @Test
    public void testStringEncode() throws Exception {
        String encoded;

        encoded = AttributeEnDe.encode("Alice");
        assertEquals("1283139203941", encoded);

        encoded  = AttributeEnDe.encode("Bob");
        assertEquals("12151837538", encoded);

        encoded  = AttributeEnDe.encode("J.R. \"Bob\" Dobbs");
        assertEquals("198603384155604289281926069284365296243", encoded);

        encoded  = AttributeEnDe.encode("");
        assertEquals("12147483648", encoded);

        encoded = (String)AttributeEnDe.decode("1283139203941");
        assertEquals("Alice", encoded);

        encoded = (String)AttributeEnDe.decode("12151837538");
        assertEquals("Bob", encoded);

        encoded = (String)AttributeEnDe.decode("198603384155604289281926069284365296243");
        assertEquals("J.R. \"Bob\" Dobbs", encoded);

        encoded = AttributeEnDe.encode("True");
        assertEquals("13564270949", encoded);
        encoded = (String)AttributeEnDe.decode("13564270949");
        assertEquals("True", encoded);

        encoded = AttributeEnDe.encode("False");
        assertEquals("1304429691749", encoded);
        encoded = (String)AttributeEnDe.decode("1304429691749");
        assertEquals("False", encoded);

        encoded = AttributeEnDe.encode("1234");
        assertEquals("12972857140", encoded);
        encoded = (String)AttributeEnDe.decode("12972857140");
        assertEquals("1234", encoded);

        encoded = AttributeEnDe.encode("-12345");
        assertEquals("149691466347573", encoded);
        encoded = (String)AttributeEnDe.decode("149691466347573");
        assertEquals("-12345", encoded);

        encoded = AttributeEnDe.encode("\u0000");
        assertEquals("92475982747121758242", encoded);
        encoded = (String)AttributeEnDe.decode(encoded);
        assertEquals("\u0000", encoded);

        encoded = AttributeEnDe.encode("\u0001");
        assertEquals("12147483649", encoded);
        encoded = (String)AttributeEnDe.decode(encoded);
        assertEquals("\u0001", encoded);

        encoded = AttributeEnDe.encode("\u0002");
        assertEquals("12147483650", encoded);
        encoded = (String)AttributeEnDe.decode(encoded);
        assertEquals("\u0002", encoded);

    }

    @Test
    public void testBool() {
        String encoded = AttributeEnDe.encode(true);
        assertEquals("22147483650", encoded);
        Boolean t = (Boolean)AttributeEnDe.decode(encoded);
        assertTrue(t.booleanValue());

        encoded = AttributeEnDe.encode(false);
        assertEquals("22147483649", encoded);
        Boolean f = (Boolean)AttributeEnDe.decode(encoded);
        assertFalse(f.booleanValue());
    }

    @Test
    public void testBigInteger() {
/*
        (int)(2147483648) -> 3292278026040422511789490897800973956589404042040 -> (int)(2147483648)
        (int)(2147483649) -> 3292278026040422511789490897800973956589404042041 -> (int)(2147483649)
        (int)(-2147483649) -> 318853663399594688067339323832937851927518119208432441 -> (int)(-2147483649)



        String encoded;

        BigInteger bi = new BigInteger("2147483648");
        assertEquals("2147483648", bi.toString());
        encoded = AttributeEnDe.encode(bi);
        assertEquals("3292278026040422511789490897800973956589404042040", encoded);
*/

    }

    private final static char[] hexArray = "0123456789ABCDEF".toCharArray();
    public static String bytesToHex(byte[] bytes) {
        char[] hexChars = new char[bytes.length * 2];
        for ( int j = 0; j < bytes.length; j++ ) {
            int v = bytes[j] & 0xFF;
            hexChars[j * 2] = hexArray[v >>> 4];
            hexChars[j * 2 + 1] = hexArray[v & 0x0F];
        }
        return new String(hexChars);
    }

    private void aDecimalTest(BigDecimal orig, String expectedEncoding) {
        String encoded;
        BigDecimal decoded;

        encoded = AttributeEnDe.encode(orig);
        assertEquals(expectedEncoding, encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));
    }

    @Test
    public void testDecimal() {
        String encoded;
        BigDecimal orig;
        BigDecimal decoded;

        //(Decimal)(-1.9234856120348166E+37) -> 54328544340831280174737077750937923315076594646783832887 -> (Decimal)(-1.9234856120348166E+37)
        orig = new BigDecimal(-1.9234856120348166E+37);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        orig = new BigDecimal(0d);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("52147483696", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        encoded = AttributeEnDe.encode(new BigDecimal("0.1"));
        assertEquals("52150641201", encoded);
        assertEquals(BigDecimal.valueOf(0.1), AttributeEnDe.decode(encoded));

        encoded = AttributeEnDe.encode(new BigDecimal("-0.1"));
        assertEquals("52905615921", encoded);
        assertEquals(BigDecimal.valueOf(-0.1d), AttributeEnDe.decode(encoded));

        //(Decimal)(-1.9234856120348166E+37) -> 54328544340831280174737077750937923315076594646783832887 -> (Decimal)(-1.9234856120348166E+37)
        orig = new BigDecimal(-19234856120348165827208446428657483776d);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        orig = new BigDecimal(1.7976931348623157e+308);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("5269161618268042360145972004017465694160335431035209070264743835985788283147221721580935312561110946133507975343508594859652344770724128074847981685487152443622741405518894752003899242539047325884518363202478638322310509556173992084275874584046909927658401011484944150045656452585156807624649491085747233252439449003263586096402086333577995530944069599365921765556791796837319741044157258139053932589350484934760974950877213300639056696370692524570936673598361670837667621686283336687753958203404085519417100796390887548426184699350466891653483118820866871180793923062728988519292524483654481567674008917683910310359425535500424285508251141056820528030169509698440710321678537647941733488238720665247624257819334169035761346284186696393912694328", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        orig = new BigDecimal(-19234856120348165827208446428657483776d);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        orig = new BigDecimal(-1.9234856120348166e+37);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        orig = new BigDecimal(-19234856120348165827208446428657483776d);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        orig = new BigDecimal(-19234856120348166000000000000000000000d);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        orig = new BigDecimal(-19234856120348165827208446428657483776d);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        encoded = AttributeEnDe.encode(new BigDecimal(19234856120348165827208446428657483776d));
        assertEquals(new BigDecimal(19234856120348165827208446428657483776d), AttributeEnDe.decode(encoded));
        assertEquals("56266867624008333854602678985183808700155394679689609995288176626428057309994108167931770678", encoded);

        orig = new BigDecimal(-19234856120348165827208446428657483776d);
        encoded = AttributeEnDe.encode(orig);
        assertEquals("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398", encoded);
        decoded = (BigDecimal)AttributeEnDe.decode(encoded);
        assertEquals(0, decoded.compareTo(orig));

        aDecimalTest(BigDecimal.valueOf(0.1), "52150641201");
        aDecimalTest(BigDecimal.valueOf(-0.1), "52905615921");
        aDecimalTest(BigDecimal.valueOf(1.9234856120348166E+37), "518400632145967760604226737077678736148890865642451767");

        decoded = (BigDecimal)AttributeEnDe.decode("51472932770584838315967883574639936084657212638119163710453309300161582593139596536619977881398");
        assertEquals(new BigDecimal("-19234856120348165827208446428657483776"), decoded);

    }

}