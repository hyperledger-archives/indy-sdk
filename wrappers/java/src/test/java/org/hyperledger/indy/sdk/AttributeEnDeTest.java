package org.hyperledger.indy.sdk;

import org.junit.Test;

import java.math.BigInteger;

import static org.junit.Assert.*;

public class AttributeEnDeTest {

    @Test
    public void testIntEncode() throws Exception {
        String encoded = AttributeEnDe.encode(Integer.valueOf(0));
        assertEquals("0", encoded);
        encoded = AttributeEnDe.encode(Integer.valueOf(1));
        assertEquals("1", encoded);

        encoded = AttributeEnDe.encode(Integer.MAX_VALUE);
        assertEquals("2147483647", encoded);
        encoded = AttributeEnDe.encode(Integer.MIN_VALUE);
        assertEquals("-2147483648", encoded);
    }

    @Test
    public void testIntDecode() throws Exception {
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
        assertEquals("13245106551", encodedAlex);
        assertTrue(new BigInteger(encodedAlex).compareTo(BigInteger.valueOf(Integer.MAX_VALUE)) > 0);
        String encodedAlexander = AttributeEnDe.encode("Alexander");
        assertEquals("11206849146281871566193", encodedAlexander);
        assertTrue(new BigInteger(encodedAlex).compareTo(new BigInteger(encodedAlexander)) < 0);
    }

    @Test
    public void testStringDecode() throws Exception {
        String encoded = (String)AttributeEnDe.decode("13245106551");
        assertEquals("Alex", encoded);

        encoded = (String)AttributeEnDe.decode("11206849146281871566193");
        assertEquals("Alexander", encoded);
    }

}