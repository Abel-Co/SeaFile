import JSONBigInt from "json-bigint"

export const jsonBigInt = JSONBigInt({ useNativeBigInt: true })

export const stringify = obj => jsonBigInt.stringify(obj)
export const parse = str => jsonBigInt.parse(str, null)

export const objects = {
  purify: obj => parse(stringify(obj, null, null), null)
}
