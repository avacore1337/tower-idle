<!DOCTYPE html>
<template>
  <MyModal name="automation" title="Automation">
    <map v-if="false" :datas="data" />
    <div class="floor_listing_container">
      <button
        v-for="(floor, index) in seen_floors"
        :key="floor.name"
        class="floor_chooser"
        @click="chosen_floor_index = index"
      >
        {{ index + 1 }}
      </button>
    </div>
    <AutomationsBox :categories="categories" />
  </MyModal>
</template>

<script setup lang="ts">
import { ref } from "vue"

import AutomationsBox from "@c/modals/AutomationsBox.vue"
import Map from "@c/modals/Map.vue"
import MyModal from "./MyModal.vue"

import { computed } from "vue"
import { useStore } from "@store"
let store = useStore()
let wasm = computed(() => store.state.wasm)
let world = computed(() => store.state.world)

let has_seen = (a) => a.has_seen
let seen_floors = computed(() => world.value.floors.filter(has_seen))

let chosen_floor_index = ref(store.getters.current_floor.floor_index)
let chosen_floor = computed(() => store.state.world.floors[chosen_floor_index.value])

var data = computed(() => [wasm.value.get_map_for_floor(store.getters.current_floor.name)])

let categories: any = computed(() => [
  {
    name: "Collection",
    actions: chosen_floor.value.collections.filter(has_seen),
    toggle: wasm.value.toggle_priority_collection,
  },
  {
    name: "Crafting",
    actions: chosen_floor.value.craftings.filter(has_seen),
    toggle: wasm.value.toggle_priority_crafting,
  },
  {
    name: "Exploration",
    actions: chosen_floor.value.explorations.filter(has_seen),
    toggle: wasm.value.toggle_priority_exploration,
  },
])

/* watch(is_open, log) */
/* watch(chosen_floor_index, log) */

/* function log(a, b) { */
/*   console.log() */
/*   console.log(a, b) */
/*   console.log(chosen_floor_index) */
/*   console.log(chosen_floor) */
/* } */
</script>

<style scoped>
.floor_listing_container {
  display: flex;
  flex-direction: row;
  justify-content: center;
}
.floor_chooser {
  font-size: 1.25rem;
  width: 2rem;
  height: 2rem;
  /* flex-grow: 1; */
}
.automation_columns {
  display: flex;
  flex-direction: row;
  gap: 2rem;
}
.automation_column {
  min-width: 16rem;
  display: flex;
  flex-direction: column;
}
</style>
