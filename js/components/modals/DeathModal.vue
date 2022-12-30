<!DOCTYPE html>
<template>
  <vue-final-modal
    v-model="is_open"
    classes="modal-container"
    content-class="modal-content"
    name="death"
    :click-to-close="false"
  >
    <span class="modal__title">Death!</span>
    <div class="death_content">
      <div class="death_skill_wrapper">
        <span class="death_skill_name"> Skill Name </span>
        <span class="death_skill_value"> Current</span>
      </div>
      <div class="death_skill_wrapper" v-for="skill in relevant_skills" :key="skill.name">
        <span class="death_skill_name"> {{ skill.name }} </span>
        <span class="death_skill_value"> {{ skill.talent }} (+{{ skill.talent_delta }}) </span>
      </div>
      <AutomationsBox :categories="categories" />
    </div>
    <button class="death_action" @click="wasm.do_rebirth">Rebirth</button>
  </vue-final-modal>
</template>

<script setup lang="ts">
import { ref, watch } from "vue"
import { $vfm } from "vue-final-modal"
import type { Icon } from "@p/index"
import { computed } from "vue"
import { useStore } from "@store"
import AutomationsBox from "@c/modals/AutomationsBox.vue"

let store = useStore()
let wasm = computed(() => store.state.wasm)
let history = computed(() => store.state.history)
let icons = computed(() => store.state.world.icons)
let floors = computed(() => store.state.world.floors)
let is_dead = computed(() => store.state.world.status.is_dead)
let is_open = ref(false)

function get_priority_icon(item: any): Icon {
  return icons.value["Priority" + item.priority.toString()]
}

let relevant_skills = computed(() =>
  history.value.current_round.skills.filter((skill) => skill.is_visible)
)

watch(is_dead, () => {
  store.commit("update_history")
  is_open.value = store.state.world.status.is_dead
})

let categories: any = computed(() => [
  {
    name: "Collection",
    actions: floors.value
      .map((floor) => floor.collections)
      .flat()
      .filter(
        (collection) =>
          collection.has_seen && (!collection.is_automatable || collection.is_newly_automatable)
      ),
    toggle: wasm.value.toggle_priority_collection,
  },
  {
    name: "Crafting",
    actions: floors.value
      .map((floor) => floor.craftings)
      .flat()
      .filter(
        (crafting) =>
          crafting.has_seen && (!crafting.is_automatable || crafting.is_newly_automatable)
      ),
    toggle: wasm.value.toggle_priority_crafting,
  },
  {
    name: "Exploration",
    actions: floors.value
      .map((floor) => floor.explorations)
      .flat()
      .filter(
        (exploration) =>
          exploration.has_seen && (!exploration.is_automatable || exploration.is_newly_automatable)
      ),
    toggle: wasm.value.toggle_priority_exploration,
  },
])
</script>

<style scoped>
.death_skill_wrapper {
  display: flex;
  flex-direction: row;
}
.death_skill_name {
  min-width: 8rem;
}
.death_skill_value {
  min-width: 6rem;
}
.death_action {
}
.death_content {
  flex-grow: 1;
  overflow-y: auto;
  display: flex;
  gap: 3px;
  flex-direction: column;
  justify-content: center;
}
</style>
