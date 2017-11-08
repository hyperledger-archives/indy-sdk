using System;
using System.IO;
using System.Runtime.InteropServices;

namespace Hyperledger.Indy.Test
{
    static class EnvironmentUtils
    {
        public static string GetTestPoolIP()
        {
            var testPoolIp = Environment.GetEnvironmentVariable("TEST_POOL_IP");
            return testPoolIp != null ? testPoolIp : "127.0.0.1";
        }

        public static string GetUserHomePath()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux) || RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
                return Environment.GetEnvironmentVariable("HOME");
            else
                return Environment.GetEnvironmentVariable("USERPROFILE");
        }

        public static string GetIndyHomePath()
        {
            return Path.Combine(GetUserHomePath(), ".indy_client");
        }

        public static string GetIndyHomePath(string filename)
        {
            return Path.Combine(GetIndyHomePath(), filename);
        }

        public static string GetTmpPath()
        {
            return Path.Combine(Path.GetTempPath(), "indy_client");
        }

        public static string GetTmpPath(string filename)
        {
            return Path.Combine(GetTmpPath(), filename);
        }

    }
}
