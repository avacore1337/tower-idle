<!DOCTYPE html>
<template>
  <Popper
    class="my-popper"
    placement="top"
    :arrow="true"
    :interactive="false"
    hover
    open-delay="200"
  >
    {{ item.display_name }}
    <template #content>
      <div class="ap_container">
        {{ item.description }}
        <br />
        <br />
        <div class="description">
          <div v-if="item.max_completions" class="between-flex">
            <span class="data_key">round completions:</span>
            <span class="data">{{ item.round_completions }} / {{ item.max_completions }}</span>
          </div>
          <div v-if="item.dps" class="between-flex">
            <span class="data_key"> Mana lost per second: </span>
            <span class="data"> {{ item.dps }} </span>
          </div>
          <div v-if="item.materials">
            Crafting materials needed:
            <div v-for="material in item.materials" :key="material.item" class="between-flex">
              <span class="data_key">&nbsp;&nbsp;&nbsp;&nbsp;{{ material.item }}:</span>
              <span class="data">
                {{ material.amount * item.segments_paid }} /
                {{ material.amount * item.segments_needed }}
              </span>
            </div>
          </div>
          <div class="between-flex">
            <span class="data_key"> completion count: </span>
            <span class="data"> {{ item.completion_count }} / {{ item.automate_limit }} </span>
          </div>
          <div class="between-flex">
            <span class="data_key"> Time spent: </span>
            <span class="data"> <FormatNumber :value="item.ticks_spent / 30" /> sec </span>
          </div>
          <div class="between-flex">
            <span class="data_key"> Time to completion: </span>
            <span class="data">
              <FormatNumber :value="(item.ticks_to_complete - item.ticks_spent) / 30" /> sec
            </span>
          </div>
        </div>
      </div>
    </template>
  </Popper>
</template>

<script setup lang="ts">
import Popper from "vue3-popper"
import FormatNumber from "@c/util/FormatNumber.vue"

defineProps({
  item: {
    type: Object,
    required: true,
  },
})
</script>

<style scoped>
.ap_container {
  width: 25rem;
}
.data {
}
.data_key {
}
.description {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  /* width: 300px; */
  /* justify-content: center; */
  /* align-items: center; */
  /* flex-shrink: 0; */
  /* padding: 1rem 0 0; */
}
</style>
