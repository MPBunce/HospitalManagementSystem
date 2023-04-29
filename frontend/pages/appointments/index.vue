<template>
    <div class="text-black-400 grid md:grid-cols-2 grid-cols-1">
      
      <div>
        <h1>Appointments</h1>

        appointment card
        {{appointments}}

      </div>
      
      <div>
        <h1>Overnight Procedures</h1>

        surgery card
        {{surgeries}}
      </div>


    </div>
</template>
  
<script setup>
  import axios from 'axios';
  
  const appointments = ref([]);

  onMounted( async () => {
      try{
          const appointments_response = await axios.get(`http://localhost:1080/api/get_appointments`);
          appointments.value = appointments_response.data;
          console.log(appointments.value)
      }catch (error) {
          procedures.value = ["No associated appointments"];
          console.log(error);
      }
  });
  
  const surgeries = ref([]);

  onMounted( async () => {
      try{
          const surgeries_response = await axios.get(`http://localhost:1080/api/get_surgeries`);
          surgeries.value = surgeries_response.data;
          console.log(surgeries.value)
      }catch (error) {
          procedures.value = ["No associated surgeries"];
          console.log(error);
      }
  });
  
</script>