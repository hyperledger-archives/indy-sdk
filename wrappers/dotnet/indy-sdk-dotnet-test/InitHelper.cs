using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

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
