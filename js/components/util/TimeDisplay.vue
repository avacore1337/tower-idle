<template>
  <span> {{ formatedTime }} </span>
</template>

<script setup lang="ts">
import { computed } from "vue"

const props = defineProps({
  value: {
    type: Number,
    required: true,
  },
})

let formatedTime = computed(() => {
  let num = Math.floor(props.value)
  if (num === undefined) {
    return null
  }
  let sec = num % 60
  let min = Math.floor(num / 60)

  return min.toString().padStart(2, "0") + ":" + sec.toString().padStart(2, "0")
})
</script>
