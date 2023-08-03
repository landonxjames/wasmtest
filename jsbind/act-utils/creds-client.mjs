export function getCreds() {
  //TODO: it would probably be best to use the native language SDK to do credential
  //resolution and then pass those to the WASM module
  const creds = {
    accessKeyId: process.env.AWS_ACCESS_KEY_ID,
    secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY,
    sessionToken: process.env.AWS_SESSION_TOKEN,
  };

  if (
    creds.accessKeyId === undefined ||
    creds.secretAccessKey === undefined ||
    creds.sessionToken === undefined
  ) {
    //Undefined vals removed from stringified JSON without this replacer
    const replacer = (key, value) =>
      typeof value === "undefined" ? null : value;
    throw new Error(
      `Credentials not correctly configured: ${JSON.stringify(
        creds,
        replacer,
        2
      )}`
    );
  }

  return creds;
}
export default getCreds;
