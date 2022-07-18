import { ComponentCustomProperties } from "vue"
import { Store } from "vuex"
import { State } from "@state"
import * as wasm_api from "@p/index"
type Wasm = typeof wasm_api

declare module "@vue/runtime-core" {
  interface ComponentCustomProperties {
    $store: Store<State>
    $wasm: Wasm
  }
}
