import * as https from "https";
import * as request from "sync-request";

export function makeHttpRequest(options) {
  console.log(`Calling js http with: \n ${JSON.stringify(options)}`);
  // Don't actually know how to make a synchronous HTTP GET in node.
  //   let callComplete = false;
  //   const chunks = [];
  //   let result;
  const headers = {};
  for (const [k, v] of options.headers) {
    // console.log(`HEADER: ${k}: ${v}`);
    headers[k] = v;
  }

  //   const nodeOptions = {
  //     method: options.method,
  //     headers,
  //   };
  //   const req = https.request(options.uri, nodeOptions, (res) => {
  //     console.log("CALLBACK");
  //     res.on("data", () => {
  //       let chunk;
  //       while (null !== (chunk = res.read())) {
  //         console.log("CHUNK:", chunk);
  //         chunks.push(chunk);
  //       }
  //     });

  //     res.on("end", () => {
  //       if (!res.complete) {
  //         console.error(
  //           "The connection was terminated while the message was still being sent"
  //         );
  //       } else {
  //         result = {
  //           status: res.statusCode,
  //           body: chunks.join(""),
  //         };
  //       }
  //       callComplete = true;
  //     });
  //   });
  //   req.write(options.body);
  //   req.end();

  //   while (callComplete === false) {
  //     // console.log("waiting");
  //   }
  //   //   return `Fake response for ${url} from javascript function makeHttpRequest`;
  //   console.log(JSON.stringify(result));

  //   return result;

  var res = request(options.method, options.uri, {
    headers,
    body,
  });
  console.log(res.getBody());
}
export default makeHttpRequest;
