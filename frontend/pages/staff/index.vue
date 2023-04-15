<template>
    <div class="text-black-400">

        <div class="shadow-2xl">
            <div class="flex flex-row justify-between">

                <h1 class="text-xl">Doctors</h1>

                <NuxtLink :to="``"> 
                    <button class="btn my-4 bg-green-200 hover:bg-green-100">Add New Doctor</button>
                </NuxtLink>

            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3">
                <div v-for="d in doctors" class="">
                    <div >
                        <DoctorCard :doctor="d"/>                    
                    </div>

                </div>
            </div>
        </div>


        <div class="shadow-2xl mt-20">
            <div class="flex flex-row justify-between">

                <h1 class="text-xl">Nurses</h1>

                <NuxtLink :to="``"> 
                    <button class="btn my-4 bg-green-200 hover:bg-green-100">Add New Nurse</button>
                </NuxtLink>

            </div>


            <div class="grid grid-cols-1 lg:grid-cols-3">
                <div v-for="n in nurses" class="">
                    <div>
                        <NurseCard :nurse="n"/>
                    </div>

                </div>
            </div>

        </div>


  
    </div>
</template>
  
<script setup>
    import axios from 'axios';
  
    let doctors = await axios.get("http://localhost:1080/api/get_all_physicians")
    let nurses = await axios.get("http://localhost:1080/api/get_nurses")
    console.log(doctors)
    console.log(nurses)
  
    doctors = doctors.data
    nurses = nurses.data

</script>
