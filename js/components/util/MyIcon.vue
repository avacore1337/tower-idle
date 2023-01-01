<template>
  <span>
    <font-awesome-icon :icon="iconName" style="margin-right: 0.2rem" />
    <slot v-if="text === ''" />
    <span v-if="text !== ''" style="white-space: nowrap"> {{ text }} </span>
  </span>
</template>

<script setup lang="ts">
import { PropType, computed } from "vue"
import { Icon } from "@p/index"
import { icon_name } from "@util"

// Should there be a vertial align for the icons?
let props = defineProps({
  icon: {
    type: Object as PropType<Icon>,
    required: true,
  },
  text: {
    type: String,
    default: "",
  },
})
let iconName = computed(() => icon_name(props.icon))
</script>

<style scoped>
/* .fa-icon { */
/*   fill: currentColor; */
/* } */
</style>
