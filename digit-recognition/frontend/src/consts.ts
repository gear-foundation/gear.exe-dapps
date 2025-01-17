export const PROJECT_ID = process.env.VITE_WALLET_CONNECT_PROJECT_ID as string;
export const GEAR_API_NODE = process.env.VITE_GEAR_API_NODE as string;
export const ETH_CHAIN_ID = 17000; // (0x4268) Holesky
export const ETH_NODE_ADDRESS = process.env.VITE_ETH_NODE_ADDRESS as string;

export const DIGIT_RECOGNITION_CONTRACT_ADDRESS = process.env
  .VITE_CONTRACT_ADDRESS_DIGIT_RECOGNITION as `0x${string}`;

console.log("log envs:", {
  PROJECT_ID,
  GEAR_API_NODE,
  ETH_CHAIN_ID,
  ETH_NODE_ADDRESS,
  DIGIT_RECOGNITION_CONTRACT_ADDRESS,
});
