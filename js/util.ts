import { Icon } from "@p/index"

export function merge(o: any, o2: any) {
  for (const key in o2) {
    if (!(key in o)) {
      o[key] = o2[key]
      continue
    }

    if (Array.isArray(o2[key])) {
      if (o[key].length != o2[key].length) {
        /* console.log(key, o[key], o2[key]) */
        o[key] = o2[key]
        continue
      }
    }
    if (typeof o2[key] == "object") {
      merge(o[key], o2[key])
      continue
    }
    if (o[key] != o2[key]) {
      /* console.log(typeof o[key]); */
      /* console.log(key, o[key], o2[key]); */
      o[key] = o2[key]
    }
  }
}
export function recurse_update(o: any, o2: any) {
  for (const key in o2) {
    if (Array.isArray(o2[key])) {
      if (o[key].length != o2[key].length) {
        /* console.log(key, o[key], o2[key]) */
        o[key] = o2[key]
        continue
      }
    }
    if (typeof o2[key] == "object") {
      recurse_update(o[key], o2[key])
      continue
    }
    if (o[key] != o2[key]) {
      /* console.log(typeof o[key]); */
      /* console.log(key, o[key], o2[key]); */
      o[key] = o2[key]
    }
  }
}

export function icon_name(icon: Icon) {
  /* console.log(icon.prefix + " fa-" + icon.name) */
  return icon.prefix + " fa-" + icon.name
}
