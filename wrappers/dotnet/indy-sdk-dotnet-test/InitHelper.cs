using System.Runtime.InteropServices;

namespace Indy.Sdk.Dotnet.Test
{
    class InitHelper
    {
        [DllImport("kernel32.dll", SetLastError = true)]
        static extern bool SetDllDirectory(string lpPathName);

        public static void Init()
        {
            SetDllDirectory("../../../../../target/debug/");      
        }
    }
}
