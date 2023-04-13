<template>
    <div class="card text-center">
        
        <div>
            <li>{{ nurse.employee_id }}</li>
            <li>{{ nurse.name }}</li>
            <li>{{ nurse.position }}</li>
            <li>{{ nurse.registered }}</li>
            <li>{{ nurse.ssn }}</li>
            <br>
        </div>

        <!-- <NuxtLink :to="`/products/${product.id}`"> 
            <p class="btn my-4">View Details</p>
        </NuxtLink> -->

    </div>
</template>

<script setup>
    const { nurse } = defineProps(['nurse'])
</script>