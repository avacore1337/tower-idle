<template>
  <div class="skill-container">
    <div v-for="skill in skills" :key="skill.name" class="skill-box">
      <SkillPopper :skill="skill" class="skill-box2">
        <div class="between-flex">
          <div class="skill-name">
            <my-icon :icon="skill.icon">
              {{ skill.name }}
            </my-icon>
          </div>
          <span>{{ skill.level }}</span> -
          <span style="margin-right: 0.4rem"> {{ skill.talent }} </span>
        </div>
        <MyProgressBar class="skill-progress" :value="skill.level_percent" />
        <MyProgressBar
          class="skill-progress"
          :value="skill.talent_percent"
          color="progress-other"
        />
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
  flex-direction: column;
}
.skill-name {
  min-width: 9rem;
  padding-left: 0.4rem;
}
.skill-box {
  min-width: 14rem;
  /* border: 1px solid white; */
}
.skill-box2 {
  flex-grow: 1;
  flex-basis: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
  margin: 0.5rem;
}
.skill-progress {
  margin-bottom: 0.1rem;
}
/* .skill-text { */
/* padding-left: 0.2rem; */
/* padding-right: 0.2rem; */
/* } */
.text-box {
  display: flex;
  justify-content: center;
}
</style>
