<template>
  <div ref="dragging" data-tauri-drag-region style="width:100%;padding-top: 12px;padding-bottom: 8px">
    <slot></slot>
  </div>
</template>

<script setup>

import {onMounted, onUnmounted, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";

const dragging = ref(null)

onMounted(() => {
  dragging.value.addEventListener("mousedown", startDragging);
})

onUnmounted(() => {
  dragging.value.removeEventListener("mousedown", startDragging);
})

async function startDragging(e) {
  console.log(e.target)
  if (!e.target.closest("div")) return;
  await appWindow.startDragging();
}
</script>

<style lang="scss" scoped>

</style>