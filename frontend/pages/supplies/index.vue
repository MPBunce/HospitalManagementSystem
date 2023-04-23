<template>
    <div class="text-black-400">
  
      {{medication}}
  
  
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