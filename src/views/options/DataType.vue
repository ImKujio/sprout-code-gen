<template>
  <div class="table">
    <el-table :data="dataTypes">
      <el-table-column align="center" label="序号" width="60">
        <template #default="scope">
          <span>{{ scope.$index + 1 }}</span>
        </template>
      </el-table-column>
      <el-table-column prop="name" label="名称" min-width="120">
        <template #default="scope">
          <el-input v-model="scope.row.name"/>
        </template>
      </el-table-column>
      <el-table-column prop="fType" label="字段类型" min-width="120">
        <template #default="scope">
          <el-input v-model="scope.row.fType"/>
        </template>
      </el-table-column>
      <el-table-column prop="sType" label="组件类型" min-width="120">
        <template #default="scope">
          <el-input v-model="scope.row.sType"/>
        </template>
      </el-table-column>
      <el-table-column align="center" width="80">
        <template #header>
          <el-button type="text" style="font-weight: bold" @click="onAddTap(0)">
            <el-icon>
              <Plus/>
            </el-icon>
            <span>添加</span></el-button>
        </template>
        <template #default="scope">
          <el-button size="small" circle @click="onAddTap(scope.$index + 1)">
            <el-icon>
              <Plus/>
            </el-icon>
          </el-button>
          <el-button size="small" circle @click="onDelTap(scope.$index)">
            <el-icon>
              <Minus/>
            </el-icon>
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup>
import {Minus, Plus} from "@element-plus/icons-vue";
import {Option} from "../../api/data.js";
import {onMounted, reactive, watch} from "vue";

const dataTypes = reactive([])

onMounted(async () => {
  const d = await Option.DataType.get()
  if (d != null) dataTypes.push(...d)
  watch(dataTypes, async (n) => await Option.DataType.set(n))
})

function onAddTap(index) {
  dataTypes.splice(index, 0, {
    name: '',
    fType: '',
    sType: '',
  })
}

function onDelTap(index) {
  dataTypes.splice(index, 1)
}
</script>
