import request from "sync-request";

export function makeHttpRequest(options) {
  console.log(`Calling js http with: \n ${JSON.stringify(options, null, 2)}`);

  //Extracting the headers from a vec back to a map
  const headers = {};
  for (const [k, v] of options.headers) {
    headers[k] = v;
  }

  //Sending the sync HTTP request
  //NOTE: this blocks the main (only) JS thread and is not ideal
  var res = request(options.method, options.uri, {
    headers,
    body: Buffer.from(options.body),
  });

  return { status: res.statusCode, body: res.getBody() };
}
export default makeHttpRequest;
