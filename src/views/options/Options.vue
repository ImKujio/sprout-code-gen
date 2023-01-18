<template>
  <div class="content">
    <el-tabs v-model="tab">
      <el-tab-pane label="默认字段" name="1">
      </el-tab-pane>
      <el-tab-pane label="数据类型" name="2">
      </el-tab-pane>
    </el-tabs>
    <div class="content">
      <def-columns v-if="tab=== '1'"/>
      <data-type v-if="tab === '2'"/>
    </div>
  </div>
</template>

<script setup>
import DefColumns from "./DefColumns.vue";
import DataType from "./DataType.vue";
import {ref} from "vue";
const tab = ref('1')
</script>

<style lang="scss" scoped>

</style>