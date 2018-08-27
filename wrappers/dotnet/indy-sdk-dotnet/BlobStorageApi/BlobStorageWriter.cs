namespace Hyperledger.Indy.BlobStorageApi
{
    /// <summary>
    /// BLOB storage writer.
    /// </summary>
    public class BlobStorageWriter
    {
        internal BlobStorageWriter(int handle)
		{
			Handle = handle;
		}

		internal int Handle { get; }
    }
}
