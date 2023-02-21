import { createRouter, createWebHistory } from "vue-router"
import MainView from "../views/MainView.vue"
import PeoplesView from "../views/PeoplesView.vue"
import AboutView from "../views/AboutView.vue"
const routes = [
    {
        path: '/main',
        name: 'main',
        component: MainView
    },
    {
        path: '/peoples',
        name: 'peoples',
        component: PeoplesView
    },
    {
        path: '/about',
        name: 'about',
        component: AboutView
    }
]
const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router