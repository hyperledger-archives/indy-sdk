using System;
using System.IO;

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
            return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile));
        }

        public static string GetIndyHomePath()
        {
            return Path.Combine(GetUserHomePath(), ".indy");
        }

        public static string getIndyHomePath(string filename)
        {
            return GetIndyHomePath() + filename;
        }

        public static string GetTmpPath()
        {
            return Path.Combine(Path.GetTempPath(), "indy");
        }

        public static string GetTmpPath(string filename)
        {
            return GetTmpPath() + filename;
        }

    }
}
