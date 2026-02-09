import 'dotenv/config';
import { drawMnist28x28 } from "./draw-ui.ts";
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
const IDL_PATH = new URL('../../target/wasm32-gear/release/digit_recognition.idl', import.meta.url);

const CODE_ID = process.env.CODE_ID as `0x${string}`;  // digit recognition code id

const PROGRAM_ID = process.env.PROGRAM_ID as `0x${string}`;

type ProgramId = `0x${string}`;
type Mode = 'deploy' | 'predict' | 'full' | 'injected';
const MODE: Mode = (process.env.MODE as Mode) ?? 'predict';
const wait1Block = () =>
  new Promise((resolve) => setTimeout(resolve,  12_000));

function normalizeHexPayload(raw: string): `0x${string}` {
  const s = raw.replace(/\s+/g, "").trim();
  if (!s.startsWith("0x")) throw new Error("Payload must start with 0x");
  const hex = s.slice(2);
  if (hex.length === 0) throw new Error("Payload is empty");
  if (hex.length % 2 !== 0) throw new Error(`Payload hex length must be even, got ${hex.length}`);
  if (!/^[0-9a-fA-F]+$/.test(hex)) throw new Error("Payload contains non-hex characters");
  return s as `0x${string}`;
}

type PayloadMap = Record<string, `0x${string}`>;
function parseSectionedPayloadFile(text: string): PayloadMap {
  const lines = text.split(/\r?\n/);

  const sections: Record<string, string[]> = {};
  let current: string | null = null;

  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#") || trimmed.startsWith("//")) continue;

    const m = trimmed.match(/^\[(.+)\]$/);
    if (m) {
      current = m[1].trim();
      if (!current) throw new Error("Empty section name []");
      if (!sections[current]) sections[current] = [];
      continue;
    }

    if (!current) {
      throw new Error(`Found data before any section header: ${trimmed.slice(0, 32)}...`);
    }
    sections[current].push(trimmed);
  }

  const out: PayloadMap = {};
  for (const [name, chunkLines] of Object.entries(sections)) {
    out[name] = normalizeHexPayload(chunkLines.join(""));
  }
  return out;
}

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
    const raw = await readFile(new URL("./payloads.txt", import.meta.url), "utf8");
    const payloads = parseSectionedPayloadFile(raw);
    // Weights for Conv1
    await sendInjectedTx(api, programId, payloads["conv1"]);

    // Weights for Conv2
    await sendInjectedTx(api, programId, payloads["conv2"]);

    // Weights for Fc1
    await sendInjectedTx(api, programId, payloads["fc1"]);

    // Weights for Fc2
    await sendInjectedTx(api, programId, payloads["fc2"]);
    return programId;
}

async function sendMessage(
  mirror: ReturnType<typeof getMirrorClient>,
  payload: string,
  value: bigint = 0n,
): Promise<void> {
  const msgTx = await mirror.sendMessage(payload as `0x${string}`, value);
  await msgTx.send();

  const { waitForReply } = await msgTx.setupReplyListener();
  const reply = await waitForReply();

  console.log('Reply:', reply.payload, reply.replyCode, reply.value);
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
  const queryPayload = sails.services.DigitRecognition.queries.Result.encodePayload();

  const queryReply = await api.call.program.calculateReplyForHandle(
    ethereumClient.accountAddress,
    programId,
    queryPayload as `0x${string}`,
  );

  const decoded = sails.services.DigitRecognition.queries.Result.decodeResult(
    queryReply.payload,
  );

  return decoded;
}


function formatProbsConsole(raw: number[], scale = 6): string {
  const denom = 10 ** scale;

  const probs = raw.map((v) => v / denom);
  const sum = probs.reduce((a, b) => a + b, 0);

  let bestIdx = 0;
  for (let i = 1; i < probs.length; i++) {
    if (probs[i] > probs[bestIdx]) bestIdx = i;
  }

  const BAR_MAX = 30;
  const max = Math.max(...probs);

  const lines: string[] = [];
  lines.push(`Probabilities (scale=${scale}):`);
  lines.push(`Sum: ${(sum * 100).toFixed(4)}%`);

  for (let d = 0; d < probs.length; d++) {
    const p = probs[d];
    const pct = (p * 100).toFixed(2).padStart(6);
    const barLen = max > 0 ? Math.round((p / max) * BAR_MAX) : 0;
    const bar = "#".repeat(barLen).padEnd(BAR_MAX, " ");
    const mark = d === bestIdx ? " <==" : "";
    lines.push(`${d}: ${pct}% |${bar}| raw=${raw[d]}${mark}`);
  }

  lines.push(
    `Prediction: ${bestIdx} (${(probs[bestIdx] * 100).toFixed(2)}%)`,
  );

  return lines.join("\n");
}



async function readRLayers(
  sails: Sails,
  api: VaraEthApi,
  ethereumClient: EthereumClient,
  programId: ProgramId,
): Promise<number> {
  const queryPayload = sails.services.DigitRecognition.queries.LayersSet.encodePayload();

  const queryReply = await api.call.program.calculateReplyForHandle(
    ethereumClient.accountAddress,
    programId,
    queryPayload as `0x${string}`,
  );

  const decoded = sails.services.DigitRecognition.queries.LayersSet.decodeResult(
    queryReply.payload,
  );

  console.log('Layers: ', decoded);
  return decoded;
}

async function main() {
  console.log('MODE =', MODE);

  const { ethereumClient, api, router, wvara, walletClient, publicClient } = await initClients();

 
  if (MODE === 'deploy') {
    let programId = await deployProgram(api, router, wvara, walletClient, publicClient);
    await wait1Block();
    const sails = await initSails(programId);
    await readRLayers(sails, api, ethereumClient, programId);
    return;
  }

  if (MODE === 'predict') {
      const mirror = getMirrorClient(PROGRAM_ID, walletClient, publicClient);
      const stateHash = await mirror.stateHash();
      let state = await api.query.program.readState(stateHash);
      if (state.executableBalance <  BigInt(80 * 1e12)) {
        console.log("Please top up the program balance");
      }
      console.log("Open draw UI...");      

      const pixelsU8 = await drawMnist28x28({ port: 5174, openBrowser: true });
      const pixelsU16 = pixelsU8.map((v) => v); 
      const sails = await initSails(PROGRAM_ID);
      const payload = sails.services.DigitRecognition.functions.Predict.encodePayload(pixelsU16);
      console.log("Sending to contract...");
      await sendInjectedTx(api, PROGRAM_ID, payload);
      await wait1Block();
      await wait1Block();
      let result = await readResult(sails, api, ethereumClient, PROGRAM_ID);
      console.log(formatProbsConsole(result, 6));
      return;
  }
  
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