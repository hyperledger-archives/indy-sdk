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

    @Test
    public void testFloat() {
        String encoded;

        encoded = AttributeEnDe.encode(0d);
        assertEquals("513642949358284848", encoded);
        assertEquals(0d, AttributeEnDe.decode(encoded));

        encoded = AttributeEnDe.encode(0.1);
        assertEquals("5276711930654430623867169382163965112486145830196", encoded);
        assertEquals(0.1, AttributeEnDe.decode(encoded));

        encoded = AttributeEnDe.encode(-0.1d);
        assertEquals("566044285610545061943032986854396700997003110264116", encoded);
        assertEquals(-0.1d, AttributeEnDe.decode(encoded));

        encoded = AttributeEnDe.encode(-1.9234856120348166e+37);
        assertEquals(-1.9234856120348166e+37, AttributeEnDe.decode(encoded));
        assertEquals("516907337116313887486272534785745044898174533144490547", encoded);

        encoded = AttributeEnDe.encode(1.9234856120348166e+37);
        assertEquals(1.9234856120348166e+37, AttributeEnDe.decode(encoded));
        assertEquals("570838254261885868566085512853464511738190249407027", encoded);
    }


/*

    test cases by sklump

(type) orig -> encoded -> (type) decoded:
  (str)(0x00) -> 92475982747121758242 -> (str)(0x00)
  (str)(0x01) -> 12147483649 -> (str)(0x01)
  (str)(0x02) -> 12147483650 -> (str)(0x02)
  (str)(Alice) -> 1283139203941 -> (str)(Alice)
  (str)(Bob) -> 12151837538 -> (str)(Bob)
  (str)(J.R. "Bob" Dobbs) -> 198603384155604289281926069284365296243 -> (str)(J.R. "Bob" Dobbs)
  (NoneType)(None) -> 2147483648 -> (NoneType)(None)
  (bool)(True) -> 22147483650 -> (bool)(True)
  (bool)(False) -> 22147483649 -> (bool)(False)
  (int)(-5) -> -5 -> (int)(-5)
  (int)(0) -> 0 -> (int)(0)
  (int)(1024) -> 1024 -> (int)(1024)
  (int)(2147483647) -> 2147483647 -> (int)(2147483647)
  (int)(2147483648) -> 32147483648 -> (int)(2147483648)
  (int)(2147483649) -> 32147483649 -> (int)(2147483649)
  (int)(-2147483649) -> 42147483649 -> (int)(-2147483649)
  (int)(-2147483648) -> -2147483648 -> (int)(-2147483648)
  (int)(-2147483647) -> -2147483647 -> (int)(-2147483647)
  (float)(0.0) -> 52150641200 -> (float)(0.0)
  (str)(0.0) -> 12150641200 -> (str)(0.0)
  (float)(0.1) -> 52150641201 -> (float)(0.1)
  (float)(-0.1) -> 52905615921 -> (float)(-0.1)
  (float)(-1.9234856120348166e+37) -> 54328544340831280174737077750937923315076594647320703799 -> (float)(-1.9234856120348166e+37)
  (float)(1.9234856120348166e+37) -> 518400632145967760604226737077678736148890866179322679 -> (float)(1.9234856120348166e+37)
  (str)(Hello) -> 1313086733423 -> (str)(Hello)
  (str)() -> 12147483648 -> (str)()
  (str)(True) -> 13564270949 -> (str)(True)
  (str)(False) -> 1304429691749 -> (str)(False)
  (str)(1234) -> 12972857140 -> (str)(1234)
  (str)(-12345) -> 149691466347573 -> (str)(-12345)
  (list)([]) -> 92147507037 -> (list)([])
  (list)([0, 1, 2, 3]) -> 928221372711048202486278009693 -> (list)([0, 1, 2, 3])
  (dict)({'a': 1, 'b': 2, 'c': 3}) -> 93019244119479962529376486787157177996876144122637435548541 -> (dict)({'a': 1, 'b': 2, 'c': 3})
  (list)([{}, {'a': [0, 1], 'b': [2, 3, 4]}, True]) -> 9195405174979923098884988550719541473293292774988805351916090378761664927737665970773195426373133661 -> (list)([{}, {'a': [0, 1], 'b': [2, 3, 4]}, True])
*/
}