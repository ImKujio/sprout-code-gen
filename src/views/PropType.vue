<template>
  <div class="flex-col-fill">
    <content-title>
      <el-button type="primary" text style="font-weight: bold" @click="onAdd">新增
      </el-button>
      <el-button v-show="selected" type="danger" text style="margin-left: 4px" @click="onDel">删除
      </el-button>
    </content-title>

    <size-wrapper v-model:height="height">
      <el-table :max-height="height" :data="types" highlight-current-row @current-change="onSelect">
        <el-table-column type="index" align="center" label="序号" width="60"/>
        <el-table-column prop="name" label="名称"/>
        <el-table-column prop="update" label="更新时间" width="200"/>
      </el-table>
    </size-wrapper>

    <el-dialog v-model="dialog" title="添加模块" width="360">
      <el-form ref="formRef" :model="form">
        <el-form-item prop="name" label="名称" :rules="[
            {required: true, message: '请输入模块名称', trigger: 'blur'}
        ]">
          <el-input v-model="form.name" clearable/>
        </el-form-item>
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
import {Data} from "../api/data.js";

const height = ref(0)

const types = reactive([])
const dialog = ref(false)
const selected = ref(null)
const form = reactive({})
const formRef = ref(null)

function initForm(){
  form.name = null
  form.update = null
}

Data.PropTypes.get().then(res => {
  if (res != null) types.push(...res)
  watch(types, async n => {
    await Data.PropTypes.set(types)
  })
})

async function onSubmit() {
  await formRef.value.validate(valid => {
    if (!valid) return
    form.update = new Date().format("yy-MM-dd hh:mm:ss")
    types.push(Object.assign({},form))
    dialog.value = false
  })
}

function onSelect(val) {
  selected.value = val
}

function onAdd() {
  initForm()
  dialog.value = true
}

function onDel() {
  types.splice(types.indexOf(selected.value), 1)
}
</script>

<style lang="scss" scoped>

</style>