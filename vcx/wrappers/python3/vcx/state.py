from enum import IntEnum


class State(IntEnum):
    Undefined = 0,
    Initialized = 1,
    OfferSent = 2,
    RequestReceived = 3,
    Accepted = 4,
    Unfulfilled = 5,
    Expired = 6,
    Revoked = 7,
    Redirected = 8,
    Rejected = 9,


class ProofState(IntEnum):
    Undefined = 0,
    Verified = 1,
    Invalid = 2


class PublicEntityState(IntEnum):
    Built = 0,
    Published = 1
