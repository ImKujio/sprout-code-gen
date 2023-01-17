<template>
  <div class="table">
    <el-table :data="defCols">
      <el-table-column align="center" label="序号" width="60">
        <template #default="scope">
          <span>{{ scope.$index + 1 }}</span>
        </template>
      </el-table-column>
      <el-table-column prop="name" label="字段名" min-width="120">
        <template #default="scope">
          <el-input v-model="scope.row.name"/>
        </template>
      </el-table-column>
      <el-table-column prop="type" label="字段类型" min-width="120">
        <template #default="scope">
          <el-input v-model="scope.row.type"/>
        </template>
      </el-table-column>
      <el-table-column prop="comment" label="字段注释" min-width="120">
        <template #default="scope">
          <el-input v-model="scope.row.comment"/>
        </template>
      </el-table-column>
      <el-table-column prop="position" label="生成位置" min-width="80">
        <template #default="scope">
          <el-select v-model="scope.row.position">
            <el-option key="1" label="之前" value="1"/>
            <el-option key="2" label="之后" value="2"/>
          </el-select>
        </template>
      </el-table-column>
      <el-table-column align="center" prop="primary" label="主键" width="60">
        <template #default="scope">
          <el-checkbox :disabled="hasPrimary ? !scope.row.primary : false " v-model="scope.row.primary"
                       @change="onPrimaryTap(scope.row.primary)"/>
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
import {Option} from "../../../api/data.js";
import {onMounted, reactive, ref, watch} from "vue";

const hasPrimary = ref(false)
const defCols = reactive([])

onMounted(async () => {
  defCols.push(...await Option.DefColumn.get())
  hasPrimary.value = defCols.find(c => c.primary) != null
  watch(defCols,async (n) => await Option.DefColumn.set(n))
})

function onAddTap(index) {
  defCols.splice(index, 0, {
    name: '',
    type: '',
    comment: '',
    primary: false,
    position: '1'
  })
}

function onDelTap(index) {
  defCols.splice(index, 1)
}

function onPrimaryTap(val) {
  hasPrimary.value = val
}
</script>
