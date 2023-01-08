<template>
  <div class="i-section">
    <div class="around-flex">
      <my-icon :icon="icons['Collection']" class="i-title"> Collection </my-icon>
    </div>
    <hr />
    <div class="between-flex">
      <div class="i-skill i-header">Skill</div>
      <div class="i-name i-header">Collect</div>
      <div class="i-actions i-header">Action</div>
    </div>
    <div v-for="collection in collections" :key="collection.name" class="i-box">
      <Action
category="Collection"
        :item="collection"
        :do_action="do_action"
        :toggle_priority="toggle_priority"
        :schedule_action="schedule_action"
      />
    </div>
  </div>
</template>
<script setup lang="ts">
/* <h1>Hello – {{ count }}</h1> */
/* <button @click.prevent="increment">Increment Count</button> */
import { computed } from "vue"
import { useStore } from "@store"
import Action from "@c/Action.vue"

let store = useStore()
let icons = computed(() => store.state.world.icons)
let collections = computed(() =>
  store.getters.current_floor.collections.filter((collection) => {
    if (collection.max_completions && collection.round_completions >= collection.max_completions) {
      return false
    }
    return collection.is_visible
  })
)

let wasm = computed(() => store.state.wasm)
function do_action(name: string, amount: number) {
  wasm.value.prepend_collection(name, amount)
}
function toggle_priority(name: string) {
  wasm.value.toggle_priority("Collection",name)
}
function schedule_action(name: string, amount: number) {
  wasm.value.append_collection(name, amount)
}
</script>

<style scoped></style>
