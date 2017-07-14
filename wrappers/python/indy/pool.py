async def create_pool_ledger_config(command_handle: int,
                                    config_name: str,
                                    config: str) -> None:
    pass


async def open_pool_ledger(command_handle: int,
                           config_name: str,
                           config: str,
                           pool_handle: int) -> None:
    pass


async def refresh_pool_ledger(command_handle: int,
                              handle: int) -> None:
    pass


async def close_pool_ledger(command_handle: int,
                            handle: int) -> None:
    pass


async def delete_pool_ledger_config(command_handle: int,
                                    config_name: str) -> None:
    pass
