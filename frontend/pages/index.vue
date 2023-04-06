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
  console.log("herereerere")
  const doctors = await useFetch("http://localhost:3000/get_docs")
  console.log(doctors)
  console.log("XDDDDDDDDDDDDDDDDd")
</script>