<!DOCTYPE html>
<template>
  <MyModal name="history" title="History">
    This is where I display history data

    <EChart :datas="datas" />
    <p v-for="i in 2" :key="i">
      Vue Final Modal is a renderless, stackable, detachable and lightweight modal component.
    </p>
  </MyModal>
</template>

<script setup lang="ts">
import { ref, watch } from "vue"
import EChart from "@c/modals/EChart.vue"
import { computed } from "vue"
import { useStore } from "@store"
import { SkillTypes } from "@p/index"
import MyModal from "./MyModal.vue"

let store = useStore()
/* let wasm = computed(() => store.state.wasm) */
let history = computed(() => store.state.history)
let paused = computed(() => store.state.user_settings.paused)

interface DataPoint {
  name: SkillTypes
  value: number
}

let is_open = ref(false)

let datas = computed<DataPoint[]>(() => {
  let data: DataPoint[] = []
  for (var skill of history.value.current_round.skills) {
    if (skill.talent_delta != 0) {
      data.push({ value: skill.talent_delta, name: skill.name })
    }
  }
  return data
})

watch(is_open, () => store.commit("update_history"))
watch(paused, () => store.commit("update_history"))
</script>

<style scoped></style>
