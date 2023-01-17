<template>
  <div class="content">
    <Codemirror
        v-model:value="code"
        :options="cmOptions"
        border
        placeholder="test placeholder"
    />
  </div>
</template>

<script setup>
import {onMounted, ref, watch} from "vue";
import {Template} from "../../../api/data.js";

import Codemirror from "codemirror-editor-vue3";
// language
import "codemirror/mode/vue/vue";
import "codemirror/mode/javascript/javascript"
import "codemirror/mode/sql/sql"
import "codemirror/mode/clike/clike"
// theme
import "codemirror/theme/idea.css";
// 自动补全
import "codemirror/addon/hint/show-hint.css";
import "codemirror/addon/hint/show-hint";
import "codemirror/addon/hint/anyword-hint";
// 自动括号匹配
import "codemirror/addon/edit/matchbrackets";
// 代码折叠
import "codemirror/addon/fold/foldgutter.css";
import "codemirror/addon/fold/foldcode.js";
import "codemirror/addon/fold/brace-fold.js";
import "codemirror/addon/fold/comment-fold.js";
import "codemirror/addon/fold/indent-fold.js";
import "codemirror/addon/fold/foldgutter.js";
// 光标行背景高亮
import "codemirror/addon/selection/active-line.js";

const code = ref("")
const cmOptions = {
  mode: "text/x-vue", // Language mode
  theme: "idea", // Theme
  lineNumbers: true, // Show line number
  smartIndent: true, // Smart indent
  indentUnit: 2, // The smart indent unit is 2 spaces in length
  foldGutter: true, // Code folding
  lineWrapping: false,
  gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter", "CodeMirror-lint-markers"],
  styleActiveLine: true, // Display the style of the selected row
  matchBrackets: true,// 括号匹配
  extraKeys: {"Ctrl-Enter": "autocomplete"}//ctrl-space唤起智能提示
}

watch(code, async (n) => {
  await Template.Spring.set(n)
})

onMounted(async () => {
  code.value = await Template.Spring.get()
})

</script>

<style lang="scss" scoped>
.content {
  Codemirror {
    height: 100%;
    //width: 100%;
  }
}

</style>