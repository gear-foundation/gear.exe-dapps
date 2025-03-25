import { HexString } from "@/lib/types";

const PROJECT_ID = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID as string;
const GEAR_API_NODE = import.meta.env.VITE_GEAR_API_NODE as string;
const ETH_CHAIN_ID = 17000; // (0x4268) Holesky
const ETH_NODE_ADDRESS = import.meta.env.VITE_ETH_NODE_ADDRESS;

const DIGIT_RECOGNITION_CONTRACT_ADDRESS = import.meta.env
  .VITE_CONTRACT_ADDRESS_DIGIT_RECOGNITION as HexString;
const CAT_IDENTIFIER_CONTRACT_ADDRESS = import.meta.env
  .VITE_CONTRACT_ADDRESS_CAT_IDENTIFIER as HexString;
const PROBABILITY_THRESHOLD_CAT_IDENTIFIER = Number(
  import.meta.env.VITE_PROBABILITY_THRESHOLD_CAT_IDENTIFIER
);

export {
  PROJECT_ID,
  GEAR_API_NODE,
  ETH_CHAIN_ID,
  ETH_NODE_ADDRESS,
  DIGIT_RECOGNITION_CONTRACT_ADDRESS,
  CAT_IDENTIFIER_CONTRACT_ADDRESS,
  PROBABILITY_THRESHOLD_CAT_IDENTIFIER,
};
