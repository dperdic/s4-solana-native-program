import { serialize } from "borsh";
import { Buffer } from "buffer";
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

const programId = new PublicKey("3UPWWCEfPsR6RWKufmq1TEDzGJQUpPh2aXi39FUgGK6S");

const PDA_VAULT_SEED = "sol_account";

const userKeypair = Keypair.generate();

const connection = new Connection("http://localhost:8899", {
  commitment: "confirmed",
});

describe("deposit and withdraw tests", () => {
  before(async () => {
    const txHash = await connection.requestAirdrop(
      userKeypair.publicKey,
      100 * LAMPORTS_PER_SOL
    );

    const confirmation = await confirmTransaction(txHash);

    if (confirmation.value.err) {
      throw confirmation.value.err;
    }

    console.log("Airdrop transaction hash: ", txHash);
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

    if (!pdaInfo) throw "no pda";

    console.log(pdaInfo);

    assert(pdaInfo.owner === programId, "invalid owner");

    assert(
      pdaInfo.lamports === solAmount * LAMPORTS_PER_SOL,
      "invalid account balance after deposit"
    );
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

    if (!pdaInfo) throw "no pda";

    console.log(pdaInfo);

    assert(pdaInfo.owner === programId, "invalid owner");

    assert(
      pdaInfo.lamports === solAmount * LAMPORTS_PER_SOL,
      "invalid account balance after deposit"
    );
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
      // data: Buffer.from(new Uint8Array([SolAccountInstruction.WithdrawSol])),
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

    if (!pdaInfo) throw "no pda";

    console.log(pdaInfo);

    assert(pdaInfo.owner === programId, "invalid owner");

    assert(0, "invalid account balance after withdraw");
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
