<template>
  <div class="flex-col-fill">
    <content-title>
      <el-button type="primary" text style="font-weight: bold" @click="onAdd">新增
      </el-button>
    </content-title>

    <size-wrapper v-model:height="height">
      <el-table :max-height="height" :data="modules"  >
        <el-table-column type="index" align="center" label="序号" width="60"/>
        <el-table-column prop="name" label="名称"/>
        <el-table-column prop="entity" label="类名"/>
        <el-table-column prop="update" label="更新时间" width="200"/>
        <el-table-column align="center" label="操作" width="100">
          <template #default="scope">
            <el-button class="act-btn" size="small" circle :icon="Edit" @click="onEdit(scope.row)">
            </el-button>
            <el-button class="act-btn" size="small" circle :icon="Minus" @click="onDel(scope.$index)">
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </size-wrapper>

    <el-dialog v-model="dialog" title="添加模块" width="360">
      <el-form ref="formRef" :model="form">
        <el-form-item prop="name" label="名称" :rules="[
          {required: true, message: '请输入模块名称', trigger: 'blur'}
        ]">
          <el-input v-model="form.name" clearable placeholder="请输入模块名称"/>
        </el-form-item>
        <el-form-item prop="entity" label="类名" :rules="[
          {required: true, message: '请输入类名称', trigger: 'blur'},
          {pattern: /^[A-Z][a-z]{2,20}$/, message: '格式错误,例:SysUser'}
        ]">
          <el-input v-model="form.entity" clearable placeholder="请输入类名称"/>
        </el-form-item>
<!--        <el-form-item prop="type" label="类型" :rules="[-->
<!--          {required: true, message: '请输入类名称', trigger: 'blur'},-->
<!--        ]">-->
<!--          <el-select style="width: 100%" v-model="form.type" clearable placeholder="请输入类名称">-->
<!--            <el-option v-for="type in types"-->
<!--                       :value="type.name"-->
<!--                       :label="type.name"/>-->
<!--          </el-select>-->
<!--        </el-form-item>-->
      </el-form>
      <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialog = false">取消</el-button>
        <el-button type="primary" @click="onSubmit">
          确定
        </el-button>
      </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import {reactive, ref, watch} from "vue";
import SizeWrapper from "../components/SizeWrapper.vue";
import ContentTitle from "../components/ContentTitle.vue";
import {Minus, Edit} from "@element-plus/icons-vue";
import {Data} from "../api/data.js";

const height = ref(0)

const modules = reactive([])
const types = ref(null)
const dialog = ref(false)
const form = reactive({})
const formRef = ref(null)

function initForm() {
  form.id = null
  form.name = null
  form.entity = null
  form.update = null
}

Data.Modules.get().then(res => {
  if (res != null) modules.push(...res)
  watch(modules, async n => {
    await Data.Modules.set(modules)
  })
})

Data.PropTypes.get().then(res => {
  types.value = res
})

async function onSubmit() {
  await formRef.value.validate(valid => {
    if (!valid) return
    const now = new Date()
    form.id = now.getTime()
    form.update = now.format("yy-MM-dd hh:mm:ss")
    modules.push(Object.assign({}, form))
    dialog.value = false
  })
}

function onAdd() {
  initForm()
  dialog.value = true
}

function onEdit(row) {

}

function onDel(index) {
  modules.splice(index, 1)
}
</script>

<style lang="scss" scoped>

</style>