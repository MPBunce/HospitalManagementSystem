<template>
    <div class="text-black-400 grid md:grid-cols-3 grid-cols-1">
      
      <div>
        <h1>Department Heads</h1>

          {{department_list}}

      </div>
      
      <div>
        <h1>Procedures</h1>
          {{procedures_list}}
      </div>

      <div>
        <h1>Rooms</h1>
          {{room_list}}
      </div>

    </div>
</template>

<script setup>
  import axios from 'axios';

  const department_list = ref([]);

  onMounted( async () => {
      try{
          const department_list_response = await axios.get(`http://localhost:1080/api/departments_list`);
          department_list.value = department_list_response.data;
          console.log(department_list.value)
      }catch (error) {
          procedures.value = ["No associated department_list"];
          console.log(error);
      }
  });

  const procedures_list = ref([]);

  onMounted( async () => {
      try{
          const procedures_list_response = await axios.get(`http://localhost:1080/api/procedures_list`);
          procedures_list.value = procedures_list_response.data;
          console.log(procedures_list.value)
      }catch (error) {
          procedures.value = ["No associated department_list"];
          console.log(error);
      }
  });

  const room_list = ref([]);

  onMounted( async () => {
      try{
          const room_list_response = await axios.get(`http://localhost:1080/api/room_list`);
          room_list.value = room_list_response.data;
          console.log(room_list.value)
      }catch (error) {
          procedures.value = ["No associated department_list"];
          console.log(error);
      }
  });

</script>