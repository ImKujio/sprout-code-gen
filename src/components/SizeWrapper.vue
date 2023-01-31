<template>
  <div ref="wrapper" class="flex-col-fill">
    <slot></slot>
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