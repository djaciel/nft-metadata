import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { Metaplex } from "@metaplex-foundation/js";
// ** Comment this to use solpg imported IDL **
import { NftMetadata } from "../target/types/nft_metadata";

describe("MetaNFT - Mint with metaplex metadata", async () => {
  const testNftTitle = "MetaNFT Test";
  const testNftSymbol = "MNFT";
  const testNftUri = "https://raw.githubusercontent.com/djaciel/anchor-basics/main/testNftUri.json";

  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const metaplex = new Metaplex(provider.connection);

  const program = anchor.workspace.NftMetadata as anchor.Program<NftMetadata>;

  it("Mint!", async () => {
    // Derive the mint address and the associated token account address

    const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: wallet.publicKey,
    });
    console.log(`New token: ${mintKeypair.publicKey}`);

    // Derive the metadata and master edition addresses

    const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      program.programId
    ))[0];

    console.log(`New metadata Address: ${metadataAddress}`);

    // Transact with the "mint" function in our on-chain program

    await program.methods
      .mint(testNftTitle, testNftSymbol, testNftUri)
      .accounts({
        metadata: metadataAddress,
        mint: mintKeypair.publicKey,
        tokenAccount: tokenAddress,
        mintAuthority: wallet.publicKey,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([mintKeypair])
      .rpc();
    
    const nfts = await metaplex
      .nfts()
      .findByMint({ mintAddress: mintKeypair.publicKey });
    console.log(JSON.stringify(nfts));
  });
});
