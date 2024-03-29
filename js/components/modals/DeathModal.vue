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
      <div class="row-flex">
        <span class="death_skill_name"> Mana gained: </span>

        <FormatNumber class="death_skill_value" :value="mana_gained" />
      </div>
      <hr />
      <div class="death_skill_wrapper">
        <span class="death_skill_name"> Skill Name </span>
        <span class="death_skill_value"> Current</span>
      </div>
      <div v-for="skill in relevant_skills" :key="skill.name" class="death_skill_wrapper">
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
import { computed } from "vue"
import { useStore } from "@store"
import AutomationsBox from "@c/modals/AutomationsBox.vue"
import FormatNumber from "@c/util/FormatNumber.vue"
import { Category } from "@state"
import { SkillHistory } from "@p/index"

let store = useStore()
let wasm = computed(() => store.state.wasm)
let history = computed(() => store.state.history)
let is_dead = computed(() => store.state.world.status.is_dead)
let is_open = ref(false)
let mana_gained = computed(() => history.value.current_round.mana_gained)

let relevant_skills = computed<SkillHistory[]>(() =>
  history.value.current_round.skills.filter((skill) => skill.is_visible)
)

watch(is_dead, () => {
  store.commit("update_history")
  is_open.value = store.state.world.status.is_dead
})

let not_or_newly_automateable = (crafting) =>
  crafting.has_seen && (!crafting.is_automatable || crafting.is_newly_automatable)

let categories = computed<Category[]>(() => [
  {
    name: "Collection",
    actions: store.getters.all_collections.filter(not_or_newly_automateable),
  },
  {
    name: "Crafting",
    actions: store.getters.all_craftings.filter(not_or_newly_automateable),
  },
  {
    name: "Exploration",
    actions: store.getters.all_explorations.filter(not_or_newly_automateable),
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
