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
    <slot />
    <template #content>
      <div>
        This is the Popper content
        <br />
        {{ content }}
      </div>
    </template>
  </Popper>
</template>

<script setup lang="ts">
import Popper from "vue3-popper"
defineProps({
  content: { type: String, required: true },
})
</script>

<style></style>
