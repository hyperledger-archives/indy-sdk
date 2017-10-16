using System.IO;

namespace Hyperledger.Indy.Test
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
            var tmpDir = EnvironmentUtils.GetTmpPath();
            var homeDir = EnvironmentUtils.GetIndyHomePath();

            CleanDirectory(tmpDir);
            CleanDirectory(homeDir);
        }        
    }
}
