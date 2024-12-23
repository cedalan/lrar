import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

//import App from './App.vue'
import App_lrar from './App_lrar.vue'

const app = createApp(App_lrar)

app.use(createPinia())

app.mount('#app')
