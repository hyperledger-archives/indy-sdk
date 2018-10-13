using Microsoft.VisualStudio.TestTools.UnitTesting;
using NLog;
using NLog.Config;
using NLog.Targets;

namespace Hyperledger.Indy.Test
{
    [TestClass]
    public class SetupAssemblyInitializer
    {
        const int PROTOCOL_VERSION = 2;

        [AssemblyInitialize]
        public static void AssemblyInit(TestContext context)
        {
            //Initialization code goes here.
            var config = new LoggingConfiguration();

            // Step 2. Create targets
            var consoleTarget = new ColoredConsoleTarget("target1")
            {
                Layout = @"${date:format=HH\:mm\:ss} ${level} ${message} ${exception}"
            };
            consoleTarget.DetectConsoleAvailable = false;
            config.AddTarget(consoleTarget);

            config.AddRuleForAllLevels(consoleTarget); // all to console

            // Step 4. Activate the configuration
            LogManager.Configuration = config;

            Hyperledger.Indy.Utils.Logger.Init();
            StorageUtils.CleanupStorage();
        }

        [AssemblyCleanup]
        public static void AssemblyCleanup()
        {
            //Cleanup code goes here.
            StorageUtils.CleanupStorage();
        }
    }
}
