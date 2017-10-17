"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var StateType;
(function (StateType) {
    StateType[StateType["None"] = 0] = "None";
    StateType[StateType["Initialized"] = 1] = "Initialized";
    StateType[StateType["OfferSent"] = 2] = "OfferSent";
    StateType[StateType["RequestReceived"] = 3] = "RequestReceived";
    StateType[StateType["Accepted"] = 4] = "Accepted";
    StateType[StateType["Unfulfilled"] = 5] = "Unfulfilled";
    StateType[StateType["Expired"] = 6] = "Expired";
    StateType[StateType["Revoked"] = 7] = "Revoked";
})(StateType = exports.StateType || (exports.StateType = {}));
//# sourceMappingURL=api.js.map