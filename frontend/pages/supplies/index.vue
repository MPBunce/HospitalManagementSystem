<template>

  <div class="text-black-400 flex justify-center space-x-96">
    <h1>
      Medications 
    </h1>

    <NuxtLink :to="`/supplies/new`"> 
      <button class="bg-green-500">Add</button>
    </NuxtLink>

  </div>
    <br>
  <div class="text-black-400 flex justify-center">

    <table class="table-auto">
      <tr class="bg-gray-400">
        <th class="mx-4 px-4 py-4">Code</th>
        <th class="mx-4 px-4 py-4">Name</th>
        <th class="mx-4 px-4 py-4">Brand</th>
        <th class="mx-4 px-4 py-4">Description</th>
        <th class="mx-4 px-4 py-4">In Stock</th>
        <th class="mx-4 px-4 py-4">Edit</th>
        <th class="mx-4 px-4 py-4">Delete</th>
      </tr>
      <tr v-for="med in medication" class="bg-gray-100">
        <th class="mx-4 px-4 py-4">{{ med.code }}</th>
        <th class="mx-4 px-4 py-4">{{ med.name }}</th>
        <th class="mx-4 px-4 py-4">{{ med.brand}}</th>
        <th class="mx-4 px-4 py-4">{{med.description}}</th>
        <th class="mx-4 px-4 py-4">{{med.in_stock}}</th>
        <th class="mx-4 px-4 py-4"><button class="bg-yellow-200">Edit</button></th>
        <th class="mx-4 px-4 py-4"><button class="bg-red-200">Delete</button></th>
      </tr>
    </table>

  </div>

</template>
  
<script setup>
  import axios from 'axios';
  const medication = ref([]);


  onMounted( async () => {
      try{
          const medication_response = await axios.get(`http://localhost:1080/api/get_medication`);
          medication.value = medication_response.data;
          console.log(medication.value)
      }catch (error) {
          procedures.value = ["No medication"];
          console.log(error);
      }
  });

</script>