class Exit(Exception):
    pass


class CliError(Exception):
    def __init__(self, message):
        super(CliError, self).__init__(message)


def indy_error_to_message(err_code):
    errors = {
        100: 'Passed invalid value as param 1',
        101: 'Passed invalid value as param 2',
        102: 'Passed invalid value as param 3',
        103: 'Passed invalid value as param 4',
        104: 'Passed invalid value as param 5',
        105: 'Passed invalid value as param 6',
        106: 'Passed invalid value as param 7',
        107: 'Passed invalid value as param 8',
        108: 'Passed invalid value as param 9',
        109: 'Passed invalid value as param 10',
        110: 'Passed invalid value as param 11',
        111: 'Passed invalid value as param 12',
        112: 'Library invalid state.',
        113: 'Passed invalid parameter.',
        114: 'IO Error',
        200: 'Passed invalid wallet handle',
        201: 'Passed unknown type of wallet',
        202: 'Attempt to register already existing wallet type',
        203: 'Attempt to create wallet with name used for another exists wallet',
        204: 'Object not found',
        205: 'Wallet and Pool don\'t correspond',
        206: 'Wallet already opened already',
        300: 'Pool Ledger not created',
        301: 'Can not connect to pool ledger',
        302: 'Pool Ledger terminated',
        303: 'No concensus during ledger operation',
        304: 'Invalid transaction message',
        305: 'Attempt to send transaction without the necessary privileges',
        306: 'Pool ledger config with already existing pool',
        400: 'Revocation registry is full and creation of new registry is necessary',
        401: 'Invalid user revocation index',
        402: 'Accumulator is full',
        403: 'Claim not issued',
        404: 'Attempt to generate master secret with duplicated name',
        405: 'Proof Rejected',
        500: 'Unknown crypto type'
    }
    return 'ERROR: ' + errors[err_code]
