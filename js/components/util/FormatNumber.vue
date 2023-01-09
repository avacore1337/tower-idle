<template>
  <span> {{ printableNumber }} </span>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"

let store = useStore()
const props = defineProps({
  value: {
    type: Number,
    required: true,
  },
})
// Inspiration:
// https://github.com/FreddecGames/ngsc/blob/master/frontend/src/components/FormatNumber.vue
let printableNumber = computed<string>(() => {
  let num = props.value
  if (num === undefined) {
    return "0"
  }

  /* if (num < 0.01) { */
  /*   return num.toFixed(3) */
  /* } */
  if (num < 0.1) {
    return num.toFixed(2)
  }
  if (num < 100) {
    return num.toFixed(1)
  }
  if (num < 10000) {
    return Math.floor(num).toString()
  }

  if (store.state.numberFormat === "SCIENTIFIC") {
    let exponent = 1
    while (num >= 10) {
      num /= 10
      exponent++
    }

    return `${num.toFixed(1)}e${exponent}`
  }

  const ending = ["K", "M", "B", "T", "Qa", "Qi", "He", "Se", "Oc", "No", "De"]
  let index = -1
  while (num >= 10000 && index < ending.length - 1) {
    num /= 1000
    index++
  }

  return `${num.toPrecision(4)}${ending[index]}`
})
</script>
