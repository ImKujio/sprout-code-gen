import {createApp} from "vue";
import ElementPlus from 'element-plus'
import { GlobalCmComponent } from "codemirror-editor-vue3";
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import "./style.css";
import App from "./App.vue";


const app = createApp(App)

app.use(ElementPlus)
app.use(GlobalCmComponent);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}

app.mount("#app");

