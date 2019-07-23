from typing import Union

from .._command import LibindyCommand


class BlobStorage:

    @staticmethod
    @LibindyCommand('indy_open_blob_storage_reader')
    async def open_blob_storage_reader(
            storage_reader_type: str,
            storage_reader_config: Union[dict, str]
    ) -> int:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_open_blob_storage_writer')
    async def open_blob_storage_writer(
            storage_writer_type: str,
            storage_writer_config: Union[dict, str]
    ) -> int:
        """"""
        pass
