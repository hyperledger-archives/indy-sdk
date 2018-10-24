package org.hyperledger.indy.sdk;

import org.json.JSONArray;
import org.json.JSONObject;

public class JsonObjectSimilar {

    /**
     * Determine if two JSONObjects are similar.
     * They must contain the same set of names which must be associated with
     * similar values.
     *
     * @param other The other JSONObject
     * @return true if they are equal
     */
    public static boolean similar(JSONObject me, Object other) {
        try {
            return other instanceof JSONObject && similar(me, (JSONObject) other);
        } catch (Throwable exception) {
            return false;
        }
    }

    public static boolean similar(JSONObject me, JSONObject other) {
        try {
            if (!me.keySet().equals(other.keySet())) {
                return false;
            }
            for (final String name : me.keySet()) {
                Object valueThis = me.get(name);
                Object valueOther = other.get(name);
                if(valueThis == valueOther) {
                	continue;
                }
                if(valueThis == null) {
                	return false;
                }
                if (valueThis instanceof JSONObject) {
                    if (!similar((JSONObject)valueThis, valueOther)) {
                        return false;
                    }
                } else if (valueThis instanceof JSONArray) {
                    if (!similar((JSONArray)valueThis, valueOther)) {
                        return false;
                    }
                } else if (!valueThis.equals(valueOther)) {
                    return false;
                }
            }
            return true;
        } catch (Throwable exception) {
            return false;
        }
    }
    /**
     * Determine if two JSONArrays are similar.
     * They must contain similar sequences.
     *
     * @param other The other JSONArray
     * @return true if they are equal
     */
    private static boolean similar(JSONArray me, Object other) {
        return other instanceof JSONArray && similar(me, (JSONArray) other);
    }

    private static boolean similar(JSONArray me, JSONArray other) {
        int len = me.length();
        if (len != other.length()) {
            return false;
        }
        for (int i = 0; i < len; i += 1) {
            Object valueThis = me.get(i);
            Object valueOther = other.get(i);
            if(valueThis == valueOther) {
            	continue;
            }
            if(valueThis == null) {
            	return false;
            }
            if (valueThis instanceof JSONObject) {
                if (!((JSONObject)valueThis).similar(valueOther)) {
                    return false;
                }
            } else if (valueThis instanceof JSONArray) {
                if (!similar((JSONArray)valueThis, valueOther)) {
                    return false;
                }
            } else if (!valueThis.equals(valueOther)) {
                return false;
            }
        }
        return true;
    }

}
