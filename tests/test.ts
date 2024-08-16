import {
  Connection,
  Keypair,
  PublicKey,
  RpcResponseAndContext,
  SignatureResult,
  TransactionInstruction,
} from "@solana/web3.js";

const programId = new PublicKey("SsLs4JcUzhftkoMSuTSsdNJazY6X8uV3KmEV56tmfpw");

const userKeypair = Keypair.generate();

const connection = new Connection("http://localhost:8899", {
  commitment: "confirmed",
});

describe("deposit and withdraw tests", () => {
  before(async () => {});

  after(async () => {});

  it("should deposit sol", async () => {
    new TransactionInstruction({
      programId: programId,
      keys: [],
    });
  });

  it("should withdraw sol", async () => {});
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
