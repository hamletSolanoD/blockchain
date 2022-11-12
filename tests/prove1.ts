import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Prove1 } from "../target/types/prove1";

describe("prove1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Prove1 as Program<Prove1>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
