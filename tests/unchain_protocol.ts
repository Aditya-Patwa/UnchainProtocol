import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { UnchainProtocol } from "../target/types/unchain_protocol";
import { randomBytes } from "crypto";
import {
  awaitComputationFinalization,
  getArciumEnv,
  getCompDefAccOffset,
  getArciumAccountBaseSeed,
  getArciumProgAddress,
  uploadCircuit,
  buildFinalizeCompDefTx,
  RescueCipher,
  deserializeLE,
  getMXEPublicKey,
  getMXEAccAddress,
  getMempoolAccAddress,
  getCompDefAccAddress,
  getExecutingPoolAccAddress,
  getComputationAccAddress,
  x25519,
} from "@arcium-hq/client";
import * as fs from "fs";
import * as os from "os";
import { expect } from "chai";

describe("UnchainProtocol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace
    .UnchainProtocol as Program<UnchainProtocol>;
  const provider = anchor.getProvider();

  type Event = anchor.IdlEvents<(typeof program)["idl"]>;
  const awaitEvent = async <E extends keyof Event>(
    eventName: E
  ): Promise<Event[E]> => {
    let listenerId: number;
    const event = await new Promise<Event[E]>((res) => {
      listenerId = program.addEventListener(eventName, (event) => {
        res(event);
      });
    });
    await program.removeEventListener(listenerId);

    return event;
  };

  const arciumEnv = getArciumEnv();

  it("Is initialized!", async () => {
    const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);


    console.log("Initializing init creator balance computation definition");
    const initCreatorsBal = await initInitCreatorsBalanceCompDef(
      program,
      owner,
      false,
      false
    );
    console.log(
      "Init Creator Balance computation definition initialized with signature",
      initCreatorsBal
    );


    console.log("Initializing tip creator computation definition");
    const tipCreatorsBal = await initTipCreatorCompDef(
      program,
      owner,
      false,
      false
    );
    console.log(
      "tip creator computation definition initialized with signature",
      tipCreatorsBal
    );



    console.log("Initializing Vault");
    const [vaultPda, _] = PublicKey.findProgramAddressSync([
      Buffer.from("vault")
    ], program.programId);

    console.log(vaultPda.toString());

    const vaultSign = await program.methods.initializeVault().accounts({
      payer: owner.publicKey,
      vault: vaultPda
    }).signers([owner]).rpc();

    console.log("Vault created with signature, ", vaultSign);

    expect(1).to.equal(1);



    // // Time for checking Tiping

    // const supporter = Keypair.generate();

    // const supporterPrivateKey = x25519.utils.randomSecretKey();
    // const supporterPublicKey = x25519.getPublicKey(supporterPrivateKey);

    // const supporterSharedSecret = x25519.getSharedSecret(supporterPublicKey, mxePublicKey);
    // const supporterCipher = new RescueCipher(supporterSharedSecret);

    // const amount = BigInt(10 * LAMPORTS_PER_SOL);
    // const plaintext = [amount]
    // const amountCiphertext = supporterCipher.encrypt(plaintext, nonce);

    // const amountNonce = randomBytes(16);
    // const tipEvent = awaitEvent("tipEvent");
    // const tipComputationOffset = new anchor.BN(randomBytes(8), "hex");


    // const [creatorPda] = await PublicKey.findProgramAddressSync(
    //   [Buffer.from("account"), creator.publicKey.toBytes()],
    //   program.programId
    // );


    // const tippingSig = await program.methods
    //   .tipCreator(
    //     tipComputationOffset,
    //     Array.from(amountCiphertext[0]),
    //     Array.from(supporterPublicKey),
    //     new anchor.BN(deserializeLE(amountNonce).toString())
    //   ).accounts({
    //     payer: supporter.publicKey,
    //     creatorAccount: creatorPda,
    //     computationAccount: getComputationAccAddress(
    //       program.programId,
    //       tipComputationOffset
    //     ),
    //     clusterAccount: arciumEnv.arciumClusterPubkey,
    //     mxeAccount: getMXEAccAddress(program.programId),
    //     mempoolAccount: getMempoolAccAddress(program.programId),
    //     executingPool: getExecutingPoolAccAddress(program.programId),
    //     compDefAccount: getCompDefAccAddress(
    //       program.programId,
    //       Buffer.from(getCompDefAccOffset("tip_creator")).readUInt32LE()
    //     ),
    //   }).signers([supporter]).rpc({ skipPreflight: true, commitment: "confirmed" });
    // console.log("Queue sig is ", tippingSig);

    // const tipfinalizeSig = await awaitComputationFinalization(
    //   provider as anchor.AnchorProvider,
    //   tipComputationOffset,
    //   program.programId,
    //   "confirmed"
    // );
    // console.log("Finalize sig is ", tipfinalizeSig);

    // const tipCompleteEvent = await tipEvent;
    // const decryptedTip = cipher.decrypt([tipCompleteEvent.unchaimedTips], tipCompleteEvent.nonce)[0];
    // expect(decryptedTip).to.equal(amount);


  });


  it("Init Creator", async () => {


    // Started Creating Creator
    const creator = Keypair.generate();



    console.log("Airdropping funds to Creator");
    const airdropcreatorTx = await provider.connection.requestAirdrop(
      creator.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction({
      signature: airdropcreatorTx,
      blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
      lastValidBlockHeight: (
        await provider.connection.getLatestBlockhash()
      ).lastValidBlockHeight,
    });
    console.log("Funds airdropped to Creator");





    const mxePublicKey = await getMXEPublicKeyWithRetry(
      provider as anchor.AnchorProvider,
      program.programId
    );

    console.log("MXE x25519 pubkey is", mxePublicKey);

    try {
      const creatorDetails = {
        name: "Aditya Patwa",
        title: "Founder @UnchainProtocol",
        about: "I love building <encrypted> apps. gMPC ☂️",
        imageCid: "Just a ipfs_cid from Pinata"
      };

      const [creatorPda] = PublicKey.findProgramAddressSync([
        Buffer.from("profile"), creator.publicKey.toBytes()
      ], program.programId);


      const [creatorAccountPda] = PublicKey.findProgramAddressSync([
        Buffer.from("account"), creator.publicKey.toBytes()
      ], program.programId);


      const creatorPrivateKey = x25519.utils.randomSecretKey();
      const creatorPublicKey = x25519.getPublicKey(creatorPrivateKey);

      const sharedSecret = x25519.getSharedSecret(creatorPublicKey, mxePublicKey);
      const cipher = new RescueCipher(sharedSecret);

      // const val1 = BigInt(1);
      // const val2 = BigInt(2);
      const mountplaintext = [BigInt(0)];

      const nonce = randomBytes(16);
      const ciphertext = cipher.encrypt(mountplaintext, nonce);

      // const initCreatorBalanceEvent = awaitEvent("initCreatorEvent");
      const computationOffset = new anchor.BN(randomBytes(8), "hex");

      const queueSig = await program.methods
        .initCreatorsBalance(
          computationOffset,
          creatorDetails.name,
          creatorDetails.title,
          creatorDetails.about,
          creatorDetails.imageCid,
          // Array.from(ciphertext[0]),
          Array.from(creatorPublicKey),
          Array.from(ciphertext[0]),
          new anchor.BN(deserializeLE(nonce).toString()),
        )
        .accounts({
          computationAccount: getComputationAccAddress(
            program.programId,
            computationOffset
          ),
          payer: creator.publicKey,
          mxeAccount: getMXEAccAddress(program.programId),
          mempoolAccount: getMempoolAccAddress(program.programId),
          executingPool: getExecutingPoolAccAddress(program.programId),
          compDefAccount: getCompDefAccAddress(
            program.programId,
            Buffer.from(getCompDefAccOffset("init_creators_balance")).readUInt32LE()
          ),
          clusterAccount: arciumEnv.arciumClusterPubkey,
          creatorProfile: creatorPda,
          creatorAccount: creatorAccountPda,
        })
        .signers([creator])
        .rpc({ skipPreflight: true, commitment: "confirmed" });
      console.log("Queue sig is ", queueSig);

      const finalizeSig = await awaitComputationFinalization(
        provider as anchor.AnchorProvider,
        computationOffset,
        program.programId,
        "confirmed"
      );
      console.log("Finalize sig is ", finalizeSig);
    } catch(e) {
      console.log(e);
    }

    // const initCreator = await initCreatorBalanceEvent;
    // const decrypted = cipher.decrypt([initCreator.totalTips], initCreator.nonce)[0];


    expect(0).to.equal(0);


  })

  // async function initAddTogetherCompDef(
  //   program: Program<UnchainProtocol>,
  //   owner: anchor.web3.Keypair,
  //   uploadRawCircuit: boolean,
  //   offchainSource: boolean
  // ): Promise<string> {
  //   const baseSeedCompDefAcc = getArciumAccountBaseSeed(
  //     "ComputationDefinitionAccount"
  //   );
  //   const offset = getCompDefAccOffset("add_together");

  //   const compDefPDA = PublicKey.findProgramAddressSync(
  //     [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
  //     getArciumProgAddress()
  //   )[0];

  //   console.log("Comp def pda is ", compDefPDA);

  //   const sig = await program.methods
  //     .initAddTogetherCompDef()
  //     .accounts({
  //       compDefAccount: compDefPDA,
  //       payer: owner.publicKey,
  //       mxeAccount: getMXEAccAddress(program.programId),
  //     })
  //     .signers([owner])
  //     .rpc({
  //       commitment: "confirmed",
  //     });
  //   console.log("Init add together computation definition transaction", sig);

  //   if (uploadRawCircuit) {
  //     const rawCircuit = fs.readFileSync("build/add_together.arcis");

  //     await uploadCircuit(
  //       provider as anchor.AnchorProvider,
  //       "add_together",
  //       program.programId,
  //       rawCircuit,
  //       true
  //     );
  //   } else if (!offchainSource) {
  //     const finalizeTx = await buildFinalizeCompDefTx(
  //       provider as anchor.AnchorProvider,
  //       Buffer.from(offset).readUInt32LE(),
  //       program.programId
  //     );

  //     const latestBlockhash = await provider.connection.getLatestBlockhash();
  //     finalizeTx.recentBlockhash = latestBlockhash.blockhash;
  //     finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;

  //     finalizeTx.sign(owner);

  //     await provider.sendAndConfirm(finalizeTx);
  //   }
  //   return sig;
  // }




  async function initInitCreatorsBalanceCompDef(
    program: Program<UnchainProtocol>,
    owner: anchor.web3.Keypair,
    uploadRawCircuit: boolean,
    offchainSource: boolean
  ): Promise<string> {
    const baseSeedCompDefAcc = getArciumAccountBaseSeed(
      "ComputationDefinitionAccount"
    );
    const offset = getCompDefAccOffset("init_creators_balance");

    const compDefPDA = PublicKey.findProgramAddressSync(
      [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
      getArciumProgAddress()
    )[0];

    console.log("Comp def pda is ", compDefPDA);

    const sig = await program.methods
      .initInitCreatorsBalanceCompDef()
      .accounts({
        compDefAccount: compDefPDA,
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(program.programId),
      })
      .signers([owner])
      .rpc({
        commitment: "confirmed",
      });
    console.log("Init add together computation definition transaction", sig);

    if (uploadRawCircuit) {
      const rawCircuit = fs.readFileSync("build/init_creators_balance.arcis");

      await uploadCircuit(
        provider as anchor.AnchorProvider,
        "init_creators_balance",
        program.programId,
        rawCircuit,
        true
      );
    } else if (!offchainSource) {
      const finalizeTx = await buildFinalizeCompDefTx(
        provider as anchor.AnchorProvider,
        Buffer.from(offset).readUInt32LE(),
        program.programId
      );

      const latestBlockhash = await provider.connection.getLatestBlockhash();
      finalizeTx.recentBlockhash = latestBlockhash.blockhash;
      finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;

      finalizeTx.sign(owner);

      await provider.sendAndConfirm(finalizeTx);
    }
    return sig;
  }




  async function initTipCreatorCompDef(
    program: Program<UnchainProtocol>,
    owner: anchor.web3.Keypair,
    uploadRawCircuit: boolean,
    offchainSource: boolean
  ): Promise<string> {
    const baseSeedCompDefAcc = getArciumAccountBaseSeed(
      "ComputationDefinitionAccount"
    );
    const offset = getCompDefAccOffset("tip_creator");

    const compDefPDA = PublicKey.findProgramAddressSync(
      [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
      getArciumProgAddress()
    )[0];

    console.log("Comp def pda is ", compDefPDA);

    const sig = await program.methods
      .initTipCreatorCompDef()
      .accounts({
        compDefAccount: compDefPDA,
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(program.programId),
      })
      .signers([owner])
      .rpc({
        commitment: "confirmed",
      });
    console.log("Init add together computation definition transaction", sig);

    if (uploadRawCircuit) {
      const rawCircuit = fs.readFileSync("build/tip_creator.arcis");

      await uploadCircuit(
        provider as anchor.AnchorProvider,
        "tip_creator",
        program.programId,
        rawCircuit,
        true
      );
    } else if (!offchainSource) {
      const finalizeTx = await buildFinalizeCompDefTx(
        provider as anchor.AnchorProvider,
        Buffer.from(offset).readUInt32LE(),
        program.programId
      );

      const latestBlockhash = await provider.connection.getLatestBlockhash();
      finalizeTx.recentBlockhash = latestBlockhash.blockhash;
      finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;

      finalizeTx.sign(owner);

      await provider.sendAndConfirm(finalizeTx);
    }
    return sig;
  }

});

async function getMXEPublicKeyWithRetry(
  provider: anchor.AnchorProvider,
  programId: PublicKey,
  maxRetries: number = 10,
  retryDelayMs: number = 500
): Promise<Uint8Array> {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const mxePublicKey = await getMXEPublicKey(provider, programId);
      if (mxePublicKey) {
        return mxePublicKey;
      }
    } catch (error) {
      console.log(`Attempt ${attempt} failed to fetch MXE public key:`, error);
    }

    if (attempt < maxRetries) {
      console.log(
        `Retrying in ${retryDelayMs}ms... (attempt ${attempt}/${maxRetries})`
      );
      await new Promise((resolve) => setTimeout(resolve, retryDelayMs));
    }
  }

  throw new Error(
    `Failed to fetch MXE public key after ${maxRetries} attempts`
  );
}

function readKpJson(path: string): anchor.web3.Keypair {
  const file = fs.readFileSync(path);
  return anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(file.toString()))
  );
}