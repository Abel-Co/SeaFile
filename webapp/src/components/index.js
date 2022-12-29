import { decamelize } from '../utils/decamelize'

const components = import.meta.globEager('./**/*.vue')

Object.assign(components)

export default {
  install(app) {
    Object.keys(components).forEach(path => {
      const name = decamelize(path.split(/[./]/)[2])
      app.component(`comp-${name}`, components[path].default)
    })
  }
}
