<template>
    <div class="card text-center p-8 m-8 outline outline-offset-2 outline-green-200 bg-white-200 bg-gray-100">
        
        <div>
            <ul>{{ doctor.employee_id }}</ul>
            <ul>{{ doctor.name }}</ul>
            <ul>{{ doctor.position }}</ul>
            <ul>{{ doctor.ssn }}</ul>
            <br>
        </div>

        <div class="flex flex-row justify-between">
            <NuxtLink :to="`/staff/doctors/${doctor.employee_id}`"> 
                <button class="btn my-4 bg-blue-200 hover:bg-blue-100">View Details</button>
            </NuxtLink>
        </div>


    </div>
</template>

<script setup>
    const { doctor } = defineProps(['doctor'])
</script>