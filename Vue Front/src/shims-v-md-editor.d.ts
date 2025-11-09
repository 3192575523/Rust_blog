// 为 v-md-editor 相关的 JS 包提供临时类型声明
declare module '@kangc/v-md-editor' {
  const VMdEditor: any
  export default VMdEditor
}
declare module '@kangc/v-md-editor/lib/preview' {
  const VMdPreview: any
  export default VMdPreview
}
declare module '@kangc/v-md-editor/lib/theme/github.js' {
  const githubTheme: any
  export default githubTheme
}
declare module '@kangc/v-md-editor/lib/plugins/katex/index' {
  const createKatexPlugin: any
  export default createKatexPlugin
}
declare module '@kangc/v-md-editor/lib/plugins/mathjax/index' {
  const createMathjaxPlugin: any
  export default createMathjaxPlugin
}
declare module '@kangc/v-md-editor/lib/plugins/katex/cdn' {
  const createKatexPlugin: any
  export default createKatexPlugin
}