<template>
  <div class="flex-col-fill">
    <content-title>
      <el-button type="primary" :icon="Plus" text style="margin-left: 4px;font-weight: bold" @click="onAdd">新增
      </el-button>
      <el-button text style="margin-left: 4px" @click="onEdit" :disabled="!selCol">编辑
      </el-button>
      <el-button type="danger" text style="margin-left: 4px" @click="onDel" :disabled="!selCol">删除
      </el-button>
    </content-title>

    <size-wrapper v-model:height="height">
      <el-table
          :max-height="height"
          :data="columns"
          highlight-current-row
          @current-change="m => selCol = m"
      >
        <el-table-column type="index" align="center" label="序号" width="50"/>
        <el-table-column prop="name" label="列名"/>
        <el-table-column prop="comment" label="注释"/>
        <el-table-column prop="type" label="表单组件"/>
        <el-table-column prop="defVal" label="默认值"/>
        <el-table-column prop="options" label="选项"/>
        <el-table-column prop="update" label="更新时间" width="150"/>
      </el-table>
    </size-wrapper>

    <el-dialog v-model="dialog" title="添加类信息列" width="400">
      <el-form ref="formRef" :model="form" label-width="auto">
        <el-form-item prop="name" label="列名" :rules="[
          {required: true, message: '请输入列名', trigger: 'blur'},
          {pattern: /^[A-Za-z]{2,20}$/, message: '格式错误,只能是英文'}
        ]">
          <el-input v-model="form.name" clearable placeholder="请输入列名"/>
        </el-form-item>
        <el-form-item prop="comment" label="注释" :rules="[
          {required: true, message: '请输入注释', trigger: 'blur'}
        ]">
          <el-input v-model="form.comment" clearable placeholder="请输入注释"/>
        </el-form-item>
        <el-form-item prop="type" label="表单组件">
          <el-radio-group v-model="form.type">
            <el-radio label="输入框">输入框</el-radio>
            <el-radio label="开关">开关</el-radio>
            <el-radio label="选择器">选择器</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item v-if="form.type === '选择器'" :rows="2" prop="options" label="选项">
          <el-input type="textarea" v-model="form.options" clearable placeholder="请输入选项" @input="onChange"/>
        </el-form-item>
        <el-form-item v-if="form.type !== '输入框'" prop="defVal" label="默认值" :rules="[
            {required: true, message: '请选择默认值', trigger: 'blur'}
        ]">
          <el-select style="width: 100%" v-model="form.defVal" placeholder="请选择默认值" :key="defValKey">
            <el-option v-if="form.type ==='开关'" label="true" value="true" />
            <el-option v-if="form.type ==='开关'" label="false" value="false" />
            <el-option
                v-if="form.type === '选择器'"
                v-for="(item,index) in defValOpts"
                :key="index"
                :value="item"
                :label="item"/>
          </el-select>
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
import {Plus} from "@element-plus/icons-vue";
import {Store} from "../api/store.js";

const height = ref(0)

const columns = reactive([])
const dialog = ref(false)
const form = reactive({})
const formRef = ref(null)
const selCol = ref(null)
const defValKey = ref(0)
const defValOpts = ref([])

function initForm(val) {
  form.id = !val ? null : val.id
  form.name = !val ? null : val.name
  form.comment = !val ? null : val.comment
  form.type = !val ? "输入框" : val.type
  form.defVal = !val ? null : val.defVal
  form.options = !val ? null : val.options
  form.update = !val ? null : val.update
  defValOpts.value = !val ? [] : !val.options ? [] : val.options.split(",")
}

Store.ClassColumns.list().then(res => {
  if (res != null) columns.push(...res)
}).finally(() => {
  watch(columns, async n => {
    await Store.ClassColumns.setList(n)
  })
})

async function onSubmit() {
  await formRef.value.validate(valid => {
    if (!valid) return
    const now = new Date()
    form.update = now.format("yy-MM-dd hh:mm:ss")
    if (form.id) {
      const index = columns.findIndex(c => c.id === form.id)
      columns.splice(index, 1, Object.assign({}, form))
    } else {
      form.id = now.getTime()
      columns.push(Object.assign({}, form))
    }
    dialog.value = false
  })
}

function onChange(val) {
  if (val == null) {
    defValOpts.value = []
    return
  }
  const nVal = val.replace(/[\s\t\n]/g, "")
  if (nVal === "") {
    defValOpts.value = []
    return
  }
  defValOpts.value = nVal.split(",")
  if (defValOpts.value.indexOf(form.defVal) === -1)
    form.defVal = null
  defValKey.value++
  if (nVal !== form.options)
    form.options = nVal
}

function onAdd() {
  initForm()
  dialog.value = true
}

function onEdit() {
  initForm(selCol.value)
  dialog.value = true
}

function onDel() {
  columns.splice(columns.indexOf(selCol), 1)
}
</script>

<style lang="scss" scoped>
</style>