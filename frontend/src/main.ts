import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

//import App from './App.vue'
import App_lrar from './App_lrar.vue'
import router from './router'

const app = createApp(App_lrar)

app.use(createPinia())
app.use(router)

app.mount('#app')
