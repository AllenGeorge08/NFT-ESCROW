import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftEscrow } from "../target/types/nft_escrow";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN} from "bn.js";
import { createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {createUmi} from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair,generateSigner,signerIdentity, KeypairSigner } from "@metaplex-foundation/umi";
import {createV1, expectPda, fetchAssetsByOwner, MPL_CORE_PROGRAM_ID, mplCore} from "@metaplex-foundation/mpl-core";
import {fromWeb3JsKeypair, fromWeb3JsPublicKey,toWeb3JsPublicKey} from "@metaplex-foundation/umi-web3js-adapters";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { expect } from "chai";

//e https://developers.metaplex.com/umi/web3js-differences-and-adapters#from-web3-js-to-umi-5


describe("nft_escrow", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.nftEscrow as Program<NftEscrow>;
  const connection = provider.connection;

  const token_program = TOKEN_PROGRAM_ID;
  const payer = provider.wallet as anchor.Wallet;

  let maker: Keypair;
  let buyer: Keypair;

  let mint_sol: PublicKey;
  let maker_ata_sol: PublicKey;
  let buyer_ata_sol: PublicKey;
  let vault_ata_sol: PublicKey;
  let escrowPDA: PublicKey;
  let escrowBump: number;

  //MPL Core asset
  let assetAddress: PublicKey;
  let umi: any;
  let asset: KeypairSigner;


  before("Setup" , async () => {
    //e Maker Signer account(Payer also)
    maker = Keypair.generate();
    buyer = Keypair.generate();

    console.log("Maker's PublicKey: ", maker.publicKey.toBase58());
    console.log("Buyer's PublicKey: ", buyer.publicKey.toBase58());

   
  //Airdropping SOL to maker and buyer for tx fees
    const makerAirdropSignature = await connection.requestAirdrop(
      maker.publicKey,
      100*anchor.web3.LAMPORTS_PER_SOL
    );
    const txMakerConfirmed = await connection.confirmTransaction({
     signature: makerAirdropSignature,
     blockhash: (await  provider.connection.getLatestBlockhash()).blockhash,
     lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight
    });

    console.log("SOL Airdrop to maker confirmed: ", makerAirdropSignature);

    const buyerAirdropSignature = await connection.requestAirdrop(
      buyer.publicKey,
      100*anchor.web3.LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction({
      signature: buyerAirdropSignature,
      blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
      lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,
    });

    console.log("SOL Airdrop to buyer confirmed: ", buyerAirdropSignature);

    //Mint_sol
    mint_sol = await createMint(
      connection,
      payer.payer, //e The authority is the payer who pays for the mint
      payer.publicKey, //e Mint Authority
      null, //Freeze authority
      6,//Decimals,
      undefined,
      undefined,
      token_program
    );
    
    console.log("Mint created: ", mint_sol.toBase58());

    //e Asset
    umi = createUmi(connection);
    const umiMaker = fromWeb3JsKeypair(payer.payer); //e converting maker keypair to umi format
    const umiSigner = createSignerFromKeypair(umi,umiMaker); //e Should umi be changed to a local keypair?
    umi.use(signerIdentity(umiSigner));
    umi.use(mplCore());

    //e Generating asset address
    asset = generateSigner(umi);

    //e Creating the asset
    await createV1(umi,{
        asset,
        name: "Test NFT",
        uri: "",
        owner: fromWeb3JsPublicKey(maker.publicKey),
      }).sendAndConfirm(umi);

    assetAddress = toWeb3JsPublicKey(asset.publicKey);

    console.log("Asset created at: ", assetAddress.toBase58());

    //EscrowPDA
    const seed = new anchor.BN(2);
    [escrowPDA,escrowBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        seed.toArrayLike(Buffer,"le",8),
      ],
      program.programId
    );    

    console.log("ESCROW PDA: ",escrowPDA.toBase58());
    console.log("ESCROW BUMP: ", escrowBump);

  //VaultATA for sol
    // try {
    // console.log("Creating Vault ATA: ");
    // vault_ata_sol = (await getOrCreateAssociatedTokenAccount(
    //   connection,
    //   payer.payer,
    //   mint_sol,
    //   escrowPDA,
    //   true, //e bcz the owner is an pda
    //   "confirmed",
    //   undefined,
    //   token_program
    // )).address;
    // console.log("Vault ATA Address: ",vault_ata_sol.toBase58());
    // } catch (error) {
    //   console.log("Error creating Vault: ", error);
    // }

    vault_ata_sol = await getAssociatedTokenAddress(
      mint_sol,
      escrowPDA,
      true,
      token_program
    );

    console.log("Vault ATA: ", vault_ata_sol.toBase58());

    //MAKER_ATA For sol
    try {
      console.log("Creating an maker ata for sol:  ...");
      maker_ata_sol= (await getOrCreateAssociatedTokenAccount(
      connection,
      provider.wallet.payer,
      mint_sol,
      maker.publicKey,
    )).address;
    console.log("Maker ATA Created Succesfully...", maker_ata_sol.toBase58());
    } catch (error) {
      console.log("Error Creating Maker ATA: ",error);
    }

 

  //e Buyer ATA for sol
  try {
    console.log("Creating an buyer ata: ...");
    buyer_ata_sol= (await getOrCreateAssociatedTokenAccount(
      connection,
      provider.wallet.payer,
      mint_sol,
      buyer.publicKey,
    )).address;
   console.log("Buyer ATA Created Succesfully..: ", buyer_ata_sol.toBase58());
  } catch (error) {
      console.log("Error Creating Buyer ATA: ", error);
  }
  
  //Airdrop sol to maker and buyer
  const mint_to_maker = await mintTo(
      connection,
      payer.payer,
      mint_sol,
      maker_ata_sol,
      payer.publicKey,
      100*10**9,
      [payer.payer],
    );

   const mint_to_buyer = await mintTo(
      connection,
      payer.payer,
      mint_sol,
      buyer_ata_sol,
      payer.publicKey,
      100*10**9,
      [payer.payer],
      undefined,
      token_program
    );

    console.log("Setup complete");

    //AssociatedATA
    //TokenProgram
    //SystemProgram
    //MPL_CORE_PROGRAM
  });


  //WORKING
  it("Is initialized!", async () => {
    
    const seed  = new BN(2);
    const escrowBeforeInitialization = await  connection.getAccountInfo(escrowPDA);
    expect(escrowBeforeInitialization).to.be.null;

    // Add your test here.
    const tx = await program.methods.initialize(seed, new BN(3)).accountsPartial({
      maker: maker.publicKey,
      mintSol: mint_sol,
      asset: assetAddress,
      vault: vault_ata_sol,
      escrow: escrowPDA,
      makerAtaSol: maker_ata_sol,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
      mplCoreProgram: MPL_CORE_PROGRAM_ID
    }).signers([maker]).rpc();
    console.log("Your transaction signature", tx);
    
    const escrowAfterInitialization = await connection.getAccountInfo(escrowPDA);
    expect(escrowAfterInitialization).to.not.be.null;

    expect((await escrowAfterInitialization).owner.toBase58()).to.equal(program.programId.toBase58());
  
    
    //e Escrow Struct/State
    const escrowState = await program.account.escrow.fetch(escrowPDA);
    expect(escrowState.seed.toString()).to.equal((new BN(2)).toString());
    expect(escrowState.maker.toBase58()).to.equal(maker.publicKey.toBase58());
    expect(escrowState.makerMint.toBase58()).to.eq(maker_ata_sol.toBase58());
    expect(escrowState.fee).to.equals(5);

   
    console.log("\n Escrow State After Initialization:");
    console.log("  Seed:", escrowState.seed.toString());
    console.log("  Maker:", escrowState.maker.toBase58());
    console.log("  Maker Mint:", escrowState.makerMint.toBase58());
    console.log("  Bump:", escrowState.bump);
    console.log("  Fee:", escrowState.fee);

   


  });

  //WORKING
  it("List NFT", async () => {
      const assetsByOwnerBefore = await fetchAssetsByOwner(umi, escrowPDA.toString(), {
     skipDerivePlugins: false, 
   })
     console.log("Assets owned by the escrow after listing: " ,assetsByOwnerBefore)
    const amount = new BN(4);
    const seed  = new BN(2);
    const tx = await program.methods.listNft(amount, seed).accountsPartial({
      maker: maker.publicKey,
      mintSol: mint_sol,
      asset: assetAddress,
      vault: vault_ata_sol,
      escrow: escrowPDA,
      makerAtaSol: maker_ata_sol,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
      mplCoreProgram: MPL_CORE_PROGRAM_ID
    }).signers([maker]).rpc()

    console.log("Your list transaction signature: ", tx);

    const assetsByOwner = await fetchAssetsByOwner(umi, escrowPDA.toString(), {
     skipDerivePlugins: false, 
   })
    console.log("Assets owned by the escrow after listing: " ,assetsByOwner)


  })

  // //WORKING
  // it("Buy", async() => {
  //   const seed = new BN(2);
  //   const tx = await program.methods.buyNft(seed).accountsPartial({
  //     buyer: buyer.publicKey,
  //     maker: maker.publicKey,
  //     mintSol: mint_sol,
  //     asset: assetAddress,
  //     vault: vault_ata_sol,
  //     escrow: escrowPDA,
  //     makerAtaSol: maker_ata_sol,
  //     buyerAtaSol: buyer_ata_sol,
  //     associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     systemProgram: SYSTEM_PROGRAM_ID,
  //     mplCoreProgram: MPL_CORE_PROGRAM_ID
  //   }).signers([buyer]).rpc();

  //   console.log("Your buy transaction signature: ", tx);
  // })

  it("Unlist", async() => {
    const seed  = new BN(2);
 

    const unlisttx = await program.methods.unlist(seed).accountsPartial({
      maker: maker.publicKey,
      asset: assetAddress,
      escrow: escrowPDA,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
      mplCoreProgram: MPL_CORE_PROGRAM_ID
    }).signers([maker]).rpc();

    console.log("Your unlist transaction signature: ",unlisttx);
  })
});
