<template>
    <div>
      <form @submit.prevent="submitForm" class="flex flex-col space-y-12">
        <div class="space-x-48">
          <label>Code</label>
          <input v-model="formData.code" type="number" placeholder="#" />
        </div>
  
        <div class="space-x-48">
          <label>Name</label>
          <input v-model="formData.name" placeholder="Oxy" />
        </div>
  
        <div class="space-x-48">
          <label>Brand</label>
          <input v-model="formData.brand" placeholder="Phizer" />
        </div>
  
        <div class="space-x-48">
          <label>Description</label>
          <input v-model="formData.description" placeholder="What does the drug do?" />
        </div>
  
        <div class="space-x-48">
          <label>In Stock</label>
          <input v-model="formData.in_stock" placeholder="true" />
        </div>
      </form>
  
      <button class="mt-8 bg-green-200" type="submit" @click="submitForm">
        SUBMIT
      </button>
    </div>
  </template>
  
<script setup>

  
    const formData = reactive({
      code: "",
      name: "",
      brand: "",
      description: "N/A",
      in_stock: true,
    });
  
    

    const submitForm = () => {
        console.log("here");
        console.log(formData);
        formData.description = ""

    };
</script>
