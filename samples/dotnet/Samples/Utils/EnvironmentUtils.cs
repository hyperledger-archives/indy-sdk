using System;
using System.IO;

namespace Hyperledger.Indy.Samples.Utils
{
    static class EnvironmentUtils
    {
        public static string GetTestPoolIP()
        {
            var testPoolIp = Environment.GetEnvironmentVariable("TEST_POOL_IP");
            return testPoolIp != null ? testPoolIp : "127.0.0.1";
        }

        public static string GetTmpPath()
        {
            return Path.Combine(Path.GetTempPath(), "indy");
        }

        public static string GetUserPath()
        {
            return Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        }

        public static string GetIndyHomePath()
        {
            return Path.Combine(GetUserPath(), ".indy_client");
        }

        public static string GetIndyHomePath(string filename)
        {
            return Path.Combine(GetIndyHomePath(), filename);
        }

        public static string GetTmpPath(string filename)
        {
            return GetTmpPath() + filename;
        }
    }
}
