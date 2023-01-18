<template>
  <div ref="contentWrapper" class="flex-col-fill">
    <slot></slot>
  </div>
</template>

<script setup>
import {onMounted, onUnmounted, ref} from "vue";

const emit = defineEmits(['update:height'])
const contentWrapper = ref(null)

onMounted(() => {
  getHeight()
  window.addEventListener('resize', getHeight)
})

onUnmounted(() => {
  window.removeEventListener('resize', getHeight)
})

function getHeight() {
  console.log(contentWrapper.value)
  const h = contentWrapper.value.clientHeight
  console.log(h)
  emit('update:height', h)
}

</script>

<style scoped>

</style>