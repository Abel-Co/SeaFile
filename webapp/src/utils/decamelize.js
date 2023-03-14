import { isString } from 'lodash'

export const decamelize = (text) => {
  if (!isString(text)) {
    throw new TypeError('Expected a string')
  }
  const rCapitalLetter = /([A-Z])([a-z0-9]*)?/g
  const separator = '-'
  return text.replace(rCapitalLetter, (oText, r1 = '', r2 = '', index) => {
    return `${index === 0 ? '' : separator}${r1.toLowerCase()}${r2}`
  })
}
