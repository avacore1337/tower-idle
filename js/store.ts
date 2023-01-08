import { InjectionKey } from "vue"
import { createStore, useStore as baseUseStore, Store } from "vuex"
import { State, MFloor, Wasm, MCollection, MCrafting, MExploration } from "./state"
import { AllCollects, AllExplors, AllCrafts } from "@p/index"
import { merge, recurse_update } from "./util"

export const store_key: InjectionKey<Store<State>> = Symbol()

export function useStore() {
  return baseUseStore(store_key)
}

export const createMyStore = function (wasm: Wasm): Store<State> {
  const world = wasm.get_world()
  const state = wasm.get_state()
  const history = wasm.get_history()
  const user_settings = wasm.get_user_settings()
  merge(world, state)
  console.log("world: ", world)

  const action_queue = wasm.get_action_queue()
  console.log("action queue: ", action_queue)
  return createStore<State>({
    state: {
      wasm,
      world,
      history,
      user_settings,
      action_queue,
      numberFormat: "SCIENTIFIC",
      messages: [],
    },
    mutations: {
      update_dynamic_data(state: State): void {
        internal_update_dynamic_data(state)
      },
      update_history(state: State): void {
        state.history = wasm.get_history()
      },
      tick(state: State): void {
        if (state.user_settings.paused) {
          state.wasm.paused()
        } else {
          state.wasm.tick()
        }
        if (document.hidden) {
          return
        }
        internal_update_dynamic_data(state)
      },
    },
    getters: {
      all_collections: (state: State) => {
        return state.world.floors.map((floor) => floor.collections).flat()
      },
      all_craftings: (state: State) => {
        return state.world.floors.map((floor) => floor.craftings).flat()
      },
      all_explorations: (state: State) => {
        return state.world.floors.map((floor) => floor.explorations).flat()
      },
      collection:
        (state: State) =>
        (collection_type: AllCollects): MCollection | null => {
          for (const i in state.world.floors) {
            const floor = state.world.floors[i]
            for (const j in floor.collections) {
              const collection = floor.collections[j]
              if (JSON.stringify(collection.name) == JSON.stringify(collection_type)) {
                return collection
              }
            }
          }
          return null
        },
      crafting:
        (state: State) =>
        (crafting_type: AllCrafts): MCrafting | null => {
          for (const i in state.world.floors) {
            const floor = state.world.floors[i]
            for (const j in floor.craftings) {
              const crafting = floor.craftings[j]
              if (JSON.stringify(crafting.name) == JSON.stringify(crafting_type)) {
                return crafting
              }
            }
          }
          return null
        },
      exploration:
        (state: State) =>
        (exploration_type: AllExplors): MExploration | null => {
          for (const i in state.world.floors) {
            const floor = state.world.floors[i]
            for (const j in floor.explorations) {
              const exploration = floor.explorations[j]
              if (JSON.stringify(exploration.name) == JSON.stringify(exploration_type)) {
                return exploration
              }
            }
          }
          return null
        },
      current_floor(state: State): MFloor {
        for (const i in state.world.floors) {
          const floor = state.world.floors[i]
          if (floor.name == state.world.current_floor) {
            return floor
          }
        }
        // Needed by typescript but should never happen
        console.error("Current floor is broken somehow")
        return state.world.floors[0]
      },
    },
  })
}

function internal_update_dynamic_data(state: State): void {
  state.user_settings = state.wasm.get_user_settings()
  const new_queue = state.wasm.get_action_queue()
  const new_state = state.wasm.get_state()
  const messages = state.wasm.get_new_texts()
  if (messages.length != 0) {
    state.messages.push(...messages)
  }
  recurse_update(state.world, new_state)
  if (state.action_queue.length != new_queue.length) {
    state.action_queue = new_queue
  } else {
    recurse_update(state.action_queue, new_queue)
  }
}

//   return new Vuex.Store({
//     state: {
//       wasm: wasm,
//       numberFormat: 'DEFAULT',
//       meta: wasm.get_meta_data(),
//       tutorial_data: Object.freeze(tutorial_data),
//     },
//     mutations: {
//       toggleNumberFormat(state) {
//         state.numberFormat = {
//           DEFAULT: 'SCIENTIFIC',
//           SCIENTIFIC: 'DEFAULT',
//         }[state.numberFormat]
//       },
//     },
//     getters: {
//       getNextNumberFormat(state) {
//         return {
//           DEFAULT: 'Scientific notation',
//           SCIENTIFIC: 'Natural numbers',
//         }[state.numberFormat]
//       },
//     },
//   })
