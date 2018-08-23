namespace Hyperledger.Indy
{
    /// <summary>
    /// PInvoke import of C-Callable SDK library functions and associated delegates.
    /// </summary>
    internal static class Consts
    {
#if __IOS__
        public const string NATIVE_LIB_NAME = "__Internal";
#else
        public const string NATIVE_LIB_NAME = "indy";
#endif
    }
}