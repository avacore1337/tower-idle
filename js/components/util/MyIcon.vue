<template>
  <span class="vertical-align">
    <font-awesome-icon :icon="'fa-solid fa-' + icon.name" style="margin-right: 0.2rem" />
    <slot v-if="text === ''" />
    <span v-if="text !== ''" style="white-space: nowrap"> {{ text }} </span>
  </span>
</template>

<script setup lang="ts">
import { PropType } from "vue"
import { Icon } from "@p/index"

defineProps({
  icon: {
    type: Object as PropType<Icon>,
    required: true,
  },
  text: {
    type: String,
    default: "",
  },
})
</script>

<style scoped>
/* .fa-icon { */
/*   fill: currentColor; */
/* } */
</style>
