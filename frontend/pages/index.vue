<template>
  <div class="text-black-400">

    <h1>doctors:</h1>

    <div v-for="d in doctors">
        <li>{{ d.employee_id }}</li>
        <li>{{ d.name }}</li>
        <li>{{ d.position }}</li>
        <li>{{ d.ssn }}</li>
        <br>
    </div>


  </div>
</template>

<script setup>
  import axios from 'axios';
  
  let doctors = await axios.get("http://localhost:1080/api/get_all_physicians")
  doctors = doctors.data


</script>