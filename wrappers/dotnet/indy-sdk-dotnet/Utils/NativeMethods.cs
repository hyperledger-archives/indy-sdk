using System;
using System.Runtime.InteropServices;


namespace Hyperledger.Indy.Utils
{
    class NativeMethods
    {

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_default_logger(string level);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_logger(IntPtr context, LoggingEnabledDelegate enabled, LogMessageDelegate log, LogFlushDelegate flush);

        internal delegate void LoggingEnabledDelegate();
        internal delegate void LogMessageDelegate(IntPtr context, int level, string target, string message, string module_path, string file, int line);
        internal delegate void LogFlushDelegate();


    }
}
