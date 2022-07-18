<template>
  <div class="skill-container">
    <div v-for="skill in skills" :key="skill.name" class="skill-box">
      <SkillPopper :skill="skill" class="skill-box2">
        <div class="text-box">
          <my-icon :icon="skill.icon" />
          {{ skill.name }} {{ skill.level }}
        </div>
        <MyProgressBar :value="skill.level_percent" />
        <MyProgressBar :value="skill.talent_percent" />
      </SkillPopper>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"
import MyProgressBar from "@c/util/MyProgressBar.vue"
import SkillPopper from "@c/SkillPopper.vue"

let store = useStore()
let skills = computed(() =>
  store.state.world.skills.filter((skill) => {
    return skill.is_visible
  })
)
</script>

<style scoped>
.skill-container {
  display: flex;
  flex-grow: 1;
  flex-basis: 0;
}
.skill-box {
  border: 1px solid white;
  flex-grow: 1;
  flex-basis: 0;
  min-width: 10rem;
}
.skill-box2 {
  flex-grow: 1;
  flex-basis: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
  margin-left: 0.2rem;
  margin-right: 0.2rem;
  margin-bottom: 1px;
}
.text-box {
  display: flex;
  justify-content: center;
}
</style>
