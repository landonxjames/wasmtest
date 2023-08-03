export function getSysTimeUnixMillis(url) {
  const val = Date.now();
  return BigInt(val);
}
export default getSysTimeUnixMillis;
