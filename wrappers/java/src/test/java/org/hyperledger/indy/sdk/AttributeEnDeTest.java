package org.hyperledger.indy.sdk;

import org.junit.Test;

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
        assertEquals("19642085236040994", encoded);

        encoded  = AttributeEnDe.encode("Bob");
        assertEquals("1149290967586", encoded);

        encoded  = AttributeEnDe.encode("J.R. \"Bob\" Dobbs");
        assertEquals("1195759976482321268426281255675417640214603133730", encoded);

        encoded  = AttributeEnDe.encode("");
        assertEquals("12147483648", encoded);

        encoded = (String)AttributeEnDe.decode("19642085236040994");
        assertEquals("Alice", encoded);

        encoded = (String)AttributeEnDe.decode("1149290967586");
        assertEquals("Bob", encoded);

        encoded = (String)AttributeEnDe.decode("1195759976482321268426281255675417640214603133730");
        assertEquals("J.R. \"Bob\" Dobbs", encoded);
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

    @Test
    public void testFloat() {
/*
  (float)(0.0) -> 456284244423472 -> (float)(0.0)
  (float)(0.1) -> 456284244423473 -> (float)(0.1)
  (float)(-0.1) -> 43631083483811885873 -> (float)(-0.1)
  (float)(-1.9234856120348166e+37) -> 4118346363103572376150257369462524638568181085458527748347795511114206373863808144206434440804224330543005905719 -> (float)(-1.9234856120348166e+37)
  (float)(1.9234856120348166e+37) -> 41834518484686151139011264936952423078332017444442362935927001841153457642328110021483463741845116787307319 -> (float)(1.9234856120348166e+37)
*/
        String encoded;

        encoded = AttributeEnDe.encode(0d);
        assertEquals("52150641200", encoded);
        assertEquals(0d, AttributeEnDe.decode(encoded));

        encoded = AttributeEnDe.encode(0.1);
        assertEquals("456284244423473", encoded);
        assertEquals(0.1, AttributeEnDe.decode(encoded));

        encoded = AttributeEnDe.encode(-0.1d);
        assertEquals("43631083483811885873", encoded);
        assertEquals(-0.1d, AttributeEnDe.decode(encoded));

        encoded = AttributeEnDe.encode(-1.9234856120348166e+37);
        assertEquals(-1.9234856120348166e+37, AttributeEnDe.decode(encoded));
        assertEquals("4118346363103572376150257369462524638568181085458527748347795511114206373863808144206434440804224330543005905719", encoded);

        encoded = AttributeEnDe.encode(1.9234856120348166e+37);
        assertEquals(1.9234856120348166e+37, AttributeEnDe.decode(encoded));
        assertEquals("41834518484686151139011264936952423078332017444442362935927001841153457642328110021483463741845116787307319", encoded);


    }
    /*

    test cases by sklump

    == Edge cases - (type) orig -> encoded -> (type) decoded:
  (int)(2147483648) -> 3292278026040422511789490897800973956589404042040 -> (int)(2147483648)
  (int)(2147483649) -> 3292278026040422511789490897800973956589404042041 -> (int)(2147483649)
  (int)(-2147483649) -> 318853663399594688067339323832937851927518119208432441 -> (int)(-2147483649)
  (int)(-2147483648) -> -2147483648 -> (int)(-2147483648)
  (int)(-2147483647) -> -2147483647 -> (int)(-2147483647)
  (str)(0.0) -> 156284244423472 -> (str)(0.0)
  (str)(Hello) -> 1246599980865402989393510 -> (str)(Hello)
  (str)() -> 12147483648 -> (str)()
  (str)(Enjoy the process) -> 11547585876377423137304076650266942085095821252384200430752202418975662793106339635 -> (str)(Enjoy the process)
  (str)(True) -> 13833749873760745013 -> (str)(True)
  (str)(False) -> 1246563086251355677079093 -> (str)(False)
  (str)(1234) -> 13688785862641005364 -> (str)(1234)
  (str)(-12345) -> 115595384821298869659296346933 -> (str)(-12345)
  (list)([]) -> 93043112292 -> (str)([])
  (list)([0, 1, 2, 3]) -> 91308961905647194589628906076081553261331542782599924757860 -> (str)([0, 1, 2, 3])
     */
}