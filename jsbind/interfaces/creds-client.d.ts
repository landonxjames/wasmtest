export namespace CredsClient {
  export function getCreds(): Creds;
}
export interface Creds {
  accessKeyId: string,
  secretAccessKey: string,
  sessionToken: string,
}
