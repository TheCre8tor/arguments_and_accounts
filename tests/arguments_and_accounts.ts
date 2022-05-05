import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ArgumentsAndAccounts } from "../target/types/arguments_and_accounts";

describe("arguments_and_accounts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ArgumentsAndAccounts as Program<ArgumentsAndAccounts>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
