export const mirrorAbi = [
  {
    type: "function",
    name: "claimValue",
    inputs: [
      {
        name: "claimedId",
        type: "bytes32",
        internalType: "bytes32",
      },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "decoder",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "address",
      },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "executableBalanceTopUp",
    inputs: [
      {
        name: "value",
        type: "uint128",
        internalType: "uint128",
      },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "inheritor",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "address",
      },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "initialize",
    inputs: [
      {
        name: "initializer",
        type: "address",
        internalType: "address",
      },
      {
        name: "decoder",
        type: "address",
        internalType: "address",
      },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "nonce",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256",
      },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "performStateTransition",
    inputs: [
      {
        name: "transition",
        type: "tuple",
        internalType: "struct Gear.StateTransition",
        components: [
          {
            name: "actorId",
            type: "address",
            internalType: "address",
          },
          {
            name: "newStateHash",
            type: "bytes32",
            internalType: "bytes32",
          },
          {
            name: "inheritor",
            type: "address",
            internalType: "address",
          },
          {
            name: "valueToReceive",
            type: "uint128",
            internalType: "uint128",
          },
          {
            name: "valueClaims",
            type: "tuple[]",
            internalType: "struct Gear.ValueClaim[]",
            components: [
              {
                name: "messageId",
                type: "bytes32",
                internalType: "bytes32",
              },
              {
                name: "destination",
                type: "address",
                internalType: "address",
              },
              {
                name: "value",
                type: "uint128",
                internalType: "uint128",
              },
            ],
          },
          {
            name: "messages",
            type: "tuple[]",
            internalType: "struct Gear.Message[]",
            components: [
              {
                name: "id",
                type: "bytes32",
                internalType: "bytes32",
              },
              {
                name: "destination",
                type: "address",
                internalType: "address",
              },
              {
                name: "payload",
                type: "bytes",
                internalType: "bytes",
              },
              {
                name: "value",
                type: "uint128",
                internalType: "uint128",
              },
              {
                name: "replyDetails",
                type: "tuple",
                internalType: "struct Gear.ReplyDetails",
                components: [
                  {
                    name: "to",
                    type: "bytes32",
                    internalType: "bytes32",
                  },
                  {
                    name: "code",
                    type: "bytes4",
                    internalType: "bytes4",
                  },
                ],
              },
            ],
          },
        ],
      },
    ],
    outputs: [
      {
        name: "",
        type: "bytes32",
        internalType: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "router",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "address",
      },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "sendMessage",
    inputs: [
      {
        name: "payload",
        type: "bytes",
        internalType: "bytes",
      },
      {
        name: "value",
        type: "uint128",
        internalType: "uint128",
      },
    ],
    outputs: [
      {
        name: "",
        type: "bytes32",
        internalType: "bytes32",
      },
    ],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "sendReply",
    inputs: [
      {
        name: "repliedTo",
        type: "bytes32",
        internalType: "bytes32",
      },
      {
        name: "payload",
        type: "bytes",
        internalType: "bytes",
      },
      {
        name: "value",
        type: "uint128",
        internalType: "uint128",
      },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "stateHash",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bytes32",
        internalType: "bytes32",
      },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "transferLockedValueToInheritor",
    inputs: [],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "event",
    name: "ExecutableBalanceTopUpRequested",
    inputs: [
      {
        name: "value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "Message",
    inputs: [
      {
        name: "id",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "destination",
        type: "address",
        indexed: true,
        internalType: "address",
      },
      {
        name: "payload",
        type: "bytes",
        indexed: false,
        internalType: "bytes",
      },
      {
        name: "value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "MessageQueueingRequested",
    inputs: [
      {
        name: "id",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "source",
        type: "address",
        indexed: true,
        internalType: "address",
      },
      {
        name: "payload",
        type: "bytes",
        indexed: false,
        internalType: "bytes",
      },
      {
        name: "value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "Reply",
    inputs: [
      {
        name: "payload",
        type: "bytes",
        indexed: false,
        internalType: "bytes",
      },
      {
        name: "value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
      {
        name: "replyTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "replyCode",
        type: "bytes4",
        indexed: true,
        internalType: "bytes4",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "ReplyQueueingRequested",
    inputs: [
      {
        name: "repliedTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "source",
        type: "address",
        indexed: true,
        internalType: "address",
      },
      {
        name: "payload",
        type: "bytes",
        indexed: false,
        internalType: "bytes",
      },
      {
        name: "value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "StateChanged",
    inputs: [
      {
        name: "stateHash",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "ValueClaimed",
    inputs: [
      {
        name: "claimedId",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "ValueClaimingRequested",
    inputs: [
      {
        name: "claimedId",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "source",
        type: "address",
        indexed: true,
        internalType: "address",
      },
    ],
    anonymous: false,
  },
];
