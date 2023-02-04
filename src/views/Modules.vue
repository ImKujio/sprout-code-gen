<template>
  <div class="flex-col-fill">
    <div v-if="!editPage" class="flex-col-fill">
      <content-title>
        <el-button type="primary" :icon="Plus" text style="margin-left: 4px;font-weight: bold" @click="onAdd">新增
        </el-button>
        <el-button text style="margin-left: 4px" @click="onEdit" :disabled="!selMod">编辑
        </el-button>
        <el-button text style="margin-left: 4px" @click="onEditProp" :disabled="!selMod">编辑属性
        </el-button>
        <el-button type="danger" text style="margin-left: 4px" @click="onDel" :disabled="!selMod">删除
        </el-button>
      </content-title>

      <size-wrapper v-model:height="height">
        <el-table
            :max-height="height"
            :data="modules"
            highlight-current-row
            @current-change="m => selMod = m"
        >
          <el-table-column type="index" align="center" label="序号" width="50"/>
          <el-table-column prop="name" label="类名"/>
          <el-table-column prop="comment" label="注释"/>
          <el-table-column prop="update" label="更新时间" width="150"/>
        </el-table>
      </size-wrapper>

      <el-dialog v-model="dialog" title="添加模块" width="320">
        <el-form ref="formRef" :model="form">
          <el-form-item prop="name" label="类名" :rules="[
          {required: true, message: '请输入类名', trigger: 'blur'},
          {pattern: /^[A-Z][a-z]{2,20}$/, message: '格式错误,大驼峰格式,例:SysUser'}
        ]">
            <el-input v-model="form.name" clearable placeholder="请输入模块名称"/>
          </el-form-item>
          <el-form-item prop="comment" label="注释" :rules="[
          {required: true, message: '请输入注释', trigger: 'blur'},
        ]">
            <el-input v-model="form.comment" clearable placeholder="请输入注释"/>
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
    <mod-editor v-else :mod-id="selMod.id" @edited="onPropEdit" @back="editPage = false"/>
  </div>

</template>

<script setup>
import {reactive, ref, watch} from "vue";
import SizeWrapper from "../components/SizeWrapper.vue";
import ContentTitle from "../components/ContentTitle.vue";
import ModEditor from "./ModEditor.vue"
import {Plus} from "@element-plus/icons-vue";
import {Data} from "../api/data.js";

const height = ref(0)

const modules = reactive([])
const dialog = ref(false)
const form = reactive({})
const formRef = ref(null)
const selMod = ref(null)
const editPage = ref(false)

function initForm(val) {
  form.id = !val ? null : val.id
  form.name = !val ? null : val.name
  form.comment = !val ? null : val.comment
  form.update = !val ? null : val.update
}

Data.Modules.get().then(res => {
  if (res != null) modules.push(...res)
  watch(modules, async n => {
    await Data.Modules.set(modules)
  })
})

async function onSubmit() {
  await formRef.value.validate(valid => {
    if (!valid) return
    const now = new Date()
    form.update = now.format("yy-MM-dd hh:mm:ss")
    if (form.id) {
      const index = modules.findIndex(m => m.id = form.id)
      modules.splice(index, 1, Object.assign({}, form))
    } else {
      form.id = now.getTime()
      modules.push(Object.assign({}, form))
    }
    dialog.value = false
  })
}

function onAdd() {
  initForm()
  dialog.value = true
}

function onEdit() {
  initForm(selMod.value)
  dialog.value = true
}

function onEditProp() {
  editPage.value = true
}

function onPropEdit(id) {
  modules.find(m => m.id === id).update = new Date().format("yy-MM-dd hh:mm:ss")
}

function onDel() {
  modules.splice(modules.indexOf(selMod), 1)
}
</script>

<style lang="scss" scoped>
</style>