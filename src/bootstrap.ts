import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './Main.vue'

const app = createApp(App)

app.use(createPinia())

app.mount('#window')
import { invoke, window, } from '@tauri-apps/api'

invoke('greet', { name: 'World' })
  // `invoke` 返回的是一个 Promise
  .then((response) => console.log(response))

// window.getCurrent().setAlwaysOnTop(true)
// window.getCurrent().setResizable(false)
const webview = new window.WebviewWindow('theUniqueLabel', {
  url: 'https://',
  resizable: false,
  focus: true,
  alwaysOnTop: true,
  skipTaskbar: true,
  decorations: false,
  width: 400,
  height: 300,
  x: (await window.getCurrent().innerPosition()).x,
  y: (await window.getCurrent().innerPosition()).y
},);

