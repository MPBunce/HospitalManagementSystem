<template>
    <div class="card text-center p-8 m-8 outline outline-offset-2 outline-green-200 bg-white-200 bg-gray-100">
        
        <div>
            <ul>{{ nurse.employee_id }}</ul>
            <ul>{{ nurse.name }}</ul>
            <ul>{{ nurse.position }}</ul>
            <ul>{{ nurse.registered }}</ul>
            <ul>{{ nurse.ssn }}</ul>
            <br>
        </div>

        <div class="flex flex-row justify-between">
            <NuxtLink :to="`/staff/nurses/${nurse.employee_id}`"> 
                <button class="btn my-4 bg-blue-200 hover:bg-blue-100">View Details</button>
            </NuxtLink>

            <NuxtLink :to="`/staff/nurses/${nurse.employee_id}`"> 
                <button class="btn my-4 bg-yellow-200 hover:bg-yellow-100">Edit Details</button>
            </NuxtLink>
            <NuxtLink :to="`/staff/nurses/${nurse.employee_id}`"> 
                <button class="btn my-4 bg-red-200 hover:bg-red-100">Delete Doctor</button>
            </NuxtLink>
        </div>

    </div>
</template>

<script setup>
    const { nurse } = defineProps(['nurse'])
</script>