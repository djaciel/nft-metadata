import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftMetadata } from "../target/types/nft_metadata";

describe("nft-metadata", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftMetadata as Program<NftMetadata>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
