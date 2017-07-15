using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test
{
    class StorageUtils
    {
        private static void CleanDirectory(string path)
        {
            if (Directory.Exists(path))
            {
                Directory.Delete(path, true);
            }
        }

        public static void CleanupStorage()
        {
            string tmpDir = GetTmpPath();
            string homeDir = GetIndyHomePath();

            CleanDirectory(tmpDir);
            CleanDirectory(homeDir);
        }

        public static String GetIndyHomePath()
        {
            return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".indy");
        }

        public static String GetIndyHomePath(string filename)
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
