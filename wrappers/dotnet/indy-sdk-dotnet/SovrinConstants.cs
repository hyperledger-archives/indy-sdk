using System;

namespace Indy.Sdk.Dotnet
{
    /// <summary>
    /// Constants used in Sovrin.
    /// </summary>
    public static class SovrinConstants
    {
        public const String ROLE_TRUSTEE = "TRUSTEE";
	    public const String ROLE_STEWARD = "STEWARD";

	    public const String OP_NODE = "0";
	    public const String OP_NYM = "1";
	    public const String OP_ATTRIB = "100";
	    public const String OP_SCHEMA = "101";
	    public const String OP_CLAIM_DEF = "102";
	    public const String OP_GET_ATTR = "104";
	    public const String OP_GET_NYM = "105";
	    public const String OP_GET_SCHEMA = "107";
	    public const String OP_GET_CLAIM_DEF = "108";
    }
}
