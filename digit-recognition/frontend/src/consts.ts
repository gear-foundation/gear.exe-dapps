import { HexString } from "@gear-js/api";

export const PROJECT_ID = import.meta.env
  .VITE_WALLET_CONNECT_PROJECT_ID as string;
export const GEAR_API_NODE = import.meta.env.VITE_GEAR_API_NODE as string;
export const ETH_CHAIN_ID = 17000; // (0x4268) Holesky
export const ETH_NODE_ADDRESS = import.meta.env.VITE_ETH_NODE_ADDRESS;

export const DIGIT_RECOGNITION_CONTRACT_ADDRESS = import.meta.env
  .VITE_CONTRACT_ADDRESS_DIGIT_RECOGNITION as HexString;
export const CAT_IDENTIFIER_CONTRACT_ADDRESS = import.meta.env
  .VITE_CONTRACT_ADDRESS_CAT_IDENTIFIER as HexString;
