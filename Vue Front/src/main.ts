import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import App from './App.vue'

import VMdEditor from '@kangc/v-md-editor'
import '@kangc/v-md-editor/lib/style/base-editor.css'
import githubTheme from '@kangc/v-md-editor/lib/theme/github.js'
import '@kangc/v-md-editor/lib/theme/style/github.css'
import Prism from 'prismjs'

// 🔁 关键：用 cdn 版插件（不是 index / index.js）
import createKatexPlugin from '@kangc/v-md-editor/lib/plugins/katex/cdn'

import VMdPreview from '@kangc/v-md-editor/lib/preview'
import '@kangc/v-md-editor/lib/style/preview.css'

VMdEditor.use(githubTheme, { Prism })
VMdEditor.use(createKatexPlugin())

VMdPreview.use(githubTheme, { Prism })
VMdPreview.use(createKatexPlugin())

createApp(App).use(createPinia()).use(router).use(VMdEditor).use(VMdPreview).mount('#app')
