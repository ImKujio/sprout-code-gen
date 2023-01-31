<template>
  <size-wrapper v-model:height="height">
    <el-table :max-height="height" :data="types">
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
      <el-table-column label="操作 " align="center" width="80">
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
      <template #empty>
        <el-button type="text" style="font-weight: bold" @click="onAddTap(0)">
          <el-icon>
            <Plus/>
          </el-icon>
          <span>添加一行</span></el-button>
      </template>
    </el-table>
  </size-wrapper>
</template>

<script setup>
import {Minus, Plus} from "@element-plus/icons-vue";
import {onMounted, reactive, ref, watch} from "vue";
import {Option} from "../../api/data.js";
import SizeWrapper from "../../components/SizeWrapper.vue";

const height = ref(0)
const types = reactive([])
const newType = {
  name: '',
  fType: '',
  sType: '',
}


onMounted(async () => {
  const d = await Option.DataType.get()
  if (d != null) types.push(...d)
  watch(types, async (n) => await Option.DataType.set(n))
})

function onAddTap(index) {
  types.splice(index, 0, Object.assign({},newType))
}

function onDelTap(index) {
  types.splice(index, 1)
}

</script>

<style scoped>

</style>