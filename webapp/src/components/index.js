import { decamelize } from '../utils/decamelize'

const components = import.meta.globEager('./**/*.vue')

Object.assign(components)

export default {
  install(app) {
    Object.keys(components).forEach(path => {
      const item = components[path].default
      app.component(`comp-${decamelize(item.name)}`, item)
    })
  }
}
