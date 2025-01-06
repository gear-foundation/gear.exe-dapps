import { GearApi, decodeAddress } from "@gear-js/api";
import { TypeRegistry } from "@polkadot/types";
import { TransactionBuilder, ActorId, ZERO_ADDRESS } from "sails-js";

export interface FixedPoint {
  num: number | string | bigint;
  scale: number;
}

export interface Point {
  c_re: FixedPoint;
  c_im: FixedPoint;
}

export interface Result {
  c_re: FixedPoint;
  c_im: FixedPoint;
  iter: number;
}

export class Program {
  public readonly registry: TypeRegistry;
  public readonly manager: Manager;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
      FixedPoint: { num: "i64", scale: "u32" },
      Point: { c_re: "String", c_im: "String" },
      Result: { c_re: "String", c_im: "String", iter: "u32" },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.manager = new Manager(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      "upload_program",
      "New",
      "String",
      "String",
      code
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      "create_program",
      "New",
      "String",
      "String",
      codeId
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class Manager {
  constructor(private _program: Program) {}

  public addCheckers(checkers: Array<ActorId>): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error("Program ID is not set");
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      "send_message",
      ["Manager", "AddCheckers", checkers],
      "(String, String, Vec<[u8;32]>)",
      "Null",
      this._program.programId
    );
  }

  public checkPointsSet(
    max_iter: number,
    batch_size: number
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error("Program ID is not set");
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      "send_message",
      ["Manager", "CheckPointsSet", max_iter, batch_size],
      "(String, String, u32, u32)",
      "Null",
      this._program.programId
    );
  }

  public generateAndStorePoints(
    width: number,
    height: number,
    x_min: FixedPoint,
    x_max: FixedPoint,
    y_min: FixedPoint,
    y_max: FixedPoint,
    points_per_call: number
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error("Program ID is not set");
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      "send_message",
      [
        "Manager",
        "GenerateAndStorePoints",
        width,
        height,
        x_min,
        x_max,
        y_min,
        y_max,
        points_per_call,
      ],
      "(String, String, u32, u32, FixedPoint, FixedPoint, FixedPoint, FixedPoint, u32)",
      "Null",
      this._program.programId
    );
  }

  public restart(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error("Program ID is not set");
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      "send_message",
      ["Manager", "Restart"],
      "(String, String)",
      "Null",
      this._program.programId
    );
  }

  public resultCalculated(
    points: Array<Point>,
    results: Array<number>
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error("Program ID is not set");
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      "send_message",
      ["Manager", "ResultCalculated", points, results],
      "(String, String, Vec<Point>, Vec<u32>)",
      "Null",
      this._program.programId
    );
  }

  public sendNextBatch(
    checker: ActorId,
    max_iter: number,
    batch_size: number
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error("Program ID is not set");
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      "send_message",
      ["Manager", "SendNextBatch", checker, max_iter, batch_size],
      "(String, String, [u8;32], u32, u32)",
      "Null",
      this._program.programId
    );
  }

  public async getCheckers(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`
  ): Promise<Array<ActorId>> {
    const payload = this._program.registry
      .createType("(String, String)", ["Manager", "GetCheckers"])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess)
      throw new Error(
        this._program.registry.createType("String", reply.payload).toString()
      );
    const result = this._program.registry.createType(
      "(String, String, Vec<[u8;32]>)",
      reply.payload
    );
    return result[2].toJSON() as unknown as Array<ActorId>;
  }

  public async getPoints(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`
  ): Promise<Array<Point>> {
    const payload = this._program.registry
      .createType("(String, String)", ["Manager", "GetPoints"])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess)
      throw new Error(
        this._program.registry.createType("String", reply.payload).toString()
      );
    const result = this._program.registry.createType(
      "(String, String, Vec<Point>)",
      reply.payload
    );
    return result[2].toJSON() as unknown as Array<Point>;
  }

  public async getPointsLen(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`
  ): Promise<number> {
    const payload = this._program.registry
      .createType("(String, String)", ["Manager", "GetPointsLen"])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess)
      throw new Error(
        this._program.registry.createType("String", reply.payload).toString()
      );
    const result = this._program.registry.createType(
      "(String, String, u32)",
      reply.payload
    );
    return result[2].toNumber() as unknown as number;
  }

  public async getResults(
    start_index: number,
    end_index: number,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`
  ): Promise<Array<Result>> {
    const payload = this._program.registry
      .createType("(String, String, u32, u32)", [
        "Manager",
        "GetResults",
        start_index,
        end_index,
      ])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess)
      throw new Error(
        this._program.registry.createType("String", reply.payload).toString()
      );
    const result = this._program.registry.createType(
      "(String, String, Vec<Result>)",
      reply.payload
    );
    return result[2].toJSON() as unknown as Array<Result>;
  }

  public async pointsSent(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`
  ): Promise<number> {
    const payload = this._program.registry
      .createType("(String, String)", ["Manager", "PointsSent"])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess)
      throw new Error(
        this._program.registry.createType("String", reply.payload).toString()
      );
    const result = this._program.registry.createType(
      "(String, String, u32)",
      reply.payload
    );
    return result[2].toNumber() as unknown as number;
  }
}
