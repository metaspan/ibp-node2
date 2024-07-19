export async function getNonce(api, address) {
  const { nonce } = await api.query.system.account(address);
  return nonce.toNumber();
}

export async function submitTransaction(tx, signer, nonce, tip) {
  return new Promise((resolve, reject) => {
    tx.signAndSend(signer, { nonce, tip }, ({ status, dispatchError }) => {
      if (dispatchError) {
        if (dispatchError.isModule) {
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { documentation, name, section } = decoded;
          console.log(`${section}.${name}: ${documentation.join(' ')}`);
          reject(new Error(`${section}.${name}: ${documentation.join(' ')}`));
        } else {
          console.log(dispatchError.toString());
          reject(new Error(dispatchError.toString()));
        }
      } else {
        if (status.isInBlock) {
          console.log(`Transaction included at blockHash ${status.asInBlock}`);
          resolve(status.asInBlock);
        } else if (status.isFinalized) {
          console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
          resolve(status.asFinalized);
        } else {
          console.log(`Current status: ${status.type}`);
        }
      }
    });
  });
}

export async function handleTransactionResponse (api, { status, dispatchError }) {
  if (dispatchError) {
    if (dispatchError.isModule) {
      const decoded = api.registry.findMetaError(dispatchError.asModule);
      const { documentation=[], name, section } = decoded;
      console.log(`${section}.${name}: ${documentation.join(' ')}`);
    } else {
      console.log(dispatchError.toString());
    }
  } else {
    if (status.isInBlock) {
      console.log(`Transaction included at blockHash ${status.asInBlock}`);
    } else if (status.isFinalized) {
      console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
    } else {
      console.log(`Current status: ${status.type}`);
    }
  }
}
