<template>
  <div class="flex-col-fill">
    <content-title>
      <el-row :gutter="10">
        <el-col :span="1.5">
          <el-button
              type="primary"
              plain
              size="small"
              :icon="Plus"
              @click="onAdd"
          >新增
          </el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button
              type="success"
              plain
              size="small"
              :icon="Edit"
          >修改
          </el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button
              type="danger"
              plain
              size="small"
              :icon="Delete"
          >删除
          </el-button>
        </el-col>
      </el-row>
    </content-title>

    <size-wrapper v-model:height="height">
      <el-table :max-height="height"  :data="modules" highlight-current-row>
        <el-table-column align="center" label="序号" width="60">
          <template #default="scope">
            <span>{{ scope.$index + 1 }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="name" label="名称"/>
        <el-table-column prop="table" label="表名"/>
        <el-table-column prop="entity" label="类名"/>
        <el-table-column prop="update" label="更新时间" width="200"/>
      </el-table>
    </size-wrapper>
    <el-dialog v-model="dialog" :title="title">
      弹窗
    </el-dialog>
  </div>
</template>

<script setup>
import {reactive, ref} from "vue";
import SizeWrapper from "../components/SizeWrapper.vue";
import {Plus,Edit,Delete} from "@element-plus/icons-vue";
import ContentTitle from "../components/ContentTitle.vue";
const height = ref(0)

const modules = reactive([
  {
    name: "商品",
    table: "goods",
    entity: "Goods",
    update: "2023-21-02 11:11:24"
  }
])
const dialog = ref(false)
const title = ref("")

function onAdd() {
  console.log("dialog is show")
  dialog.value = true
}
</script>

<style lang="scss" scoped>

</style>