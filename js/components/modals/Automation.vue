<!DOCTYPE html>
<template>
  <MyModal name="automation" title="Automation">
    <MyMap v-if="false" :datas="data" />
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
import MyMap from "@c/modals/MyMap.vue"
import MyModal from "./MyModal.vue"
import { Category } from "@state"

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

let categories = computed<Category[]>(() => [
  {
    name: "Collection",
    actions: chosen_floor.value.collections.filter(has_seen),
  },
  {
    name: "Crafting",
    actions: chosen_floor.value.craftings.filter(has_seen),
  },
  {
    name: "Exploration",
    actions: chosen_floor.value.explorations.filter(has_seen),
  },
])

let favourites = computed<Category[]>(() => [
  {
    name: "Collection",
    actions: store.getters.all_collections.filter(is_favourite),
  },
  {
    name: "Crafting",
    actions: store.getters.all_craftings.filter(is_favourite),
  },
  {
    name: "Exploration",
    actions: store.getters.all_explorations.filter(is_favourite),
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
