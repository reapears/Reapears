<!-- Harvests list page -->
<!-- <script setup>
import api_url from "../../api-handlers/index";
</script> -->

<template>


    <!-- Card layout (Album) -->
    <div class="container-fluid py-4">
        <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 row-cols-lg-4 g-3.2">
            <div v-for="harvest in harvests">
                <div class="col ">
                    <!-- Card-->
                    <router-link to="/harvests/" style="text-decoration: none;">
                        <div class="card h-100 border-0 py-0 mb-5 ">
                            <img src="http://localhost:3000/cultivars/p/tomatoes.jpg"
                                class="card-img-top card-img rounded-4  img-fluid" alt="...">
                            <div class="card-body px-0 mx-0 pt-1 pb-0">
                                <p class="card-title my-0">{{ harvest.name }}</p>
                                <p class="card-text py-0 my-0">{{ harvest.placeName }}, {{ harvest.region }}</p>
                                <p class="card-text py-0 my-0 text-muted">{{ harvest.farm }}</p>
                                <p class="card-text py-0 my-0 text-muted">{{ harvest.availableAt }}</p>
                                <p class="card-text py-0 my-0 ">N${{ harvest.price.amount }} NAD {{
                                    harvest.price.unit
                                }}
                                </p>
                            </div>
                        </div>
                    </router-link>
                </div>
            </div>
        </div>
    </div>

</template>

<script>

import { getHarvestList } from '../../api-handlers/harvest';

export default {

    data() {
        return {
            harvests: []
        }
    },

    methods: {
        async getHarvests() {
            this.harvests = await getHarvestList();
        }
    },

    created() {
        this.getHarvests()
    }
}


</script>


<style scoped>
/* .card-img{
    width: 100%!important; 
height: 15rem!important; 
object-fit: cover;
} 

*/
</style>