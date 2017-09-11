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
            string tmpDir = EnvironmentUtils.GetTmpPath();
            string homeDir = EnvironmentUtils.GetIndyHomePath();

            CleanDirectory(tmpDir);
            CleanDirectory(homeDir);
        }        
    }
}
