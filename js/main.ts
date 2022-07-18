import "../css/styles.css"

import { library } from "@fortawesome/fontawesome-svg-core"
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome"
import MyIcon from "@c/util/MyIcon.vue"
import MyPopper from "@c/MyPopper.vue"
import { fas } from "@fortawesome/pro-solid-svg-icons"
import { Wasm } from "./state"
import vSelect from "vue-select"
import "vue-select/dist/vue-select.css"
import VueFinalModal from "vue-final-modal"

library.add(fas)

import { createApp } from "vue"
import { createMyStore, store_key } from "./store"

import App from "@c/App.vue"

import("@p/index.js")
  .then(function (wasm: Wasm) {
    const store = createMyStore(wasm)
    createApp(App, {})
      .use(store, store_key)
      .use(VueFinalModal)
      .component("font-awesome-icon", FontAwesomeIcon)
      .component("my-icon", MyIcon)
      .component("MyPopper", MyPopper)
      .component("v-select", vSelect)
      .mount("#app")
  })
  .catch(console.error)
