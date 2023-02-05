import { createRouter, createWebHistory } from "vue-router"
import Main from "../views/Main.vue"
import Peoples from "../views/Peoples.vue"
const routes = [
    {
        path: '/main',
        name: 'main',
        component: Main
    },
    {
        path: '/peoples',
        name: 'peoples',
        component: Peoples
    }
]
const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router