import 'dotenv/config';

import { createPublicClient, createWalletClient, hexToBytes, webSocket } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { readFile } from 'node:fs/promises';
import { VaraEthApi, WsVaraEthProvider, EthereumClient, getMirrorClient, getRouterClient, getWrappedVaraClient } from '@vara-eth/api';
import type { Hex } from "viem";
import type { IInjectedTransaction } from '@vara-eth/api';
import { Sails } from 'sails-js';
import { SailsIdlParser } from 'sails-js-parser';

const ETHEREUM_RPC = process.env.ETHEREUM_RPC!;
const PRIVATE_KEY = process.env.PRIVATE_KEY as `0x${string}`;
const ROUTER_ADDRESS = process.env.ROUTER_ADDRESS as `0x${string}`;
const VARA_ETH_RPC = process.env.VARA_ETH_RPC! as "ws://"; 
const IDL_PATH = new URL('../../../target/wasm32-gear/release/vara_arkanoid.idl', import.meta.url);

const CODE_ID = process.env.CODE_ID as `0x${string}`;  // digit recognition code id

type ProgramId = `0x${string}`;

const wait1Block = () =>
  new Promise((resolve) => setTimeout(resolve,  12_000));

async function waitForProgramOnVara(
  api: VaraEthApi,
  programId: ProgramId,
  {
    maxAttempts = 1000,
    delayMs = 3_000,
  }: { maxAttempts?: number; delayMs?: number } = {},
): Promise<void> {
  const target = programId.toLowerCase();

  for (let i = 0; i < maxAttempts; i++) {
    const ids = await api.query.program.getIds();
    const hasProgram = ids.map((x) => x.toLowerCase()).includes(target);

    if (hasProgram) {
      console.log(`Program ${programId} appeared on Vara.Eth (attempt ${i + 1}).`);
      return;
    }

    console.log(
      `Program not yet visible on Vara.Eth, attempt ${i + 1}/${maxAttempts}...`,
    );
    await new Promise((resolve) => setTimeout(resolve, delayMs));
  }

  throw new Error(`Program ${programId} did not appear on Vara.Eth in time`);
}

async function initClients() {
  if (!ETHEREUM_RPC) throw new Error('ETH_RPC is not set');
  if (!PRIVATE_KEY) throw new Error('PRIVATE_KEY is not set');
  if (!ROUTER_ADDRESS) throw new Error('ROUTER_ADDRESS is not set');

  console.log('Using ETH RPC:', ETHEREUM_RPC);
  console.log('Router:', ROUTER_ADDRESS);
  console.log('Vara HTTP:', VARA_ETH_RPC);

  const transport = webSocket(ETHEREUM_RPC);

  const publicClient = createPublicClient({ transport });

  const account = privateKeyToAccount(PRIVATE_KEY);

  const walletClient = createWalletClient({account,transport});

  const ethereumClient = new EthereumClient(publicClient, walletClient, ROUTER_ADDRESS);

  await ethereumClient.isInitialized;

  const api = new VaraEthApi(
    new WsVaraEthProvider(VARA_ETH_RPC),
    ethereumClient,
  );

  const router = ethereumClient.router;
  const wvara = ethereumClient.wvara;
  return { ethereumClient, api, router, wvara, walletClient, publicClient };
}

async function initSails(programId: ProgramId) {
  const parser = await SailsIdlParser.new();
  const sails = new Sails(parser);

  const idl = await readFile(IDL_PATH, 'utf8');

  sails.parseIdl(idl);
  sails.setProgramId(programId);

  return sails;
}

async function deployProgram(
  api: VaraEthApi,
  router: ReturnType<typeof getRouterClient>,
  wvara: ReturnType<typeof getWrappedVaraClient>,
  walletClient: ReturnType<typeof createWalletClient>,
  publicClient: ReturnType<typeof createPublicClient>,
): Promise<ProgramId> {
    if (!CODE_ID) throw new Error('MAN_CODE_ID is not set');

    const topUpAmount = BigInt(10000 * 1e12);

    const tx = await router.createProgram(CODE_ID);
    await tx.sendAndWaitForReceipt();

    const programId = (await tx.getProgramId()) as ProgramId;
    console.log('ProgramId:', programId);

    await waitForProgramOnVara(api, programId);

    const mirror = getMirrorClient(programId, walletClient, publicClient);

    const approveTx = await wvara.approve(programId, topUpAmount);
    await approveTx.sendAndWaitForReceipt();

    let newStateHash: Hex | undefined = undefined;
    const unwatch = mirror.watchStateChangedEvent((hash) => {
        newStateHash = hash;
      });
    const topUpTx = await mirror.executableBalanceTopUp(topUpAmount);
    const { status } = await topUpTx.sendAndWaitForReceipt();
    while (!newStateHash) {
        console.log(newStateHash)
        await wait1Block();
      }
  
    unwatch();
    console.log('Executable balance result:', status);

    // init()
    const sails = await initSails(programId);

    const initMsgTx = await mirror.sendMessage(sails.ctors.Init.encodePayload());
    await initMsgTx.send();
    const { waitForReply: initReply } = await initMsgTx.setupReplyListener();
    const reply = await initReply();
    console.log('Init reply:', reply);
    return programId;
}

async function sendInjectedTx(
  api: VaraEthApi,
  programId: ProgramId,
  payload: `0x${string}`,
): Promise<void> {
    const injected: IInjectedTransaction = {
        destination: programId,
        payload,
      };
    const tx = await api.createInjectedTransaction(injected);
    await tx.sendAndWaitForPromise();
}

async function readResult(
  sails: Sails,
  api: VaraEthApi,
  ethereumClient: EthereumClient,
  programId: ProgramId,
): Promise<number> {
  const queryPayload = sails.services.VaraArkanoid.queries.BallPosition.encodePayload();

  const queryReply = await api.call.program.calculateReplyForHandle(
    ethereumClient.accountAddress,
    programId,
    queryPayload as `0x${string}`,
  );

  const decoded = sails.services.VaraArkanoid.queries.BallPosition.decodeResult(
    queryReply.payload,
  );

  return decoded;
}

async function main() {
    const { ethereumClient, api, router, wvara, walletClient, publicClient } = await initClients();

    let programId = await deployProgram(api, router, wvara, walletClient, publicClient);
    await wait1Block();

    const numOfSteps = 100;
    const sails = await initSails(programId);
    const payload = sails.services.VaraArkanoid.functions.SimulateGame.encodePayload(numOfSteps);
    console.log("Sending to contract...");
    console.log(programId)
    console.log(payload)
    await sendInjectedTx(api, programId, payload);
    await wait1Block();
    await wait1Block();
    let result = await readResult(sails, api, ethereumClient, programId);
    console.log("Ball position: ", result);
}

main()
  .then(() => {
    console.log('Done');
    process.exit(0);
  })
  .catch((err) => {
    console.error('Error:', err);
    process.exit(1);
  });