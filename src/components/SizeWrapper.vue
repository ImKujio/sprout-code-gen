<template>
  <div ref="wrapper" style="flex: 1;position: relative">
    <div style="position: absolute;width: 100%;height: 100%">
      <slot></slot>
    </div>
  </div>
</template>

<script setup>
import {onMounted, onUnmounted, ref} from "vue";

const emit = defineEmits(['update:width','update:height'])
const wrapper = ref(null)

onMounted(() => {
  getSize()
  window.addEventListener('resize', getSize)
})

onUnmounted(() => {
  window.removeEventListener('resize', getSize)
})

function getSize() {
  const height = wrapper.value.clientHeight
  const width = wrapper.value.clientWidth
  emit('update:height', height)
  emit('update:width',width)
}

</script>

<style scoped>

</style>