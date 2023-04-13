<template>
    <div class="card text-center">
        
        <div>
            <li>{{ doctor.employee_id }}</li>
            <li>{{ doctor.name }}</li>
            <li>{{ doctor.position }}</li>
            <li>{{ doctor.ssn }}</li>
            <br>
        </div>

        <!-- <NuxtLink :to="`/products/${product.id}`"> 
            <p class="btn my-4">View Details</p>
        </NuxtLink> -->

    </div>
</template>

<script setup>
    const { doctor } = defineProps(['doctor'])
</script>