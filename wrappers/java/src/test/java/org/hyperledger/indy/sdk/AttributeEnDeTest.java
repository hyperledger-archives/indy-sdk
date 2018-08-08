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
        assertNotNull("oops", decoded);
        assertTrue(decoded.compareTo(Integer.MAX_VALUE - 1) == 0);
        decoded = (Integer) AttributeEnDe.decode("-2147483648");
        assertTrue(decoded.compareTo(Integer.MIN_VALUE) == 0);
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
        String encodedAlex = AttributeEnDe.encode("Alex");
        assertEquals("13760846828278724408", encodedAlex);
        assertTrue(new BigInteger(encodedAlex).compareTo(BigInteger.valueOf(Integer.MAX_VALUE)) > 0);
        String encodedAlexander = AttributeEnDe.encode("Alexander");
        assertEquals("14546584831725787018466483923330011388786482", encodedAlexander);
        assertTrue(new BigInteger(encodedAlex).compareTo(new BigInteger(encodedAlexander)) < 0);

        String encoded;

/*
        encoded = AttributeEnDe.encode("Alice");
        assertEquals("1246470866604555559450165", encoded);

        encoded  = AttributeEnDe.encode("Bob");
        assertEquals("157392413161010", encoded);

        encoded  = AttributeEnDe.encode("J.R. \"Bob\" Dobbs");
        assertEquals("123692000107487509633306574080617894598073199214594302365883455161101326628659", encoded);
*/

        encoded  = AttributeEnDe.encode("");
        assertEquals("12147483648", encoded);
    }

    @Test
    public void testStringDecode() throws Exception {
        String encoded = (String)AttributeEnDe.decode("13760846828278724408");
        assertEquals("Alex", encoded);

        encoded = (String)AttributeEnDe.decode("14546584831725787018466483923330011388786482");
        assertEquals("Alexander", encoded);

/*
        encoded = (String)AttributeEnDe.decode("1246470866604555559450165");
        assertEquals("Alice", encoded);

        encoded = (String)AttributeEnDe.decode("157392413161010");
        assertEquals("Bob", encoded);

        encoded = (String)AttributeEnDe.decode("123692000107487509633306574080617894598073199214594302365883455161101326628659");
        assertEquals("J.R. \"Bob\" Dobbs", encoded);
*/
    }



    @Test
    public void testBool() throws Exception {
        String encoded = AttributeEnDe.encode(true);
        assertEquals("22147483650", encoded);
        Boolean t = (Boolean)AttributeEnDe.decode(encoded);
        assertTrue(t.booleanValue());

        encoded = AttributeEnDe.encode(false);
        assertEquals("22147483649", encoded);
        Boolean f = (Boolean)AttributeEnDe.decode(encoded);
        assertFalse(f.booleanValue());
    }

    /*

    test cases by sklump

    == Edge cases - (type) orig -> encoded -> (type) decoded:
  (str)(Alice) -> 1246470866604555559450165 -> (str)(Alice)
  (str)(Bob) -> 157392413161010 -> (str)(Bob)
  (str)(J.R. "Bob" Dobbs) -> 123692000107487509633306574080617894598073199214594302365883455161101326628659 -> (str)(J.R. "Bob" Dobbs)
  (NoneType)(None) -> 2147483648 -> (NoneType)(None)
  (bool)(True) -> 22147483650 -> (bool)(True)
  (bool)(False) -> 22147483649 -> (bool)(False)
  (int)(-5) -> -5 -> (int)(-5)
  (int)(0) -> 0 -> (int)(0)
  (int)(1024) -> 1024 -> (int)(1024)
  (int)(2147483647) -> 2147483647 -> (int)(2147483647)
  (int)(2147483648) -> 3292278026040422511789490897800973956589404042040 -> (int)(2147483648)
  (int)(2147483649) -> 3292278026040422511789490897800973956589404042041 -> (int)(2147483649)
  (int)(-2147483649) -> 318853663399594688067339323832937851927518119208432441 -> (int)(-2147483649)
  (int)(-2147483648) -> -2147483648 -> (int)(-2147483648)
  (int)(-2147483647) -> -2147483647 -> (int)(-2147483647)
  (float)(0.0) -> 456284244423472 -> (float)(0.0)
  (str)(0.0) -> 156284244423472 -> (str)(0.0)
  (float)(0.1) -> 456284244423473 -> (float)(0.1)
  (float)(-0.1) -> 43631083483811885873 -> (float)(-0.1)
  (float)(-1.9234856120348166e+37) -> 4118346363103572376150257369462524638568181085458527748347795511114206373863808144206434440804224330543005905719 -> (float)(-1.9234856120348166e+37)
  (float)(1.9234856120348166e+37) -> 41834518484686151139011264936952423078332017444442362935927001841153457642328110021483463741845116787307319 -> (float)(1.9234856120348166e+37)
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