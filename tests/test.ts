import { serialize } from "borsh";
import { Buffer } from "buffer";
import { config } from "dotenv";
import {
  Connection,
  Keypair,
  PublicKey,
  RpcResponseAndContext,
  SignatureResult,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import assert from "assert";

config();

const programId = new PublicKey("B2WsLnHt4Ap1tjpWijFuUVXx59Z9tdcRdv2FW3Cnnvd1");

const PDA_VAULT_SEED = "sol_account";

const userKeypair = Keypair.generate();

const connection = new Connection(process.env.RPC_URL, {
  commitment: "confirmed",
});

describe("deposit and withdraw tests", () => {
  before(async () => {
    const txHash = await connection.requestAirdrop(
      userKeypair.publicKey,
      100 * LAMPORTS_PER_SOL
    );

    try {
      const confirmation = await confirmTransaction(txHash);

      if (confirmation.value.err) {
        console.error(confirmation.value.err);

        throw "airdrop failed";
      }
    } catch (error) {
      console.error(error);

      throw "airdrop failed";
    }
  });

  it("should deposit sol", async () => {
    const solAmount = 1;

    const pda = getPda(userKeypair.publicKey);

    const deposit_ix = new TransactionInstruction({
      keys: [
        {
          pubkey: userKeypair.publicKey,
          isSigner: true,
          isWritable: false,
        },
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: programId,
      data: serializeDeposit(BigInt(solAmount * LAMPORTS_PER_SOL)),
    });

    const tx = new Transaction().add(deposit_ix);

    const txHash = await sendAndConfirmTransaction(
      connection,
      tx,
      [userKeypair],
      {
        commitment: "confirmed",
      }
    );

    assert(typeof txHash === "string", "unexpected transaction hash");

    console.log("Deposit transaction hash: ", txHash);

    const pdaInfo = await connection.getAccountInfo(pda);

    assert(pdaInfo !== null, "no pda");

    console.log(pdaInfo);

    assert(pdaInfo.owner.toBase58() === programId.toBase58(), "invalid owner");
  });

  it("should deposit sol", async () => {
    const solAmount = 1;

    const pda = getPda(userKeypair.publicKey);

    const deposit_ix = new TransactionInstruction({
      keys: [
        {
          pubkey: userKeypair.publicKey,
          isSigner: true,
          isWritable: false,
        },
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: programId,
      data: serializeDeposit(BigInt(solAmount * LAMPORTS_PER_SOL)),
    });

    const tx = new Transaction().add(deposit_ix);

    const txHash = await sendAndConfirmTransaction(
      connection,
      tx,
      [userKeypair],
      {
        commitment: "confirmed",
      }
    );

    assert(typeof txHash === "string", "unexpected transaction hash");

    console.log("Deposit transaction hash: ", txHash);

    const pdaInfo = await connection.getAccountInfo(pda);

    assert(pdaInfo !== null, "no pda");

    console.log(pdaInfo);

    assert(pdaInfo.owner.toBase58() === programId.toBase58(), "invalid owner");
  });

  it("should withdraw sol", async () => {
    const pda = getPda(userKeypair.publicKey);

    const withdraw_ix = new TransactionInstruction({
      keys: [
        {
          pubkey: userKeypair.publicKey,
          isSigner: true,
          isWritable: false,
        },
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: programId,
      data: serializeWithdraw(),
    });

    const tx = new Transaction().add(withdraw_ix);

    const txHash = await sendAndConfirmTransaction(
      connection,
      tx,
      [userKeypair],
      {
        commitment: "confirmed",
      }
    );

    assert(typeof txHash === "string", "unexpected transaction hash");

    console.log("Withdraw transaction hash: ", txHash);

    const pdaInfo = await connection.getAccountInfo(pda);

    assert(pdaInfo !== null, "no pda");

    console.log(pdaInfo);

    assert(pdaInfo.owner.toBase58() === programId.toBase58(), "invalid owner");
  });
});

const confirmTransaction = async (
  tx: string
): Promise<RpcResponseAndContext<SignatureResult>> => {
  const bh = await connection.getLatestBlockhash();

  return await connection.confirmTransaction(
    {
      signature: tx,
      blockhash: bh.blockhash,
      lastValidBlockHeight: bh.lastValidBlockHeight,
    },
    "confirmed"
  );
};

const getPda = (user: PublicKey): PublicKey => {
  const [pda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from(PDA_VAULT_SEED), user.toBytes()],
    programId
  );

  return pda;
};

const serializeDeposit = (amount: bigint): Buffer => {
  const schema = {
    struct: {
      instruction: "u8",
      amount: "u64",
    },
  };

  return Buffer.from(
    serialize(schema, {
      instruction: SolAccountInstruction.DepositSol,
      amount: amount,
    })
  );
};

const serializeWithdraw = (): Buffer => {
  const schema = {
    struct: {
      instruction: "u8",
    },
  };

  return Buffer.from(
    serialize(schema, {
      instruction: SolAccountInstruction.WithdrawSol,
    })
  );
};

enum SolAccountInstruction {
  DepositSol = 0,
  WithdrawSol = 1,
}
