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

let formatedTime = computed<string>(() => {
  let num = Math.floor(props.value)
  if (num === undefined) {
    return ""
  }
  let sec = num % 60
  let min = Math.floor(num / 60) % 60
  let hour = Math.floor(num / 60 / 60)

  if (hour === 0) {
    return min.toString().padStart(2, "0") + ":" + sec.toString().padStart(2, "0")
  } else {
    return (
      hour.toString().padStart(2, "0") +
      ":" +
      min.toString().padStart(2, "0") +
      ":" +
      sec.toString().padStart(2, "0")
    )
  }
})
</script>
