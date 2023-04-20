<template>
    <div>
        {{ input_employee_id }}
        
        {{departments_data}}

        {{procedures}}
    </div>
</template>

<script setup>
    import axios from 'axios';


    const { employee_id } = useRoute().params
    let input_employee_id = employee_id


    let doctor = await axios.get(`http://localhost:1080/api/get_physician/${input_employee_id}`)
    doctor = doctor.data

    let departments = await axios.get(`http://localhost:1080/api/get_departments/${input_employee_id}`)
    console.log(departments)
    let departments_data = departments.data


    const procedures = ref([]);

    onMounted(async () => {
        try {
            const response = await axios.get(`http://localhost:1080/api/get_procedures/${input_employee_id}`);
            procedures.value = response.data;
        } catch (error) {
            procedures.value = ["Doctor is useless"];
            console.log(error);
        }
    });

    // onMounted(async () => {
              
    //     try {
    //         procedures = await axios.get(`http://localhost:1080/api/get_procedures/${input_employee_id}`)
    //             .catch(function (error) {
    //                 procedures = "Doctor is useless"
    //             });
    //     } catch (e){
    //         console.log(e)
    //     }
    // })



</script>
