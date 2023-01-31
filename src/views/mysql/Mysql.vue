<template>
  <CodeEditor v-model="code" :mode="'text/x-vue'"/>
</template>

<script setup>

import CodeEditor from "../../components/CodeEditor.vue";
import {onMounted, ref, watch} from "vue";
import {Mysql} from "../../api/data.js";

const code = ref("")

onMounted(async () => {
  code.value = await Mysql.Create.get()
  watch(code, async n => {
    await Mysql.Create.set(n)
  })
})


</script>

<style scoped>

</style>