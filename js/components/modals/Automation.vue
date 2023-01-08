<!DOCTYPE html>
<template>
  <MyModal name="automation" title="Automation">
    <map v-if="false" :datas="data" />
    <div class="floor_listing_container">
      <button class="section_chooser" @click="tab = 0">Floors</button>
      <button class="section_chooser" @click="tab = 1">Favourites</button>
    </div>
    <hr />
    <div v-if="tab == 0">
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
    </div>
    <div v-if="tab == 1">
      <AutomationsBox :categories="favourites" />
    </div>
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
let is_favourite = (a) => a.favourite
let seen_floors = computed(() => world.value.floors.filter(has_seen))

let tab = ref(0)
let chosen_floor_index = ref(store.getters.current_floor.floor_index)
let chosen_floor = computed(() => store.state.world.floors[chosen_floor_index.value])

var data = computed(() => [wasm.value.get_map_for_floor(store.getters.current_floor.name)])

let categories: any = computed(() => [
  {
    name: "Collection",
    actions: chosen_floor.value.collections.filter(has_seen),
    toggle: wasm.value.toggle_priority_collection,
    favourite_toggle: wasm.value.toggle_favourite_collection,
  },
  {
    name: "Crafting",
    actions: chosen_floor.value.craftings.filter(has_seen),
    toggle: wasm.value.toggle_priority_crafting,
    favourite_toggle: wasm.value.toggle_favourite_crafting,
  },
  {
    name: "Exploration",
    actions: chosen_floor.value.explorations.filter(has_seen),
    toggle: wasm.value.toggle_priority_exploration,
    favourite_toggle: wasm.value.toggle_favourite_exploration,
  },
])

let favourites: any = computed(() => [
  {
    name: "Collection",
    actions: chosen_floor.value.collections.filter(has_seen).filter(is_favourite),
    toggle: wasm.value.toggle_priority_collection,
    favourite_toggle: wasm.value.toggle_favourite_collection,
  },
  {
    name: "Crafting",
    actions: chosen_floor.value.craftings.filter(has_seen).filter(is_favourite),
    toggle: wasm.value.toggle_priority_crafting,
    favourite_toggle: wasm.value.toggle_favourite_crafting,
  },
  {
    name: "Exploration",
    actions: chosen_floor.value.explorations.filter(has_seen).filter(is_favourite),
    toggle: wasm.value.toggle_priority_exploration,
    favourite_toggle: wasm.value.toggle_favourite_exploration,
  },
])
</script>

<style scoped>
.section_chooser {
  font-size: 1.25rem;
  width: 10rem;
  height: 2rem;
  /* flex-grow: 1; */
}
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
