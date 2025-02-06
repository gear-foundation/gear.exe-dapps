export const digitRecognitionAbi = [
  {
    type: "function",
    name: "fnDigitRecognitionPredict",
    inputs: [
      { name: "pixels", type: "uint8[]", internalType: "uint8[]" },
      { name: "_value", type: "uint128", internalType: "uint128" },
    ],
    outputs: [],
    stateMutability: "payable",
  },
  {
    type: "function",
    name: "fnDigitRecognitionSetConv1Weights",
    inputs: [
      {
        name: "weights",
        type: "tuple[]",
        internalType: "struct IDigitRecognition.FixedPoint[]",
        components: [
          { name: "num", type: "int64", internalType: "int64" },
          { name: "scale", type: "uint32", internalType: "uint32" },
        ],
      },
      {
        name: "bias",
        type: "tuple[]",
        internalType: "struct IDigitRecognition.FixedPoint[]",
        components: [
          { name: "num", type: "int64", internalType: "int64" },
          { name: "scale", type: "uint32", internalType: "uint32" },
        ],
      },
      { name: "_value", type: "uint128", internalType: "uint128" },
    ],
    outputs: [],
    stateMutability: "payable",
  },
  {
    type: "function",
    name: "fnDigitRecognitionSetConv2Weights",
    inputs: [
      {
        name: "weights",
        type: "tuple[]",
        internalType: "struct IDigitRecognition.FixedPoint[]",
        components: [
          { name: "num", type: "int64", internalType: "int64" },
          { name: "scale", type: "uint32", internalType: "uint32" },
        ],
      },
      {
        name: "bias",
        type: "tuple[]",
        internalType: "struct IDigitRecognition.FixedPoint[]",
        components: [
          { name: "num", type: "int64", internalType: "int64" },
          { name: "scale", type: "uint32", internalType: "uint32" },
        ],
      },
      { name: "_value", type: "uint128", internalType: "uint128" },
    ],
    outputs: [],
    stateMutability: "payable",
  },
  {
    type: "function",
    name: "fnDigitRecognitionSetFc1Weights",
    inputs: [
      {
        name: "weights",
        type: "tuple[]",
        internalType: "struct IDigitRecognition.FixedPoint[]",
        components: [
          { name: "num", type: "int64", internalType: "int64" },
          { name: "scale", type: "uint32", internalType: "uint32" },
        ],
      },
      {
        name: "bias",
        type: "tuple[]",
        internalType: "struct IDigitRecognition.FixedPoint[]",
        components: [
          { name: "num", type: "int64", internalType: "int64" },
          { name: "scale", type: "uint32", internalType: "uint32" },
        ],
      },
      { name: "_value", type: "uint128", internalType: "uint128" },
    ],
    outputs: [],
    stateMutability: "payable",
  },
  {
    type: "function",
    name: "fnDigitRecognitionSetFc2Weights",
    inputs: [
      {
        name: "weights",
        type: "tuple[]",
        internalType: "struct IDigitRecognition.FixedPoint[]",
        components: [
          { name: "num", type: "int64", internalType: "int64" },
          { name: "scale", type: "uint32", internalType: "uint32" },
        ],
      },
      {
        name: "bias",
        type: "tuple[]",
        internalType: "struct IDigitRecognition.FixedPoint[]",
        components: [
          { name: "num", type: "int64", internalType: "int64" },
          { name: "scale", type: "uint32", internalType: "uint32" },
        ],
      },
      { name: "_value", type: "uint128", internalType: "uint128" },
    ],
    outputs: [],
    stateMutability: "payable",
  },
  {
    type: "function",
    name: "initialize",
    inputs: [{ name: "_mirror", type: "address", internalType: "address" }],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "mirror",
    inputs: [],
    outputs: [{ name: "", type: "address", internalType: "address" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "onMessageSent",
    inputs: [
      { name: "id", type: "bytes32", internalType: "bytes32" },
      { name: "destination", type: "address", internalType: "address" },
      { name: "payload", type: "bytes", internalType: "bytes" },
      { name: "value", type: "uint128", internalType: "uint128" },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "onReplySent",
    inputs: [
      { name: "destination", type: "address", internalType: "address" },
      { name: "payload", type: "bytes", internalType: "bytes" },
      { name: "value", type: "uint128", internalType: "uint128" },
      { name: "replyTo", type: "bytes32", internalType: "bytes32" },
      { name: "replyCode", type: "bytes4", internalType: "bytes4" },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "sendMessage",
    inputs: [
      { name: "_payload", type: "bytes", internalType: "bytes" },
      { name: "_value", type: "uint128", internalType: "uint128" },
    ],
    outputs: [],
    stateMutability: "payable",
  },
  {
    type: "event",
    name: "DigitRecognitionPredictReply",
    inputs: [
      { name: "payload", type: "bytes", indexed: false, internalType: "bytes" },
      {
        name: "_destination",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "_value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
      {
        name: "_replyTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "_replyCode",
        type: "bytes4",
        indexed: false,
        internalType: "bytes4",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "DigitRecognitionSetConv1WeightsReply",
    inputs: [
      { name: "payload", type: "bytes", indexed: false, internalType: "bytes" },
      {
        name: "_destination",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "_value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
      {
        name: "_replyTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "_replyCode",
        type: "bytes4",
        indexed: false,
        internalType: "bytes4",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "DigitRecognitionSetConv2WeightsReply",
    inputs: [
      { name: "payload", type: "bytes", indexed: false, internalType: "bytes" },
      {
        name: "_destination",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "_value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
      {
        name: "_replyTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "_replyCode",
        type: "bytes4",
        indexed: false,
        internalType: "bytes4",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "DigitRecognitionSetFc1WeightsReply",
    inputs: [
      { name: "payload", type: "bytes", indexed: false, internalType: "bytes" },
      {
        name: "_destination",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "_value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
      {
        name: "_replyTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "_replyCode",
        type: "bytes4",
        indexed: false,
        internalType: "bytes4",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "DigitRecognitionSetFc2WeightsReply",
    inputs: [
      { name: "payload", type: "bytes", indexed: false, internalType: "bytes" },
      {
        name: "_destination",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "_value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
      {
        name: "_replyTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "_replyCode",
        type: "bytes4",
        indexed: false,
        internalType: "bytes4",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "ErrorReply",
    inputs: [
      { name: "payload", type: "bytes", indexed: false, internalType: "bytes" },
      {
        name: "_destination",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "_value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
      {
        name: "_replyTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "_replyCode",
        type: "bytes4",
        indexed: false,
        internalType: "bytes4",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "OnMessageEvent",
    inputs: [
      { name: "payload", type: "bytes", indexed: false, internalType: "bytes" },
      { name: "_id", type: "bytes32", indexed: false, internalType: "bytes32" },
      {
        name: "_destination",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "_value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "OnReplyEvent",
    inputs: [
      { name: "payload", type: "bytes", indexed: false, internalType: "bytes" },
      {
        name: "_destination",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "_value",
        type: "uint128",
        indexed: false,
        internalType: "uint128",
      },
      {
        name: "_replyTo",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32",
      },
      {
        name: "_replyCode",
        type: "bytes4",
        indexed: false,
        internalType: "bytes4",
      },
    ],
    anonymous: false,
  },
];
