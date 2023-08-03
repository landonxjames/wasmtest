export namespace ConnectorTypes {
}
/**
 * # Variants
 * 
 * ## `"GET"`
 * 
 * ## `"POST"`
 * 
 * ## `"PUT"`
 * 
 * ## `"DELETE"`
 * 
 * ## `"OPTIONS"`
 * 
 * ## `"HEAD"`
 */
export type Methods = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'OPTIONS' | 'HEAD';
export type Headers = [string, string][];
export interface HttpCallOptions {
  method: Methods,
  uri: string,
  headers: Headers,
  body: Uint8Array,
}
export interface HttpReturnValues {
  status: number,
  body: string,
}