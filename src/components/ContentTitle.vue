<template>
  <div ref="dragging" data-tauri-drag-region style="width:100%;padding-top: 8px;padding-bottom: 8px;padding-right: 140px">
    <div >
      <slot></slot>
    </div>
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
  if (dragging.value == null) return
  dragging.value.removeEventListener("mousedown", startDragging);
})

const noDragSelector = "input, a, button";
async function startDragging(e) {
  if (e.target.closest(noDragSelector)) return;
  await appWindow.startDragging();
}
</script>

<style lang="scss" scoped>

</style>