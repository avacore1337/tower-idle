<!DOCTYPE html>
<template>
  <Popper
    class="my-popper"
    placement="top"
    hover
    :arrow="true"
    :interactive="false"
    open-delay="200"
  >
    <slot />
    <template #content>
      <div class="skill-p-box">
        <div class="between-flex">
          <div class="skill-p-header skill-p-name"></div>
          <div class="skill-p-header skill-p-level">Level</div>
          <div class="skill-p-header skill-p-xp">Xp</div>
        </div>
        <div class="between-flex">
          <div class="skill-p-name">Level</div>
          <div class="skill-p-level">{{ Math.trunc(skill.level) }}</div>
          <div class="skill-p-xp">
            <FormatNumber :value="skill.level_current_xp" /> /
            <FormatNumber :value="skill.level_xp_required" />
          </div>
        </div>
        <div class="between-flex">
          <div class="skill-p-name">Talent</div>
          <div class="skill-p-level">
            {{ Math.trunc(skill.talent) }}
            <span v-if="skill.talent != skill.starting_talent">
              (+{{ skill.talent - skill.starting_talent }})
            </span>
          </div>
          <div class="skill-p-xp">
            <FormatNumber :value="skill.talent_current_xp" /> /
            <FormatNumber :value="skill.talent_xp_required" />
          </div>
        </div>
        <div class="between-flex">
          <div class="skill-p-name">Total</div>
          <div>Xp rate: <FormatNumber :value="skill.total_multiplier" /></div>
        </div>
      </div>
    </template>
  </Popper>
</template>

<script setup lang="ts">
import Popper from "vue3-popper"
import FormatNumber from "@c/util/FormatNumber.vue"

defineProps({
  skill: {
    type: Object,
    required: true,
  },
})
</script>

<style scoped>
.skill-p-box {
  padding: 0.1rem 0.1rem 0.1rem 0.1rem;
  flex-grow: 1;
  flex-basis: 0;
  display: flex;
  min-width: 16rem;
  flex-direction: column;
  gap: 1px;
}
.skill-p-name {
  min-width: 8rem;
  text-align: left;
}
.skill-p-xp {
  min-width: 5rem;
  text-align: left;
}
.skill-p-level {
  min-width: 5rem;
  text-align: left;
}
.skill-p-title {
  text-align: center;
}
.skill-p-header {
  font-weight: bold;
  text-align: left;
}
</style>
