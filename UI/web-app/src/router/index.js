import {
	createRouter,
	createWebHistory,
	createWebHashHistory,
} from "vue-router";
import Harvest from "../views/harvest/Harvest.vue";
import HarvestList from "../views/harvest/HarvestList.vue";

import SignUp from "../views/account/SignUp.vue";
import Login from "../views/account/Login.vue";

const routes = [
	// Harvest-routes
	{
		path: "/harvest/:id",
		name: "Harvest",
		component: Harvest,
	},
	{
		path: "/",
		name: "Harvests",
		component: HarvestList,
	},

	// Account-routes
	{
		path: "/signup",
		name: "SignUp",
		component: SignUp,
	},

	{
		path: "/login",
		name: "Login",
		component: Login,
	},

	//   {
	//     path: '/about',
	//     name: 'About',
	//     // route level code-splitting
	//     // this generates a separate chunk (about.[hash].js) for this route
	//     // which is lazy-loaded when the route is visited.
	//     component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
	//   }
];

const router = createRouter({
	history: createWebHashHistory(),
	// history: createWebHistory(process.env.BASE_URL),
	routes,
});

export default router;
