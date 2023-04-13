<template>
    <div class="text-black-400">

        <h1>doctors:</h1>
        <div v-for="d in doctors">
            <DoctorCard :doctor="d"/>
        </div>
  

        <h1>nurses:</h1>
        <div v-for="n in nurses">
            <NurseCard :nurse="n"/>
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
